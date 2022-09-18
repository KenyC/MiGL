use std::fs::File;
use std::io::Read;
use std::path::Path;

use gl::types::*;

use crate::log::*;
use crate::error::*;



pub struct Vertex;
impl ShaderKind for Vertex   {const GLENUM_VAL : gl::types::GLenum = gl::VERTEX_SHADER;}
pub struct Fragment;
impl ShaderKind for Fragment {const GLENUM_VAL : gl::types::GLenum = gl::FRAGMENT_SHADER;}
pub struct Geometry;
impl ShaderKind for Geometry {const GLENUM_VAL : gl::types::GLenum = gl::GEOMETRY_SHADER;}


pub trait ShaderKind {
	const GLENUM_VAL : gl::types::GLenum;
}

#[derive(Debug)]
pub struct Shader<K> {
	pub id  : GLuint,
	phantom : std::marker::PhantomData<K>
}




impl<K : ShaderKind> Shader<K> {
	
	pub fn from_str(source : &str) -> Result<Self, GLError> {
		let id = unsafe { gl::CreateShader(K::GLENUM_VAL) };
		let phantom = std::marker::PhantomData::<K>;

		unsafe {
			gl::ShaderSource(id, 1, 
				&source.as_bytes().as_ptr().cast(),
				&(source.len().try_into().unwrap()),
			);
			gl::CompileShader(id);
		}


		// -- CHECK IF ERROR HAS OCCURRED
		match get_log(id, LogKind::CompilerLog) {
			None            => Ok(Self {id, phantom}),
			Some(error_msg) => Err(GLError::CompileError(error_msg)),
		}
	}


	pub fn from_file(filepath : &str) -> Result<Self, GLError> {
		let path = Path::new(filepath);

		// Open the path in read-only mode, returns `io::Result<File>`
		let mut file = File::open(&path)?;


		// // Read the file contents into a string, returns `io::Result<usize>`
		let mut s = String::new();
		file.read_to_string(&mut s)?;


		Shader::<K>::from_str(&s)
	}
}


impl<K> Drop for Shader<K> {
	fn drop(&mut self) { 
		unsafe { gl::DeleteShader(self.id) }
	}
}
