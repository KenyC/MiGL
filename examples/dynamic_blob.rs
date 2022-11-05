use std::collections::HashMap;
use std::collections::HashSet;
use std::path::Path;

use image::ImageFormat;
use migl::buffer::Buffer;
use migl::buffer::BufferBld;
use migl::math3d::Point;
use migl::program::DrawMode;
use migl::texture::TexFormat;
use migl::uniform::UniformData;
use migl::utils::camera::CylinderCamera;
use migl::utils::load::ObjLoader;
use migl::math3d::M44;
use migl::math3d::V3;
use migl::program::ProgramBuilder;
use migl::shader::Fragment;
use migl::shader::Shader;
use migl::shader::Vertex;

use rand::Rng;
use rand::rngs::ThreadRng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::GLProfile;

use migl::*;
use serde::Deserialize;

const WIDTH  : u32 = 800;
const HEIGHT : u32 = 600;

pub fn main() {
	// Constants

	let sdl_context = sdl2::init().expect("Couldn't init SDL2");
	let video_subsystem = sdl_context.video().expect("Couldn't init video");
 
	let opengl_attr = video_subsystem.gl_attr();
	opengl_attr.set_context_version(3, 3);
	opengl_attr.set_context_profile(GLProfile::Core);

	let window = video_subsystem.window("Dynamic Blob", WIDTH, HEIGHT)
		.position_centered()
		.opengl()
		.build()
		.unwrap();


	let _gl_context = window.gl_create_context().unwrap();

	let gl = GLWrap::new(video_subsystem);

	gl.set_viewport(0, 0, WIDTH as i32, HEIGHT as i32);
	gl.set_clear_color(0.0, 0.0, 0.0, 1.0);

	// -- create camera
	let mut camera = CylinderCamera::new();
	camera.height_angle = 0.8;
	camera.radius = 8.;
	camera.angle = -75_f32.to_radians();
	let projection_matrix = M44::perspective_projection(0.1, 50., 60., (WIDTH as f32) / (HEIGHT as f32));

	// -- create blob
	let mut blob = Blob::new();



	// -- create buffer
	let mut buffer : Buffer<VertexNormal> =
		BufferBld::array()
		.dynamic()
		.allocate(6 * 6 * Blob::MAX_VOXELS ) // 100 voxel limit
		.unwrap();
	buffer.pass_data(&blob.gen_faces());


	// -- create program
	let program =
		ProgramBuilder::new(
			Shader::from_file("resources/shaders/diffuse/vert.glsl").unwrap(), 
			Shader::from_file("resources/shaders/diffuse/frag.glsl").unwrap(),
		)
		.build()
		.unwrap();

	program.bind("position", buffer.view(field!(position))).unwrap();
	program.bind("normal",   buffer.view(field!(normal))).unwrap();

	program.uniform("projection"       ).unwrap().pass(&projection_matrix);
	program.uniform("ambient_strength" ).unwrap().pass(&0.3);
	program.uniform("specular_strength").unwrap().pass(&0.3);
	program.uniform("diffuse_strength" ).unwrap().pass(&1.0);
	program.uniform("light_strength"   ).unwrap().pass(&0.8);
	program.uniform("light_direction"  ).unwrap().pass(&(V3::new([1.0, 1.0, 1.0]).normalize()));
	let mv_uniform = program.uniform("model_view").unwrap();
	let camera_pos_uniform = program.uniform("camera_pos").unwrap();



	// -- rng
	let mut rng = rand::thread_rng();


	let mut event_pump = sdl_context.event_pump().unwrap();

	'main: loop {

		for event in event_pump.poll_iter() {
			match event {
				Event::Quit {..} |
				Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
					break 'main
				},
				Event::KeyDown { keycode: Some(Keycode::I), .. } => {
					blob.add_random(&mut rng);
					buffer.pass_data(&blob.gen_faces());
					program.bind("position", buffer.view(field!(position))).unwrap();
					program.bind("normal",   buffer.view(field!(normal))).unwrap();
				},
				event => {
					camera.control(&event, 10.);
				},
			}
		}

		gl.clear();

		let view_matrix = camera.matrix();
		mv_uniform.pass(&view_matrix);
		let camera_pos = camera.position();
		camera_pos_uniform.pass(&camera_pos);

		program.draw_buffer(DrawMode::Tris).unwrap();


		window.gl_swap_window();
	}
}

