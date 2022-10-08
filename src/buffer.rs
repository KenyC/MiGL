use std::ops::Range;

use gl::types::*;

use crate::attributes::*;
use crate::error::*;
use crate::*;
use crate::program::AttributePos;





// -- BUFFER
#[derive(Debug, Clone, Copy)]
pub struct BufferId(pub GLuint);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct VAOId(pub GLuint);


#[derive(Debug, PartialEq, Eq)]
pub enum BufferKind {
	ArrayBuffer,
	IndexBuffer,
	UniformBuffer,
}


impl BufferKind {
	pub fn cst(&self) -> gl::types::GLenum {
		match self {
			Self::ArrayBuffer   => gl::ARRAY_BUFFER,
			Self::IndexBuffer   => gl::ELEMENT_ARRAY_BUFFER,
			Self::UniformBuffer => gl::UNIFORM_BUFFER,
		}
	}
}

#[derive(Debug)]
pub enum UpdateKind {
	Dynamic,
	Static,
}

impl UpdateKind {
	pub fn cst(&self) -> gl::types::GLenum {
		match self {
			Self::Static  => gl::STATIC_DRAW,
			Self::Dynamic => gl::DYNAMIC_DRAW,
		}
	}
}

#[derive(Debug)]
pub struct BufferBld {
	pub kind    : BufferKind,
	pub update  : UpdateKind,
}

impl BufferBld {
	pub fn array() -> Self
	{ 
		Self {
			kind   : BufferKind::ArrayBuffer,
			update : UpdateKind::Static,
		} 
	}

	pub fn indices() -> Self
	{ 
		Self {
			kind   : BufferKind::IndexBuffer,
			update : UpdateKind::Static,
		}
	}

	pub fn uniform() -> Self
	{ 
		Self {
			kind   : BufferKind::UniformBuffer,
			update : UpdateKind::Static,
		}
	}

	pub fn r#static(mut self) -> Self {
		self.update = UpdateKind::Static;
		self
	}

	pub fn dynamic(mut self) -> Self {
		self.update = UpdateKind::Dynamic;
		self
	}

	pub fn data<A>(self, data : &[A]) -> Result<Buffer<A>, GLError> {
		Buffer::from_data(
			self.kind,
			self.update,
			data,
		)
	}

}


#[derive(Debug)]
pub struct Buffer<T> {
	pub id      : BufferId,
	pub n_elems : usize,
	pub kind    : BufferKind,
	_phantom    : std::marker::PhantomData<T>
}

impl<A> Buffer<A> {

	pub fn cast<B>(self) -> Buffer<B> {
		let Self { id, n_elems, kind, .. } = self;
		Buffer::<B> {
			id,
			n_elems,
			kind,
			_phantom: std::marker::PhantomData,
		}
	}

	pub fn from_data(
		kind    : BufferKind,
		update  : UpdateKind,
		data    : &[A],
	) -> Result<Self, GLError>
	{
		let mut buffer_id = 0;
		unsafe {
			gl::GenBuffers(1, &mut buffer_id);
		}
		if buffer_id == 0 {
			return Err(GLError::CouldNotCreateBuffer);
		}

		unsafe {
			gl::BindBuffer(kind.cst(), buffer_id)
		}

		unsafe {
			gl::BufferData(
				kind.cst(),
				std::mem::size_of_val(data) as gl::types::GLsizeiptr,
				data.as_ptr().cast(),
				update.cst(),
			)
		}
		unsafe {
			gl::BindBuffer(kind.cst(), 0)
		}
		Ok(Self {
			id       : BufferId(buffer_id),
			n_elems  : data.len(),
			kind,
			_phantom : std::marker::PhantomData, 
		})
	}


	pub fn replace_data(
		&self,
		offset  : usize,
		data    : &[A],
	) 
	{

		unsafe {
			gl::BindBuffer(self.kind.cst(), self.id.0)
		}

		unsafe {
			gl::BufferSubData(
				self.kind.cst(),
				offset as gl::types::GLintptr,
				std::mem::size_of_val(data) as gl::types::GLsizeiptr,
				data.as_ptr().cast(),
			)
		}
		unsafe {
			gl::BindBuffer(self.kind.cst(), 0)
		}
	}



