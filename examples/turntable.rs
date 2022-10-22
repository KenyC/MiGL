use std::collections::HashMap;
use std::path::Path;


use migl::buffer::BufferBld;
use migl::math3d::Point;
use migl::math3d::V2;
use migl::uniform::Uniform;
use migl::utils::axes::AxesBuilder;
use migl::math3d::M44;
use migl::math3d::V3;
use migl::program::ProgramBuilder;
use migl::shader::Fragment;
use migl::shader::Shader;
use migl::shader::Vertex;

use migl::utils::camera::TurntableCamera;
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

	let window = video_subsystem.window("Stick people", WIDTH, HEIGHT)
		.position_centered()
		.opengl()
		.build()
		.unwrap();


	let _gl_context = window.gl_create_context().unwrap();

	let gl = GLWrap::new(video_subsystem);

	gl.set_viewport(0, 0, WIDTH as i32, HEIGHT as i32);
	gl.set_clear_color(0.0, 0.0, 0.0, 1.0);
	gl.set_line_width(5.0);

	// -- CUBE PROGRAM
	let program =
		ProgramBuilder::new(
			Shader::<Vertex>::from_file("resources/shaders/cube/vert.glsl").unwrap(), 
			Shader::<Fragment>::from_file("resources/shaders/cube/frag.glsl").unwrap(), 
		)
		.build()
		.unwrap();
	let mvp_uniform : Uniform<M44> = program.uniform("mvp").unwrap();


	let buffer_vertices =
		BufferBld::array()
		.data(&VERTICES)
		.unwrap();

	let buffer_colors =
		BufferBld::array()
		.data(&COLORS)
		.unwrap();


	program.bind("position", buffer_vertices.direct_view()).unwrap();
	program.bind("color",    buffer_colors.direct_view()).unwrap();



	// -- STAR PROGRAM
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


	// -- SET CAMERA
	let center = V3::E_X.scale(5.0);
	let mut camera = TurntableCamera::new(center, V3::new([3.0, 3.0, 0.0]), V3::E_Y);
	let projection_matrix = M44::perspective_projection(0.1, 50., 60., 1.);

	// -- AXES
	let axes_builder = AxesBuilder::new().unwrap();
	let mut axes = axes_builder.axes();
	axes.set_pos(center);

	let mut event_pump = sdl_context.event_pump().unwrap();
	// let timer = Instant::now();



	'main: loop {
		// let mut elapsed = timer.elapsed().as_secs_f32();

		let mouse_state = event_pump.mouse_state();
		let current_pos = V2::new([mouse_state.x() as f32, mouse_state.y() as f32]);
		for event in event_pump.poll_iter() {
			match event {
				Event::Quit {..} |
				Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
					break 'main
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

		let vp = projection_matrix.dot(camera.matrix());
		mvp_uniform.pass(&vp);
		program.draw_buffer(program::DrawMode::Tris).unwrap();

		vp_uniform.pass(&vp);
		star_program.draw_buffer(program::DrawMode::Points).unwrap();

		axes.draw(&vp).unwrap();

		window.gl_swap_window();
	}
}






const VERTICES : [V3; 6 * 3 * 2] = [
	Point::<3>([1.0, 1.0, 1.0]),
	Point::<3>([1.0, -1.0, 1.0]),
	Point::<3>([1.0, -1.0, -1.0]),

	Point::<3>([1.0, 1.0, 1.0]),
	Point::<3>([1.0, 1.0, -1.0]),
	Point::<3>([1.0, -1.0, -1.0]),

	Point::<3>([-1.0, 1.0, 1.0]),
	Point::<3>([-1.0, -1.0, 1.0]),
	Point::<3>([-1.0, -1.0, -1.0]),

	Point::<3>([-1.0, 1.0, 1.0]),
	Point::<3>([-1.0, 1.0, -1.0]),
	Point::<3>([-1.0, -1.0, -1.0]),

	Point::<3>([1.0, 1.0, 1.0]),
	Point::<3>([-1.0, 1.0, 1.0]),
	Point::<3>([-1.0, 1.0, -1.0]),

	Point::<3>([1.0, 1.0, 1.0]),
	Point::<3>([1.0, 1.0, -1.0]),
	Point::<3>([-1.0, 1.0, -1.0]),

	Point::<3>([1.0, 1.0, -1.0]),
	Point::<3>([-1.0, 1.0, -1.0]),
	Point::<3>([-1.0, -1.0, -1.0]),

	Point::<3>([1.0, 1.0, -1.0]),
	Point::<3>([1.0, -1.0, -1.0]),
	Point::<3>([-1.0, -1.0, -1.0]),

	Point::<3>([1.0, 1.0, 1.0]),
	Point::<3>([-1.0, 1.0, 1.0]),
	Point::<3>([-1.0, -1.0, 1.0]),

	Point::<3>([1.0, 1.0, 1.0]),
	Point::<3>([1.0, -1.0, 1.0]),
	Point::<3>([-1.0, -1.0, 1.0]),

	Point::<3>([1.0, -1.0, 1.0]),
	Point::<3>([-1.0, -1.0, 1.0]),
	Point::<3>([-1.0, -1.0, -1.0]),

	Point::<3>([1.0, -1.0, 1.0]),
	Point::<3>([1.0, -1.0, -1.0]),
	Point::<3>([-1.0, -1.0, -1.0]),

];


const COLORS : [V3; 6 * 3 * 2] = [
	Point::<3>([1.0, 0.0, 0.0]),
	Point::<3>([1.0, 0.0, 0.0]),
	Point::<3>([1.0, 0.0, 0.0]),

	Point::<3>([1.0, 0.0, 0.0]),
	Point::<3>([1.0, 0.0, 0.0]),
	Point::<3>([1.0, 0.0, 0.0]),

	Point::<3>([0.0, 1.0, 1.0]),
	Point::<3>([0.0, 1.0, 1.0]),
	Point::<3>([0.0, 1.0, 1.0]),

	Point::<3>([0.0, 1.0, 1.0]),
	Point::<3>([0.0, 1.0, 1.0]),
	Point::<3>([0.0, 1.0, 1.0]),

	Point::<3>([0.0, 1.0, 0.0]),
	Point::<3>([0.0, 1.0, 0.0]),
	Point::<3>([0.0, 1.0, 0.0]),

	Point::<3>([0.0, 1.0, 0.0]),
	Point::<3>([0.0, 1.0, 0.0]),
	Point::<3>([0.0, 1.0, 0.0]),

	Point::<3>([1.0, 1.0, 0.0]),
	Point::<3>([1.0, 1.0, 0.0]),
	Point::<3>([1.0, 1.0, 0.0]),

	Point::<3>([1.0, 1.0, 0.0]),
	Point::<3>([1.0, 1.0, 0.0]),
	Point::<3>([1.0, 1.0, 0.0]),

	Point::<3>([0.0, 0.0, 1.0]),
	Point::<3>([0.0, 0.0, 1.0]),
	Point::<3>([0.0, 0.0, 1.0]),

	Point::<3>([0.0, 0.0, 1.0]),
	Point::<3>([0.0, 0.0, 1.0]),
	Point::<3>([0.0, 0.0, 1.0]),

	Point::<3>([1.0, 0.0, 1.0]),
	Point::<3>([1.0, 0.0, 1.0]),
	Point::<3>([1.0, 0.0, 1.0]),

	Point::<3>([1.0, 0.0, 1.0]),
	Point::<3>([1.0, 0.0, 1.0]),
	Point::<3>([1.0, 0.0, 1.0]),

];


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


