
#[derive(Debug)]
pub enum GLError {
	CouldNotCreateBuffer,
	CouldNotCreateTexture,
	CouldNotCreateVAO,
	IsntUniformBuffer,
	ImageTypeNotImplemented,
	LinkProgram(String),
	CompileError(String),
	FileError(std::io::Error),
	InexistentOrUndeclaredAttribute(String),
	InexistentUniform(String),
	InexistentUniformBuffer(String),
	UnregisteredVAO,
	NoBufferAttached,
	CannotGetAttributeCountOnProgram,
	AttributeNameTooLong,
	AttributeNameEncodingError,
	BufferTooSmallForConversion,
	TooManyTextures,
	CannotGetMaxTexUnits,
	CouldNotCreateFrameBuffer,
	IncompleteFrameBuffer(FrameBufferStatus),
}


impl From<std::io::Error> for GLError {
	fn from(err: std::io::Error) -> Self { Self::FileError(err) }
}

#[derive(Debug)]
pub enum FrameBufferStatus {
	Undefined,
	IncompleteAttachment,
	IncompleteMissingAttachment,
	IncompleteDrawBuffer,
	IncompleteReadBuffer,
	AttachmentObjectType,
	Unsupported,
	IncompleteMultisample,
	IncompleteLayerTargets,
}

impl FrameBufferStatus {
	pub fn from_opengl_sym(symbol : gl::types::GLuint) -> Option<Self> {
		match symbol {
			gl::FRAMEBUFFER_UNDEFINED                     => Some(FrameBufferStatus::Undefined),
			gl::FRAMEBUFFER_INCOMPLETE_ATTACHMENT         => Some(FrameBufferStatus::IncompleteAttachment),
			gl::FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT => Some(FrameBufferStatus::IncompleteMissingAttachment),
			gl::FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER        => Some(FrameBufferStatus::IncompleteDrawBuffer),
			gl::FRAMEBUFFER_INCOMPLETE_READ_BUFFER        => Some(FrameBufferStatus::IncompleteReadBuffer),
			gl::FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE        => Some(FrameBufferStatus::AttachmentObjectType),
			gl::FRAMEBUFFER_UNSUPPORTED                   => Some(FrameBufferStatus::Unsupported),
			gl::FRAMEBUFFER_INCOMPLETE_MULTISAMPLE        => Some(FrameBufferStatus::IncompleteMultisample),
			gl::FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS      => Some(FrameBufferStatus::IncompleteLayerTargets),
			_ => None
		}
	}

 	pub fn to_opengl_sym(&self) -> gl::types::GLuint {
		match self {
			FrameBufferStatus::Undefined                   => gl::FRAMEBUFFER_UNDEFINED,
			FrameBufferStatus::IncompleteAttachment        => gl::FRAMEBUFFER_INCOMPLETE_ATTACHMENT,
			FrameBufferStatus::IncompleteMissingAttachment => gl::FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT,
			FrameBufferStatus::IncompleteDrawBuffer        => gl::FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER,
			FrameBufferStatus::IncompleteReadBuffer        => gl::FRAMEBUFFER_INCOMPLETE_READ_BUFFER,
			FrameBufferStatus::AttachmentObjectType        => gl::FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE,
			FrameBufferStatus::Unsupported                 => gl::FRAMEBUFFER_UNSUPPORTED,
			FrameBufferStatus::IncompleteMultisample       => gl::FRAMEBUFFER_INCOMPLETE_MULTISAMPLE,
			FrameBufferStatus::IncompleteLayerTargets      => gl::FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS,
		}
	} 
}