	pub fn view<'a, B : GPUData, T>(&'a self, get_field : T) -> BufferView
	where T : FnOnce(*const A) -> *const B
	{
		let uninit = std::mem::MaybeUninit::<A>::uninit();
		let base_ptr  = uninit.as_ptr();
		
		let field_ptr = get_field(base_ptr);
		let offset = (field_ptr as usize) - (base_ptr as usize);

		let data_info = B::INFO;


		BufferView {
			buffer_id  : self.id,
			n_elems    : self.n_elems,
			stride     : std::mem::size_of::<A>(),
			data_info, 
			offset,
		}
	}


	pub fn register(self, gl : &mut GLWrap) -> Result<UniformBuffer<A>, GLError>  {
		if self.kind != BufferKind::UniformBuffer  {
			Err(GLError::IsntUniformBuffer)
		}
		else {
			let binding_point = gl.new_binding_point();

			unsafe {
				gl::BindBuffer(self.kind.cst(), self.id.0);
				gl::BindBufferBase(
					gl::UNIFORM_BUFFER, 
					binding_point.0, 
					self.id.0,
				); 
				gl::BindBuffer(gl::UNIFORM_BUFFER, 0);
			}

			Ok(UniformBuffer {
				buffer : self,
				binding_point,
			})
		}
	}
}

impl<A : GPUData> Buffer<A> {
	pub fn direct_view(&self) -> BufferView
	{
		self.view_range(0 .. self.n_elems)
	}

	pub fn view_range(&self, range : Range<usize>) -> BufferView
	{
		let stride = std::mem::size_of::<A>();
		BufferView {
			buffer_id  : self.id,
			n_elems    : range.len(),
			stride,
			data_info  : A::INFO, 
			offset     : range.start * stride,
		}
	}
}

#[macro_export]
macro_rules! field {
	($field : tt) => {(|buffer_ref| unsafe{ &((*buffer_ref).$field) as *const _})}
}


pub struct BufferView {
	pub buffer_id: BufferId,
	pub n_elems:   usize,
	pub stride:    usize,
	pub offset:    usize,
	pub data_info: GPUInfo,
}

impl BufferView {
    pub fn new(
    	buffer_id: BufferId, 
    	n_elems:   usize, 
    	stride:    usize, 
    	offset:    usize, 
    	data_info: GPUInfo
    ) -> Self { Self { buffer_id, stride, offset, data_info, n_elems } }

	pub fn bind_to(self, pos : AttributePos) {
		let BufferView {
			buffer_id, stride, offset,
			data_info : GPUInfo {n_components, gl_type}, ..
		} = self;
		unsafe {gl::BindBuffer(gl::ARRAY_BUFFER, buffer_id.0);}

		if gl_type.is_integer() {
			unsafe {
				gl::VertexAttribIPointer(
					pos.0,
					n_components as gl::types::GLint,
					gl_type.to_opengl_sym(),
					stride as gl::types::GLsizei,
					offset as *const _,
				)
			}
		}
		else {
			unsafe {
				gl::VertexAttribPointer(
					pos.0,
					n_components as gl::types::GLint,
					gl_type.to_opengl_sym(),
					gl::FALSE,
					stride as gl::types::GLsizei,
					offset as *const _,
				)
			}
		}
		unsafe {gl::BindBuffer(gl::ARRAY_BUFFER, 0);}
	}
}

pub type IndexBuffer = Buffer<gl::types::GLuint>;

#[derive(Debug)]
pub struct IndexedBuffer<A> {
	pub data:     Buffer<A>,
	pub indices:  IndexBuffer,
	pub n_elems:  usize,
}

impl<A> IndexedBuffer<A> {
	pub fn new(data : Buffer<A>, indices : Buffer<gl::types::GLuint>) -> Self {
		let n_elems = indices.n_elems;
		Self {data, indices, n_elems}
	}
}

pub struct UniformBuffer<D> {
	pub buffer : Buffer<D>,
	pub binding_point : UniformBindingPoint,
}


pub trait BindingPoint {
	fn to_int(&self) -> u32;
}



