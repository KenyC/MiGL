use std::path;
use std::path::Path;

use migl::buffer::BufferBld;
use migl::utils::camera::CylinderCamera;
use migl::utils::load::ObjLoader;
use migl::math3d::M44;
use migl::math3d::V3;
use migl::program::ProgramBuilder;
use migl::shader::Fragment;
use migl::shader::Shader;
use migl::shader::Vertex;

use sdl2::event::Event;
use sdl2::keyboard::KeyboardState;
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
	let program =
		ProgramBuilder::new(
			Shader::<Vertex>::from_file("resources/shaders/diffuse/vert.glsl").unwrap(),
			Shader::<Fragment>::from_file("resources/shaders/diffuse/frag.glsl").unwrap(),
		)
		.attributes(&["position", "normal"])
		.build()
		.unwrap();

	let spaceship_data = 
		ObjLoader::new()
		.load(Path::new("resources/model/starship.obj"))
		.expect("Can't load object")
		.into_vertex_normals();
	let spaceship_buffer = BufferBld::array().data(&spaceship_data).unwrap();

	program.bind("position", spaceship_buffer.view(field!(vertex))).unwrap();
	program.bind("normal",   spaceship_buffer.view(field!(normal))).unwrap();

	program.uniform("min_illumination").unwrap().pass(&0.2);
	program.uniform("max_illumination").unwrap().pass(&0.5);
	let light_dir = - V3::new([1.0; 3]);
	program.uniform("light_direction").unwrap().pass(&light_dir);



	let mvp_uniform = program.uniform("model_view_projection").unwrap();
	let model_matrix = M44::rotation(&V3::E_X, -90_f32.to_radians()).dot(&M44::scaling(1.0));
	let projection_matrix = M44::perspective_projection(0.1, 50., 90., 1.);


	let mut camera = CylinderCamera::new();

	let mut event_pump = sdl_context.event_pump().unwrap();

	'main: loop {
		'event : for event in event_pump.poll_iter() {
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

		let view_projection = camera.matrix();
		let mvp = projection_matrix.dot(&view_projection).dot(&model_matrix);

		mvp_uniform.pass(&mvp);
		program.set_current();
		program.draw_buffer(&spaceship_buffer, program::DrawMode::Tris);

		window.gl_swap_window();
	}
}
