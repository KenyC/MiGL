use crate::{texture::Texture, error::{GLError, FrameBufferStatus}};

#[derive(Debug)]
pub struct FrameBufferId(gl::types::GLuint);

impl Default for FrameBufferId {
    fn default() -> Self { FrameBufferId(0) }
}

#[derive(Debug)]
pub struct FrameBuffer {
	id : FrameBufferId,
	has_depth_buffer : bool,
}

impl Default for FrameBuffer {
    fn default() -> Self {
        Self { id: FrameBufferId::default(), has_depth_buffer: true }
    }
}

impl FrameBuffer {
	pub fn clear(&self) {
		unsafe { gl::Clear(gl::COLOR_BUFFER_BIT); }
		if self.has_depth_buffer {
			unsafe { gl::Clear(gl::DEPTH_BUFFER_BIT); }
		}
	}


	pub fn make_current(&self) {
		unsafe { gl::BindFramebuffer(gl::FRAMEBUFFER, self.id.0); }
	}
}


pub struct FrameBufferBuilder<'a> {
	color_texture :   Option<&'a Texture>,
	depth_texture :   Option<&'a Texture>,
	stencil_texture : Option<&'a Texture>,
}

impl<'a> FrameBufferBuilder<'a> {
	pub fn new() -> Self { 
		Self {
			color_texture:   None,
			depth_texture:   None,
			stencil_texture: None,
		} 
	}

	pub fn attach_color(mut self, texture: &'a Texture) -> Self {
		self.color_texture = Some(texture);
		self
	}
	pub fn attach_depth(mut self, texture: &'a Texture) -> Self {
		self.depth_texture = Some(texture);
		self
	}
	pub fn attach_stencil(mut self, texture: &'a Texture) -> Self {
		self.stencil_texture = Some(texture);
		self
	}

	pub fn build(self) -> Result<FrameBuffer, GLError> {
		let FrameBufferBuilder { color_texture, depth_texture, stencil_texture } = self;
		
		let mut id : gl::types::GLuint = 0;
		unsafe {gl::GenFramebuffers(1, &mut id)}		
		if id == 0 {
			return Err(GLError::CouldNotCreateFrameBuffer); 
		}

		unsafe {gl::BindFramebuffer(gl::FRAMEBUFFER, id);}		

		if let Some(color_texture) = color_texture {
			unsafe {
				gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, color_texture.id.0, 0)
			}
		}

		if let Some(depth_texture) = depth_texture {
			unsafe {
				gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::DEPTH_ATTACHMENT, gl::TEXTURE_2D, depth_texture.id.0, 0)
			}
		}

		if let Some(stencil_texture) = stencil_texture {
			unsafe {
				gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::STENCIL_ATTACHMENT, gl::TEXTURE_2D, stencil_texture.id.0, 0)
			}
		}

		let check_status = unsafe  { gl::CheckFramebufferStatus(gl::FRAMEBUFFER) };
		if check_status != gl::FRAMEBUFFER_COMPLETE  {
			return Err(GLError::IncompleteFrameBuffer(FrameBufferStatus::from_opengl_sym(check_status).unwrap()));
		}

		unsafe {gl::BindFramebuffer(gl::FRAMEBUFFER, 0);}		

		Ok(FrameBuffer {
			id: FrameBufferId(id),
			has_depth_buffer: depth_texture.is_some(),
		})		
	}
}
