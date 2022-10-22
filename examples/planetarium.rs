use std::collections::HashMap;
use std::path::Path;

use migl::buffer::BufferBld;
use migl::program::DrawMode;
use migl::utils::camera::CylinderCamera;
use migl::utils::load::ObjLoader;
use migl::math3d::M44;
use migl::math3d::V3;
use migl::program::ProgramBuilder;
use migl::shader::Fragment;
use migl::shader::Shader;
use migl::shader::Vertex;

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

	let window = video_subsystem.window("Planetarium", WIDTH, HEIGHT)
		.position_centered()
		.opengl()
		.build()
		.unwrap();


	let _gl_context = window.gl_create_context().unwrap();

	let gl = GLWrap::new(video_subsystem);

	gl.set_viewport(0, 0, WIDTH as i32, HEIGHT as i32);
	gl.set_clear_color(0.0, 0.0, 0.0, 1.0);
	gl.set_line_width(5.0);

	// Load program
	let character_program =
		ProgramBuilder::new(
			Shader::<Vertex>::from_file("resources/shaders/diffuse/vert.glsl").unwrap(),
			Shader::<Fragment>::from_file("resources/shaders/diffuse/frag.glsl").unwrap(),
		)
		.build()
		.unwrap();



	let character_data = 
		ObjLoader::new()
		.load(Path::new("resources/model/running_man.obj"))
		.expect("Can't load object")
		.into_vertex_normals();
	let character_buffer = BufferBld::array().data(&character_data).unwrap();

	character_program.bind("position", character_buffer.view(field!(vertex))).unwrap();
	character_program.bind("normal",   character_buffer.view(field!(normal))).unwrap();

	// Star Program
	let star_program =	
		ProgramBuilder::new(
			Shader::<Vertex>::from_file("resources/shaders/stars/vert.glsl").unwrap(), 
			Shader::<Fragment>::from_file("resources/shaders/stars/frag.glsl").unwrap(), 
		)
		.build()
		.unwrap();

	let star_positions =
		BufferBld::array()
		.data(&load_stars(Path::new("resources/stars.json")))
		.unwrap();
	star_program.bind("position", star_positions.direct_view()).unwrap();
	let vp_uniform = star_program.uniform::<M44>("view_projection").unwrap();

	// Creating uniform
	character_program.uniform("ambient_strength").unwrap().pass(&0.1);
	character_program.uniform("specular_strength").unwrap().pass(&0.5);
	character_program.uniform("diffuse_strength").unwrap().pass(&0.5);
	character_program.uniform("light_strength").unwrap().pass(&1.0);
	character_program.uniform("light_direction").unwrap().pass(&(V3::new([-1.0, -1.0, 1.0]).normalize()));
	let camera_pos_uniform = character_program.uniform("camera_pos").unwrap();



	let mv_uniform = character_program.uniform("model_view").unwrap();
	let p_uniform  = character_program.uniform("projection").unwrap();
	let model_matrix    = M44::id();
	let projection_matrix = M44::perspective_projection(0.1, 50., 90., (WIDTH as f32) / (HEIGHT as f32));


	let mut camera = CylinderCamera::new();


	let mut event_pump = sdl_context.event_pump().unwrap();

	'main: loop {

		for event in event_pump.poll_iter() {
			match event {
				Event::Quit {..} |
				Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
					break 'main
				},
				event => {
					camera.control(&event, 10.);
				},
			}
		}

		gl.clear();

		let view_matrix = camera.matrix();
		let mv_matrix = view_matrix.dot(&model_matrix);

		mv_uniform.pass(&mv_matrix);
		p_uniform.pass(&projection_matrix);
		camera_pos_uniform.pass(&camera.position());

		character_program.set_current();
		character_program.draw_buffer(DrawMode::Tris).unwrap();

		let vp_matrix = projection_matrix.dot(&view_matrix);
		vp_uniform.pass(&vp_matrix);
		star_program.draw_buffer(DrawMode::Points).unwrap();


		window.gl_swap_window();
	}
}

#[derive(Deserialize)]
struct Star {
	right_ascension : [f32; 3],
	declination :     [f32; 3],
	#[allow(unused)]
	magnitude: f32,
}

fn load_stars(filepath : &Path) -> Vec<V3> {
	let file = std::fs::File::open(filepath).unwrap();

	let stars_map : HashMap<String, Vec<Star>> = serde_json::from_reader(file).unwrap();

	let n_stars = stars_map.iter().map(|(_, arr)| arr.len()).sum();
	let mut to_return = Vec::with_capacity(n_stars);

	for stars in stars_map.into_values() {
		for Star { right_ascension, declination, .. /*magnitude*/ } in stars.into_iter() {
			let [hours, minutes, seconds] = right_ascension;
			let [degrees, arc_minutes, arc_seconds] = declination;
			let longitude = (hours + (minutes + (seconds / 60.)) / 60.) * (2. * std::f32::consts::PI  / 24.);
			let latitude  = (degrees + (arc_minutes + arc_seconds / 60.) / 60.) * (2. * std::f32::consts::PI / 360.);

			let vector = V3::new(
				[longitude.cos() * latitude.cos(), latitude.sin(), longitude.sin() * latitude.cos(),]
			);
			to_return.push(vector);	
		}
	}

	to_return
}


