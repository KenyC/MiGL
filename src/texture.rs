use image::DynamicImage;
use image::GenericImageView;
use crate::GLError;

#[derive(Debug, Clone)]
pub enum TexAxis {
	U_AXIS, // horizontal 
	V_AXIS, // vertical
}

#[derive(Debug, Clone)]
pub struct TextureId(pub gl::types::GLuint);

#[derive(Debug, Clone)]
pub struct Texture {
	pub id : TextureId,
	pub width  : u32,
	pub height : u32,
}

impl Texture {
	pub fn new(image: &DynamicImage) -> Result<Self, GLError> {
		let mut id = 0;
		unsafe {gl::GenTextures(1, &mut id);}
		if id == 0 {
			return Err(GLError::CouldNotCreateTexture);
		}


		let (width, height) = image.dimensions();

		let storage_type = match image {
			DynamicImage::ImageRgb8(_)  => Ok(gl::RGB),
			DynamicImage::ImageLuma8(_) => Ok(gl::RED),
			DynamicImage::ImageRgba8(_) => Ok(gl::RGBA),
			_ => Err(GLError::ImageTypeNotImplemented),
		}? as gl::types::GLint;

		let image_type = match image {
			DynamicImage::ImageRgb8(_)  => Ok(gl::RGB),
			DynamicImage::ImageLuma8(_) => Ok(gl::RED),
			DynamicImage::ImageRgba8(_) => Ok(gl::RGBA),
			_ => Err(GLError::ImageTypeNotImplemented),
		}?;

		let pixel_type = match image {
			DynamicImage::ImageRgb8(_)  |
			DynamicImage::ImageLuma8(_) |
			DynamicImage::ImageRgba8(_) => Ok(gl::UNSIGNED_BYTE),
			_ => Err(GLError::ImageTypeNotImplemented),
		}?;

		unsafe {gl::BindTexture(gl::TEXTURE_2D, id);}
		unsafe {
			gl::BindTexture(gl::TEXTURE_2D, id);
			gl::TexImage2D(
				gl::TEXTURE_2D, 
				0, 
				storage_type, 
				width  as gl::types::GLint, 
				height as gl::types::GLint, 
				0, 
				image_type, 
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
			id : TextureId(id),
			width, height,
		})
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
				TexAxis::U_AXIS => gl::TEXTURE_WRAP_S,
				TexAxis::V_AXIS => gl::TEXTURE_WRAP_T,
			};
			unsafe {
				gl::TexParameteri(gl::TEXTURE_2D, wrap_axis, repeat_param);	
			}
		}
		unsafe {gl::BindTexture(gl::TEXTURE_2D, 0);}
	}
}