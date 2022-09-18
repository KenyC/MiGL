use crate::math3d::*;
use crate::program::*;

use gl::types::*;



// -- UNIFORMS


#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct LayoutLocation(pub GLint);

#[derive(Debug, Clone)]
pub struct Uniform<'a, T : UniformData + ?Sized> {
	pub program_id  : &'a ProgramId,
	pub location    : LayoutLocation,
	pub phantom     : std::marker::PhantomData<T>
}

impl<'a, T : UniformData + ?Sized> Uniform<'a, T> {
	pub fn pass(&self, data : &T) {
		unsafe {gl::UseProgram(self.program_id.0);}
		data.pass(self)
	}
}

pub trait UniformData {
	fn pass(&self, uniform : &Uniform<Self>) -> ();
}



impl UniformData for M44 {
	fn pass(&self, uniform : &Uniform<Self>) -> () {
		unsafe {
			gl::UniformMatrix4fv(
				uniform.location.0,
				1,
				gl::TRUE,
				self.0.as_ptr().cast()
			);
		}
	}
}


impl UniformData for [M44] {
	fn pass(&self, uniform : &Uniform<Self>) -> () {
		unsafe {
			gl::UniformMatrix4fv(
				uniform.location.0,
				self.len() as gl::types::GLint,
				gl::TRUE,
				self.as_ptr().cast(),
			);
		}
	}
}

impl UniformData for V3 {
	fn pass(&self, uniform : &Uniform<Self>) -> () {
		unsafe {
			gl::Uniform3f(
				uniform.location.0,
				self.0[0],
				self.0[1],
				self.0[2],
			);
		}
	}
}

impl UniformData for V2 {
	fn pass(&self, uniform : &Uniform<Self>) -> () {
		unsafe {
			gl::Uniform2f(
				uniform.location.0,
				self.0[0],
				self.0[1],
			);
		}
	}
}

impl UniformData for f32 {
	fn pass(&self, uniform : &Uniform<Self>) -> () {
		unsafe {
			gl::Uniform1f(
				uniform.location.0,
				*self,
			);
		}
	}
}

impl UniformData for gl::types::GLuint {
	fn pass(&self, uniform : &Uniform<Self>) -> () {
		unsafe {
			gl::Uniform1ui(
				uniform.location.0,
				*self,
			);
		}
	}
}

impl UniformData for gl::types::GLint {
	fn pass(&self, uniform : &Uniform<Self>) -> () {
		unsafe {
			gl::Uniform1i(
				uniform.location.0,
				*self,
			);
		}
	}
}