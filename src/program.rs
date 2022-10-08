use gl::types::*;


use crate::buffer::*;
use crate::uniform::*;
use crate::shader::*;
use crate::texture::*;
use crate::attributes::*;
use crate::error::*;
use crate::log::*;

use std::cell::Cell;
use std::collections::HashSet;
use std::ffi::CString;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;


// -- Program


#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct ProgramId(pub GLuint);


pub struct ProgramBuilder {
	vert_shader       : Shader<Vertex>,
	frag_shader       : Shader<Fragment>,
	maybe_geom_shader : Option<Shader<Geometry>>,
	maybe_texture     : Option<(String, Texture)>,
	attributes        : Vec<String>
}

impl ProgramBuilder {
	pub fn new(
		vert_shader       : Shader<Vertex>,
		frag_shader       : Shader<Fragment>,
		) -> Self {
		Self {
			maybe_geom_shader : None,
			maybe_texture     : None,
			attributes        : Vec::new(),
			vert_shader, frag_shader,
		}
	}

	pub fn texture(mut self, name : &str, texture : Texture) -> Self {
		self.maybe_texture = Some((name.to_string(), texture));
		self
	}

	pub fn geom_shader(mut self, geom_shader : Shader<Geometry>) -> Self {
		self.maybe_geom_shader = Some(geom_shader);
		self
	}



	pub fn attributes(mut self, attributes : &[&str]) -> Self {
		for attribute in attributes {
			self.attributes.push(attribute.to_string());
		}
		self
	}


	pub fn build(self) -> Result<Program, GLError> {
		Program::new(self)
	}

}

#[derive(Debug, Clone, Copy)]
pub struct AttributePos(pub gl::types::GLuint);

#[derive(Debug)]
pub struct Program {
	pub id: ProgramId,
	has_geometry: bool,
	maybe_texture: Option<Texture>,
	indices : Option<IndexBuffer>,
	vao: VAOId,
	n_elems: Cell<Option<usize>>,
	attributes_loc : Rc<HashMap<String, AttributePos>>,
}

impl Program {

	pub fn new(
		builder : ProgramBuilder
		) -> Result<Self, GLError> 
	{
		let ProgramBuilder {
			vert_shader,
			frag_shader,
			maybe_geom_shader,
			maybe_texture,
			attributes,
		} = builder;
		let program_id = unsafe {gl::CreateProgram()};

		// -- Attach shaders
		unsafe {
			gl::AttachShader(program_id, vert_shader.id);
			gl::AttachShader(program_id, frag_shader.id);
			maybe_geom_shader.as_ref().map_or((), |geom_shader|
				gl::AttachShader(program_id, geom_shader.id)
				);
			gl::LinkProgram(program_id);
		}		

		// -- Link and check link
		unsafe {gl::LinkProgram(program_id)};
		let log_result = get_log(program_id, LogKind::LinkLog);

		if let Some(error_msg) = log_result {
			return Err(GLError::LinkProgram(error_msg));
		}


		// -- Detach shaders
		unsafe {
			gl::DetachShader(program_id, vert_shader.id);
			gl::DetachShader(program_id, frag_shader.id);
			maybe_geom_shader.as_ref().map_or((), |geom_shader|
				gl::DetachShader(program_id, geom_shader.id)
				);
		}

		// -- Find attribute location
		let mut attributes_loc = HashMap::new();
		for attribute in attributes.into_iter() {
			let uniform_name_c : CString = CString::new(attribute.clone()).unwrap();
			let pos = unsafe {
				gl::GetAttribLocation(program_id, uniform_name_c.as_ptr().cast())
			};

			// let pos = unsafe {pos.assume_init()};
			if pos == -1 {
				return Err(GLError::InexistentOrUndeclaredAttribute(attribute.to_string()));
			}
			else {
				attributes_loc.insert(attribute, AttributePos(pos as gl::types::GLuint));
			}
		}


		// -- Generate default vao
		let mut vao_id = 0;
		unsafe {gl::GenVertexArrays(1, &mut vao_id);}
		if vao_id == 0 {
			return Err(GLError::CouldNotCreateVAO);
		}
		let vao_id = VAOId(vao_id);


		let mut to_return = Self {
			id :           ProgramId(program_id),
			has_geometry : maybe_geom_shader.is_some(),
			vao :  vao_id,
			attributes_loc : Rc::new(attributes_loc),
			maybe_texture : None,
			n_elems: Cell::new(None),
			indices: None,
		};

		to_return.maybe_texture = if let Some((name, texture)) = maybe_texture {
			to_return.uniform::<gl::types::GLint>(&name)?.pass(&0);
			Some(texture)
		}
		else { None };

		Ok(to_return)
	}


	pub fn duplicate(&self) -> Result<Self, GLError> {
		let mut vao_id = 0;
		unsafe {gl::GenVertexArrays(1, &mut vao_id);}
		if vao_id == 0 {
			return Err(GLError::CouldNotCreateVAO);
		}
		let vao_id = VAOId(vao_id);



		return Ok(Self {
			id: self.id,
			has_geometry: self.has_geometry,
			maybe_texture: self.maybe_texture.clone(),
			vao: vao_id,
			attributes_loc: self.attributes_loc.clone(),
			n_elems: Cell::new(None),
    		indices: None,
		});
	}