#[derive(Debug)]
struct VertexNormal {
	position : V3,
	normal   : V3,
}


#[derive(Debug)]
struct Blob {
	voxels    : HashSet<[i64; 3]>,
	boundary  : HashSet<[i64; 3]>,
}

impl Blob {
	const MAX_VOXELS : usize = 100;
	fn new() -> Self {
		let mut voxels   = HashSet::with_capacity(Self::MAX_VOXELS);
		let mut boundary = HashSet::with_capacity(Self::MAX_VOXELS * 2);

		voxels.insert([0; 3]);

		for neighbour in Self::neighbours(&[0; 3]) {
			boundary.insert(neighbour);
		}

		Self { voxels, boundary }
	}

	fn neighbours(cell : &[i64; 3]) -> [[i64; 3]; 6] {
		let mut to_return = [cell.clone(); 6];
		let mut i = 0;
		for axis in [0, 1, 2] {
			for direction in [-1, 1] {
				to_return[i][axis] += direction;
				i += 1;
			}
		} 
		to_return
	}

	fn insert(&mut self, new_cell : [i64; 3]) {
		self.boundary.remove(&new_cell);
		for neighbour in Self::neighbours(&new_cell) {
			if !self.voxels.contains(&neighbour) {
				self.boundary.insert(neighbour);
			}
		}
		self.voxels.insert(new_cell);
	}

	fn add_random(&mut self, rng : &mut ThreadRng) {
		let n_boundary = self.boundary.len();
		let i = rng.gen_range(0 .. n_boundary);
		let new_cell = self.boundary.iter().nth(i).unwrap().clone();
		self.insert(new_cell);
	}

	fn gen_faces(&self) -> Vec<VertexNormal> {
		let mut to_return = Vec::with_capacity(self.boundary.len() * 2);

		for boundary_cell in self.boundary.iter() {
			for axis in [0, 1, 2] {
				for direction in [-1, 1] {
					let mut inner_cell = boundary_cell.clone();
					inner_cell[axis] += direction;

					if self.voxels.contains(&inner_cell) {
						Self::add_face(&mut to_return, boundary_cell, axis, direction)
					}
				}
			} 
		}

		to_return
	}

	fn add_face(to_return: &mut Vec<VertexNormal>, voxel: &[i64; 3], axis: usize, direction: i64) {
		let mut to_extend : [VertexNormal; 6] = [
			VertexNormal { position : Point([0., 0., 0.]), normal : V3::ZERO},
			VertexNormal { position : Point([0., 1., 0.]), normal : V3::ZERO},
			VertexNormal { position : Point([1., 0., 0.]), normal : V3::ZERO},
			VertexNormal { position : Point([0., 1., 0.]), normal : V3::ZERO},
			VertexNormal { position : Point([1., 0., 0.]), normal : V3::ZERO},
			VertexNormal { position : Point([1., 1., 0.]), normal : V3::ZERO},
		];



		let mut normal = V3::ZERO;
		normal.0[axis] = direction as f32;

		// Put correct z-coord
		if direction == 1 {
			for vertex in to_extend.iter_mut() {
				vertex.position.0[2] = 1.0;
			}
		}

		// Swap coordinate & voxel pos
		let voxel_pos = V3::new([voxel[0] as f32, voxel[1] as f32, voxel[2] as f32]);
		for vertex in to_extend.iter_mut() {
			vertex.position.0.swap(axis, 2);
			vertex.position += voxel_pos;
			vertex.normal = normal;
		}

		to_return.extend(to_extend);
	}
}
