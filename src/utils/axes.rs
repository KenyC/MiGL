use crate::buffer::BufferBld;
use crate::GLError;
use crate::shader::{Shader, Vertex, Fragment};
use crate::program::{Program, ProgramBuilder, DrawMode};
use crate::math3d::{V3, M44};
use crate::buffer::Buffer;
use crate::uniform::Uniform;


#[derive(Debug)]
pub struct AxesBuilder {
	buffer:   Buffer<V3>,
	program:  Program,
}

pub struct Axes<'a> {
	model:   M44,
	builder: &'a AxesBuilder,
	model_uniform: Uniform<'a, M44>,
	vp_uniform:    Uniform<'a, M44>,
}


impl AxesBuilder {
	pub fn new() -> Result<Self, GLError> {
		let data = [
			V3::ZERO,
			V3::E_X,
			V3::ZERO,
			V3::E_Y,
			V3::ZERO,
			V3::E_Z,
		];

		let buffer = 
			BufferBld::array()
			.data(&data)?;

		let program = 
			ProgramBuilder::new(
				Shader::<Vertex>::from_str(include_str!("../../resources/shaders/axes/vert.glsl"))?,
				Shader::<Fragment>::from_str(include_str!("../../resources/shaders/axes/frag.glsl"))?,
			)
			.attributes(&["position"])
			.build()?
		;

		program.bind("position", buffer.direct_view())?;

		let model = M44::id();

		program.uniform("model").unwrap().pass(&model);

		Ok(Self {
			buffer,
			program,
		})
	}

	pub fn axes(&self) -> Axes { 
		let model_uniform = self.program.uniform("model").unwrap();
		let vp_uniform = self.program.uniform("view_projection").unwrap();
		let model = M44::id();
		Axes { model, builder : self, model_uniform, vp_uniform } 
	}
}

impl<'a> Axes<'a> {


	pub fn set_model(&mut self, model : &M44) {
		self.model = model.clone();
	}


	pub fn set_pos(&mut self, pos : V3) {
		for i in 0 .. 3 {
			self.model.0[i][3] = pos.0[i];
		}
	}

	pub fn draw(&self, vp : &M44) -> Result<(), GLError> {
		self.builder.program.set_current();
		self.vp_uniform.pass(vp);
		self.model_uniform.pass(&self.model);
		self.builder.program.draw_buffer(&self.builder.buffer, DrawMode::Lines);
		Ok(())
	}
}

