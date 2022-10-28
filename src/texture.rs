use std::ffi::c_void;
use std::ptr::null;

use gl::types::GLuint;
use image::DynamicImage;
use image::GenericImageView;
use image::ImageBuffer;
use crate::GLError;
use crate::attributes::GLType;

#[derive(Debug, Clone, Copy)]
pub enum TexAxis {
	UAxis, // horizontal 
	VAxis, // vertical
}


#[derive(Debug, Clone, Copy)]
pub enum TexFormat {
	Monochrome,
	Rgb,
	Rgba,
	Depth,
	DepthStencil,
}


impl TexFormat {
	fn to_opengl_sym(self) -> i32 {
		match self {
			TexFormat::Monochrome => gl::RED  as gl::types::GLint,
			TexFormat::Rgb        => gl::RGB  as gl::types::GLint,
			TexFormat::Rgba       => gl::RGBA as gl::types::GLint,
			TexFormat::Depth      => gl::DEPTH_COMPONENT as gl::types::GLint,
			TexFormat::DepthStencil => gl::DEPTH_STENCIL as gl::types::GLint,
		}
	}
}

#[derive(Debug, Clone)]
pub struct TextureId(pub gl::types::GLuint);

impl TextureId {
	fn new() -> Result<TextureId, GLError> {
		let mut id = 0;
		unsafe {gl::GenTextures(1, &mut id);}
		if id == 0 {
			return Err(GLError::CouldNotCreateTexture);
		}
		Ok(TextureId(id))
	}
}

#[derive(Debug, Clone)]
pub struct Texture {
	pub id : TextureId,
	pub width  : u32,
	pub height : u32,
	pub tex_format : TexFormat,
}

impl Texture {

