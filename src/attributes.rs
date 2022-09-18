use crate::math3d::*;

#[derive(Debug)]
pub enum GLType {
	Int,
	Uint,
	Float,
}

impl GLType {
	pub fn is_integer(&self) -> bool {
		match &self {
			Self::Int   => true, 
			Self::Uint  => true, 
			Self::Float => false, 
		}
	}

	pub fn to_opengl_sym(&self) -> gl::types::GLenum {
		match &self {
			Self::Int   => gl::INT, 
			Self::Uint  => gl::UNSIGNED_INT, 
			Self::Float => gl::FLOAT, 
		}
	}
}



// -- ATTRIBUTES

#[derive(Debug)]
pub struct GPUInfo {
	pub n_components: usize,
	pub gl_type:      GLType,
}




pub trait GPUData {
	const INFO: GPUInfo;
}

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




impl GPUData for gl::types::GLuint {
	const INFO: GPUInfo = GPUInfo {
		n_components : 1,
		gl_type   : GLType::Uint,
	};
}