	pub fn bind(&self, attribute : &str, buffer_view : BufferView) -> Result<(), GLError> {
		let attribute = attribute.to_string();
		if let Some(pos) = self.attributes_loc.get(&attribute) {
			unsafe {gl::BindVertexArray(self.vao.0);}

			let new_val = Some(buffer_view.n_elems);
			buffer_view.bind_to(*pos);

			unsafe {gl::EnableVertexAttribArray(pos.0); }
			unsafe {gl::BindVertexArray(0);}

			if self.n_elems.get() < new_val {
				self.n_elems.set(new_val)
			}
			Ok(())
		}
		else {
			Err(GLError::InexistentOrUndeclaredAttribute(attribute.to_string()))
		}
	}	


	pub fn set_indices(&mut self, indices: IndexBuffer) {
		self.indices = Some(indices);
	}


	pub fn uniform<'a, T : UniformData + ?Sized>(&'a self, uniform_name : &str) -> Result<Uniform<'a, T>, GLError> {

		// shouldn't panic if uniform name contains no null bytes
		let uniform_name_c : CString = CString::new(uniform_name).unwrap();
		let location = unsafe {
			gl::GetUniformLocation(self.id.0, uniform_name_c.as_ptr() as *mut gl::types::GLchar)
		};


		if location == -1 {
			Err(GLError::InexistentUniform(uniform_name.to_string()))
		}
		else {
			Ok(Uniform::<'a, T> {
				program_id : &self.id,
				location   : LayoutLocation(location),
				phantom    : std::marker::PhantomData,
			})
		}
	}


	pub fn set_current(&self) {
		unsafe {gl::UseProgram(self.id.0)}
	}

	pub fn bind_uniform<D>(&self, name : &str, uniform_buffer : &UniformBuffer<D>) -> Result<(), GLError>
	{
		let uniform_name_c : CString = CString::new(name.to_string()).unwrap();

		let uniform_index = unsafe {
			gl::GetUniformBlockIndex(self.id.0, uniform_name_c.as_ptr().cast())
		};

		if uniform_index == gl::INVALID_INDEX {
			return Err(GLError::InexistentUniformBuffer(name.to_string()));
		}

		unsafe {
			gl::UniformBlockBinding(self.id.0, uniform_index, uniform_buffer.binding_point.0);
		}

		Ok(())
	}

	fn bind_texture(&self) {
		if let Some(texture) = &self.maybe_texture {
			unsafe {gl::ActiveTexture(gl::TEXTURE0);}
			unsafe {gl::BindTexture(gl::TEXTURE_2D, texture.id.0);}
		}
	}

	fn unbind_texture(&self) {
		if let Some(_) = &self.maybe_texture {
			unsafe {gl::BindTexture(gl::TEXTURE_2D, 0);}
		}
	}

	pub fn draw_buffer(&self, mode : DrawMode) -> Result<(), GLError> {
		if let Some(indices) = &self.indices {
			self.draw_indexed_buffer(indices);
		}
		else {
			let n_elems = self.n_elems.get().ok_or(GLError::NoBufferAttached)?;
			self.draw_buffer_partial(0, n_elems, mode);
		}
		Ok(())
	}

	pub fn draw_indexed_buffer(&self, indices : &IndexBuffer) -> () {
		self.bind_texture();
		unsafe {gl::BindVertexArray(self.vao.0);}
		unsafe {gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, indices.id.0);}
		unsafe {
			gl::DrawElements(
				DrawMode::Tris.to_gl(), 
				indices.n_elems as gl::types::GLsizei, 
				gl::UNSIGNED_INT, 
				std::ptr::null()
				);
		}
		self.unbind_texture();
	}

	pub fn draw_buffer_partial(&self, from : usize, how_many : usize, mode : DrawMode) -> () {
		self.bind_texture();
		unsafe {gl::BindVertexArray(self.vao.0);}
		unsafe {gl::DrawArrays(mode.to_gl(), from as gl::types::GLint, how_many as gl::types::GLsizei);}
		self.unbind_texture();
	}

	pub fn draw_buffer_partial_multi<A>(&self, _buffer : &Buffer<A>, ranges : &[(usize, usize)], mode : DrawMode) -> () {
		self.bind_texture();
		let starts = ranges.iter().map(|(x, _)| *x as gl::types::GLint).collect::<Vec<_>>();
		let counts = ranges.iter().map(|(_, y)| *y as gl::types::GLsizei).collect::<Vec<_>>();
		unsafe {gl::BindVertexArray(self.vao.0);}
		unsafe {
			gl::MultiDrawArrays(
				mode.to_gl(), 
				starts.as_ptr(), 
				counts.as_ptr(), 
				ranges.len() as gl::types::GLsizei
				);
		}
		self.unbind_texture();
	}


}

#[derive(Debug)]
pub enum DrawMode {
	Tris,
	Points,
	Lines,
	LineStrip,
	TriStrip,
}

impl DrawMode {
	fn to_gl(&self) -> GLuint {
		match self {
			DrawMode::Tris           => gl::TRIANGLES,
			DrawMode::Points         => gl::POINTS,
			DrawMode::Lines          => gl::LINES,
			DrawMode::LineStrip      => gl::LINE_STRIP,
			DrawMode::TriStrip       => gl::TRIANGLE_STRIP,
		}
	}
}
