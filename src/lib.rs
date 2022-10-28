pub mod shader;
pub mod texture;
pub mod log;
#[cfg(feature = "utils")]
pub mod utils;
pub mod attributes;
pub mod program;
pub mod uniform;
pub mod frame;
pub mod buffer;
pub mod error;
pub mod math3d;

extern crate gl;

use crate::error::*;
use frame::{FrameBuffer, FrameBufferId};


#[derive(Debug)]
pub struct GLWrap {
	next_free_uniform_binding_pt : UniformBindingPoint,
	default_framebuffer : FrameBuffer,
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
		Self::enable_depth();
		let next_free_uniform_binding_pt = UniformBindingPoint(0);
		Self {next_free_uniform_binding_pt, default_framebuffer: FrameBuffer::default() }
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

	pub fn set_point_size(&self, size : f32)
	{
		unsafe {gl::PointSize(size);}
	}

	pub fn set_viewport(&self, x : i32, y : i32, w : i32, h : i32)
	{
		unsafe {
			gl::Viewport(x, y, w, h);
		}
	}

	pub fn enable_program_point_size(&self) 
	{
		unsafe {gl::Enable(gl::PROGRAM_POINT_SIZE); }
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

	pub fn default_framebuffer(&self) -> &FrameBuffer { &self.default_framebuffer }

	pub fn enable_depth() {
		unsafe {
			gl::DepthFunc(gl::LESS);
			gl::Enable(gl::DEPTH_TEST);
		}
	}
}



#[derive(Debug)]
pub struct UniformBindingPoint(pub gl::types::GLuint);