	pub fn allocate(
		width : u32, height : u32, 
		format : TexFormat,
	) -> Result<Self, GLError> 
	{
		let id = TextureId::new()?;

		unsafe {gl::BindTexture(gl::TEXTURE_2D, id.0);}
		unsafe {
			let opengl_sym = format.to_opengl_sym();
			gl::TexImage2D(
				gl::TEXTURE_2D, 
				0, 
				opengl_sym, 
				width  as gl::types::GLint, 
				height as gl::types::GLint, 
				0, 
				opengl_sym as gl::types::GLuint, // should be irrelevant ?
				GLType::Ubyte.to_opengl_sym(), // should be irrelevant ?
				null(),
			);
		}

		// Set interpolation
		unsafe {
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as gl::types::GLint);
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR  as gl::types::GLint);
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as gl::types::GLint);	
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as gl::types::GLint);
		}
		unsafe {gl::BindTexture(gl::TEXTURE_2D, 0);}

		Ok(Self {
			id,
			width, height,
			tex_format : format,
		})
	}



	pub fn new_stored_as(image: &DynamicImage, format : TexFormat) -> Result<Self, GLError> {
		let id = TextureId::new()?;


		let (width, height) = image.dimensions();

		let storage_type = format.to_opengl_sym();
		// let storage_type = format_size_to_sym(format, gl_type);

		let image_format = match image {
			DynamicImage::ImageRgb8(_)    |
			DynamicImage::ImageRgb16(_)   |
			DynamicImage::ImageRgb32F(_)  => Ok(TexFormat::Rgb),
			DynamicImage::ImageLuma8(_)   |
			DynamicImage::ImageLuma16(_)  => Ok(TexFormat::Monochrome),
			DynamicImage::ImageRgba8(_)   |
			DynamicImage::ImageRgba16(_)  |
			DynamicImage::ImageRgba32F(_) => Ok(TexFormat::Rgba),
			_ => Err(GLError::ImageTypeNotImplemented),
		}?;

		let pixel_type = match image {
			DynamicImage::ImageRgb8(_)    |
			DynamicImage::ImageLuma8(_)   |
			DynamicImage::ImageLumaA8(_)  |
			DynamicImage::ImageRgba8(_)   => Ok(gl::UNSIGNED_BYTE),
			DynamicImage::ImageLuma16(_)  |
			DynamicImage::ImageLumaA16(_) |
			DynamicImage::ImageRgb16(_)   |
			DynamicImage::ImageRgba16(_)  => Ok(gl::UNSIGNED_SHORT),
			DynamicImage::ImageRgb32F(_)  |
			DynamicImage::ImageRgba32F(_) => Ok(gl::FLOAT),
			_ => Err(GLError::ImageTypeNotImplemented),
		}?;

		unsafe {gl::BindTexture(gl::TEXTURE_2D, id.0);}
		unsafe {
			gl::TexImage2D(
				gl::TEXTURE_2D, 
				0, 
				storage_type, 
				width  as gl::types::GLint, 
				height as gl::types::GLint, 
				0, 
				image_format.to_opengl_sym() as gl::types::GLuint, 
				pixel_type, 
				image.as_bytes().as_ptr().cast(),
			);
		}

		// Set interpolation
		unsafe {
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as gl::types::GLint);
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR  as gl::types::GLint);
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as gl::types::GLint);	
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as gl::types::GLint);
		}
		unsafe {gl::BindTexture(gl::TEXTURE_2D, 0);}

		Ok(Self {
			id,
			width, height,
			tex_format : image_format,
		})
	}

	pub fn new(image: &DynamicImage) -> Result<Self, GLError> {
		let format = match image {
			DynamicImage::ImageRgb8(_)    |
			DynamicImage::ImageRgb16(_)   |
			DynamicImage::ImageRgb32F(_)  => Ok(TexFormat::Rgb),
			DynamicImage::ImageLuma8(_)   |
			DynamicImage::ImageLuma16(_)  => Ok(TexFormat::Monochrome),
			DynamicImage::ImageRgba8(_)   |
			DynamicImage::ImageRgba16(_)  |
			DynamicImage::ImageRgba32F(_) => Ok(TexFormat::Rgba),
			_ => Err(GLError::ImageTypeNotImplemented),
		}?;

		let gl_type = match image {
			DynamicImage::ImageRgb8(_)    |
			DynamicImage::ImageLuma8(_)   |
			DynamicImage::ImageRgba8(_)   => Ok(GLType::Ubyte),
			DynamicImage::ImageRgb16(_)   |
			DynamicImage::ImageLuma16(_)  |
			DynamicImage::ImageRgba16(_)  => Ok(GLType::Ushort), 
			DynamicImage::ImageRgb32F(_)  | 
			DynamicImage::ImageRgba32F(_) => Ok(GLType::Float),
			_ => Err(GLError::ImageTypeNotImplemented),
		}?;


		Self::new_stored_as(image, format)
	}

	pub fn clamp(&self, dimensions : &[TexAxis])  {
		self.set_wrap(dimensions, gl::CLAMP_TO_BORDER as gl::types::GLint)
	}

	pub fn repeat(&self, dimensions : &[TexAxis])  {
		self.set_wrap(dimensions, gl::REPEAT as gl::types::GLint)
	}

	fn set_wrap(&self, dimensions :  &[TexAxis], repeat_param : gl::types::GLint) {
		unsafe {gl::BindTexture(gl::TEXTURE_2D, self.id.0);}
		for dimension in dimensions {
			let wrap_axis = match dimension {
				TexAxis::UAxis => gl::TEXTURE_WRAP_S,
				TexAxis::VAxis => gl::TEXTURE_WRAP_T,
			};
			unsafe {
				gl::TexParameteri(gl::TEXTURE_2D, wrap_axis, repeat_param);	
			}
		}
		unsafe {gl::BindTexture(gl::TEXTURE_2D, 0);}
	}

	// account for more types and channel
	pub fn to_image(&self, gl_type : GLType) -> Option<ImageBuffer<image::Rgb<u8>, Vec<u8>>> {
		let capacity : usize = (self.width * self.height).try_into().unwrap();
		let mut bytes : Vec<u8> = Vec::with_capacity(capacity * 3);
		unsafe {
			gl::BindTexture(gl::TEXTURE_2D, self.id.0);
			gl::GetTexImage(
				gl::TEXTURE_2D,
				0,
				self.tex_format.to_opengl_sym() as gl::types::GLuint,
				gl_type.to_opengl_sym(),
				bytes.as_mut_ptr().cast(),
			);
			bytes.set_len(capacity * 3);
		}
		ImageBuffer::<image::Rgb<u8>, Vec<u8>>::from_raw(self.width, self.height, bytes)
	}
}

