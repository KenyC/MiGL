use crate::math3d::*;

#[derive(Debug, Clone, Copy)]
pub enum GLType {
	Int,
	Uint,
	Byte,
	Ubyte,
	Short,
	Ushort,
	Float,
}

impl GLType {
	pub fn is_integer(&self) -> bool {
		match self {
			Self::Float  => false,
			Self::Int    => true, 
			Self::Uint   => true, 
			Self::Byte   => true,
			Self::Ubyte  => true,
			Self::Short  => true,
			Self::Ushort => true,
		}
	}

	pub fn to_opengl_sym(&self) -> gl::types::GLenum {
		match self {
			Self::Int    => gl::INT, 
			Self::Uint   => gl::UNSIGNED_INT, 
			Self::Byte   => gl::BYTE,
			Self::Ubyte  => gl::UNSIGNED_BYTE,
			Self::Short  => gl::SHORT,
			Self::Ushort => gl::UNSIGNED_SHORT,
			Self::Float  => gl::FLOAT, 
		}
	}
}



// -- ATTRIBUTES

// Type info for GPU
// describes an array of "n_components" elements of type "gl_type"'s
#[derive(Debug, Clone)]
pub struct GPUInfo {
	pub n_components: usize,
	pub gl_type:      GLType,
}




pub trait GPUData {
	const INFO: GPUInfo;
}

pub trait GPUIndex : GPUData {}

impl GPUData for V2 {
	const INFO: GPUInfo = GPUInfo {
		n_components : 2,
		gl_type   : GLType::Float,
	};
}

impl GPUData for V3 {
	const INFO: GPUInfo = GPUInfo {
		n_components : 3,
		gl_type   : GLType::Float,
	};
}

impl GPUData for V4 {
	const INFO: GPUInfo = GPUInfo {
		n_components : 4,
		gl_type   : GLType::Float,
	};
}


impl GPUData for gl::types::GLfloat {
	const INFO: GPUInfo = GPUInfo {
		n_components : 1,
		gl_type   : GLType::Float,
	};
}


impl GPUData for gl::types::GLint {
	const INFO: GPUInfo = GPUInfo {
		n_components : 1,
		gl_type   : GLType::Int,
	};
}



impl GPUIndex for gl::types::GLuint {}
impl GPUData  for gl::types::GLuint {
	const INFO: GPUInfo = GPUInfo {
		n_components : 1,
		gl_type   : GLType::Uint,
	};
}



impl GPUIndex for gl::types::GLushort {}
impl GPUData  for gl::types::GLushort {
	const INFO: GPUInfo = GPUInfo {
		n_components : 1,
		gl_type   : GLType::Ushort,
	};
}




impl GPUIndex for gl::types::GLubyte {}
impl GPUData  for gl::types::GLubyte {
	const INFO: GPUInfo = GPUInfo {
		n_components : 1,
		gl_type   : GLType::Ubyte,
	};
}



