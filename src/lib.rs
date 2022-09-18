pub mod shader;
pub mod texture;
pub mod log;
pub mod attributes;
pub mod program;
pub mod uniform;
pub mod buffer;
pub mod error;
pub mod math3d;

extern crate gl;

use crate::error::*;


#[derive(Debug)]
pub struct GLWrap {
	next_free_uniform_binding_pt : UniformBindingPoint,
}


#[cfg(feature = "sdl2")]
use sdl2::VideoSubsystem;
	
impl GLWrap {
	#[cfg(feature = "sdl2")]
	pub fn new(video : VideoSubsystem) -> Self {
		Self::new_from(|s| video.gl_get_proc_address(s) as *const std::os::raw::c_void)
	}

	pub fn new_from<F>(loadfn : F) -> Self 
	where F : FnMut(&'static str) -> *const std::os::raw::c_void
	{
		gl::load_with(loadfn);
		unsafe {
			gl::DepthFunc(gl::LESS);
			gl::Enable(gl::DEPTH_TEST);
			// gl::Enable(gl::TEXTURE_2D);
			// gl::Enable(gl::MULTISAMPLE);  
		}
		let next_free_uniform_binding_pt = UniformBindingPoint(0);
		Self {next_free_uniform_binding_pt}
	}

	pub fn set_clear_color(&self, r : f32, g : f32, b : f32, a : f32)
	{
		unsafe {
			gl::ClearColor(r, g, b, a);
		}
	}

	pub fn set_line_width(&self, w : f32)
	{
		unsafe {gl::LineWidth(w);}
	}

	pub fn set_viewport(&self, x : i32, y : i32, w : i32, h : i32)
	{
		unsafe {
			gl::Viewport(x, y, w, h);
		}
	}

	pub fn clear(&self)
	{
		unsafe {
			gl::Clear(gl::COLOR_BUFFER_BIT);
			gl::Clear(gl::DEPTH_BUFFER_BIT);
		}
	}

	pub fn new_binding_point(&mut self) -> UniformBindingPoint {
		let id = self.next_free_uniform_binding_pt.0;
		self.next_free_uniform_binding_pt.0 += 1;
		UniformBindingPoint(id)
	}


}


#[derive(Debug)]
pub struct UniformBindingPoint(pub gl::types::GLuint);







