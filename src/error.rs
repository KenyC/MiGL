
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
}


impl From<std::io::Error> for GLError {
	fn from(err: std::io::Error) -> Self { Self::FileError(err) }
}