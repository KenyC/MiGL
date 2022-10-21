use migl::utils::axes::AxesBuilder;
use migl::utils::camera::CylinderCamera;
use migl::math3d::M44;
use migl::math3d::V3;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::GLProfile;

use migl::*;

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

	// Load program
	let projection_matrix = M44::perspective_projection(0.1, 50., 60., 1.);
	let axes_builder = AxesBuilder::new().unwrap();
	let axes1 = axes_builder.axes();
	let mut axes2 = axes_builder.axes();
	axes2.set_pos(V3::new([1.0, 0.0, 0.0]));


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
		let view_projection_matrix = projection_matrix.dot(&view_matrix);

		axes1.draw(&view_projection_matrix).unwrap();
		axes2.draw(&view_projection_matrix).unwrap();

		window.gl_swap_window();
	}
}
