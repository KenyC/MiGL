use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use std::path::Path;

use migl::buffer::BufferBld;
use migl::math3d::Normed;
use migl::math3d::V2;
use migl::program::DrawMode;
use migl::utils::camera::CylinderCamera;
use migl::utils::camera::TurntableCamera;
use migl::utils::load::ObjLoader;
use migl::math3d::M44;
use migl::math3d::V3;
use migl::program::ProgramBuilder;
use migl::shader::Fragment;
use migl::shader::Shader;
use migl::shader::Vertex;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
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
	gl.enable_program_point_size();
	gl.set_line_width(5.0);





	// Star Program
	let star_program =	
		ProgramBuilder::new(
			Shader::<Vertex>::from_file("resources/shaders/stars/vert.glsl").unwrap(), 
			Shader::<Fragment>::from_file("resources/shaders/stars/frag.glsl").unwrap(), 
		)
		.build()
		.unwrap();

	let StarMap { constellations, star_data, constellation_pos } = load_stars(Path::new("resources/stars.json"));
	let star_positions =
		BufferBld::array()
		.data(&star_data)
		.unwrap();
	star_program.bind("position",      star_positions.view(field!(direction))).unwrap();
	star_program.bind("constellation", star_positions.view(field!(constellation))).unwrap();
	star_program.bind("magnitude",     star_positions.view(field!(magnitude))).unwrap();
	let max_magnitude = star_data.iter().map(|s| s.magnitude).max_by(f32::total_cmp).unwrap();
	star_program.uniform("max_magnitude").unwrap().pass(&max_magnitude);
	let min_magnitude = star_data.iter().map(|s| s.magnitude).min_by(f32::total_cmp).unwrap();
	star_program.uniform("min_magnitude").unwrap().pass(&min_magnitude);
	let constellation_index_uniform = star_program.uniform::<gl::types::GLuint>("current_constellation").unwrap();
	let mut current_constellation_index : gl::types::GLuint = 0;
	constellation_index_uniform.pass(&0);
	let vp_uniform = star_program.uniform::<M44>("view_projection").unwrap();




	let model_matrix    = M44::id();
	let projection_matrix = M44::perspective_projection(0.1, 50., 90., (WIDTH as f32) / (HEIGHT as f32));


	let mut camera = TurntableCamera::new(V3::ZERO, V3::new([3.0, 3.0, 0.0]), V3::E_Y);


	let mut event_pump = sdl_context.event_pump().unwrap();

	'main: loop {

		let mouse_state = event_pump.mouse_state();
		let current_pos = V2::new([mouse_state.x() as f32, mouse_state.y() as f32]);
		for event in event_pump.poll_iter() {
			match event {
				Event::Quit {..} |
				Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
					break 'main
				},
				Event::KeyDown { keycode: Some(Keycode::C), .. } => {
					constellation_index_uniform.pass(&(constellations.len() as gl::types::GLuint));
				},
				Event::KeyDown { keycode, .. } => {
					let changed = match keycode {
						Some(Keycode::P) => {current_constellation_index.checked_sub(1).is_some()}
						Some(Keycode::N) => {current_constellation_index += 1; true}
						_ => false
					};
					if changed {
						current_constellation_index += 1;
						let i = current_constellation_index as usize;
						println!("current constellation: {}", constellations[i]);
						let pos = constellation_pos[i];
						camera = TurntableCamera::new(V3::ZERO, pos.scale(- camera.pos_camera().norm()), V3::E_Y);
						constellation_index_uniform.pass(&current_constellation_index);
					}
				},
				Event::MouseButtonDown { mouse_btn : MouseButton::Middle, x, y, .. } => {
					camera.start_drag(current_pos);
				},
				Event::MouseButtonUp { mouse_btn : MouseButton::Middle, x, y, .. } => {
					camera.end_drag(current_pos);
				},
				Event::MouseWheel { y, .. }=> {
					let factor =
						if y.is_positive() { 1.0  }
						else               { -1.0 };
					camera.zoom(factor);
				},
				_ => {}
			}
		}
		camera.update_pos(current_pos);

		gl.clear();

		let view_matrix = camera.matrix();
		let mv_matrix = view_matrix.dot(&model_matrix);


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
	magnitude:      f32,
}

struct StarVertex {
	direction     : V3,
	magnitude     : f32,
	constellation : gl::types::GLuint,
}

struct StarMap {
	constellations    : Vec<String>,
	constellation_pos : Vec<V3>,
	star_data         : Vec<StarVertex>,
}

fn load_stars(filepath : &Path) -> StarMap {
	let file = std::fs::File::open(filepath).unwrap();

	let stars_map : HashMap<String, Vec<Star>> = serde_json::from_reader(file).unwrap();

	let n_stars = stars_map.iter().map(|(_, arr)| arr.len()).sum();
	let mut star_data         = Vec::with_capacity(n_stars);
	let mut constellation     = Vec::with_capacity(stars_map.len());
	let mut constellation_pos = Vec::with_capacity(stars_map.len());

	for (i, (constellation_name, stars)) in stars_map.into_iter().enumerate() {
		constellation.push(constellation_name);
		let mut average_pos = V3::ZERO;

		for Star { right_ascension, declination, magnitude } in stars.into_iter() {
			let [hours, minutes, seconds] = right_ascension;
			let [degrees, arc_minutes, arc_seconds] = declination;
			let longitude = (hours + (minutes + (seconds / 60.)) / 60.) * (2. * std::f32::consts::PI  / 24.);
			let latitude  = (degrees + (arc_minutes + arc_seconds / 60.) / 60.).to_radians();

			let direction = V3::new(
				[longitude.cos() * latitude.cos(), latitude.sin(), longitude.sin() * latitude.cos(),]
			);

			average_pos += direction;
			star_data.push(StarVertex { 
				direction, 
				magnitude, 
				constellation : i.try_into().unwrap(),
			});	
		}
		let average_pos = average_pos.normalize();
		constellation_pos.push(average_pos);
	}
	StarMap { constellations: constellation, star_data, constellation_pos }
}


