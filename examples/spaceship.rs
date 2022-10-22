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

	let window = video_subsystem.window("Spaceship", WIDTH, HEIGHT)
		.position_centered()
		.opengl()
		.build()
		.unwrap();


	let _gl_context = window.gl_create_context().unwrap();

	// -- Init GL
	let gl = GLWrap::new(video_subsystem);
	gl.set_viewport(0, 0, WIDTH as i32, HEIGHT as i32);
	gl.set_clear_color(0.0, 0.0, 0.0, 1.0);
	gl.set_line_width(5.0);



	// -- Load spaceship model & shader
	let program =
		ProgramBuilder::new(
			Shader::<Vertex>::from_file("resources/shaders/diffuse/vert.glsl").unwrap(),
			Shader::<Fragment>::from_file("resources/shaders/diffuse/frag.glsl").unwrap(),
		)
		.build()
		.unwrap();

	let spaceship_data = 
		ObjLoader::new()
		// .load(Path::new("resources/model/starship.obj"))
		.load(Path::new("resources/model/sphere.obj"))
		// .load(Path::new("resources/model/jug/jug.obj"))
		.expect("Can't load object")
		.into_vertex_normals();
	let spaceship_buffer = BufferBld::array().data(&spaceship_data).unwrap();

	program.bind("position", spaceship_buffer.view(field!(vertex))).unwrap();
	program.bind("normal",   spaceship_buffer.view(field!(normal))).unwrap();

	program.uniform("ambient_strength").unwrap().pass(&0.1);
	program.uniform("specular_strength").unwrap().pass(&0.2);
	program.uniform("diffuse_strength").unwrap().pass(&0.6);
	program.uniform("light_strength").unwrap().pass(&0.5);
	let light_dir_uniform  = program.uniform("light_direction").unwrap();
	let camera_pos_uniform = program.uniform("camera_pos").unwrap();



	// -- Create light rays
	let ray_program =
		ProgramBuilder::new(
			Shader::from_file("resources/shaders/rays/vert.glsl").unwrap(),
			Shader::from_file("resources/shaders/rays/frag.glsl").unwrap(),
		)
		.geom_shader(Shader::from_file("resources/shaders/rays/geom.glsl").unwrap())
		.build()
		.unwrap();
	let ray_buffer =
		BufferBld::array()
		.data(&[V3::ZERO])
		.unwrap();
	ray_program.bind("pos", ray_buffer.direct_view()).unwrap();
	let raydir_uniform = ray_program.uniform("ray_dir").unwrap();
	let ray_vp_uniform = ray_program.uniform("view_projection").unwrap();


	// -- Create matrices values
	let mv_uniform = program.uniform("model_view").unwrap();
	let p_uniform  = program.uniform("projection").unwrap();
	// let model_matrix    = M44::rotation(&V3::E_X, -90_f32.to_radians()).dot(&M44::scaling(1.0));
	let model_matrix    = M44::scaling(5.);
	let model_rotation  = model_matrix.extract_rotation();
	let projection_matrix = M44::perspective_projection(0.1, 50., 60., 1.);


	// -- Axes & Camera
	// let axes_builder = AxesBuilder::new().unwrap();
	// let mut axes = axes_builder.axes();
	// axes.set_pos(V3::new([0.0, 2.0, 0.0]));
	let mut camera = CylinderCamera::new();




	let timer = sdl_context.timer().unwrap();
	let mut old_time = timer.ticks();
	let mut event_pump = sdl_context.event_pump().unwrap();

	'main: loop {
		let current_time = timer.ticks();
		let t  = current_time as f32;
		let _dt = (current_time - old_time) as f32;
		old_time = current_time;

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
		let vp_matrix = projection_matrix.dot(&view_matrix);
		// let p_matrix = ;

		// -- Draw Spaceship
		let light_time = 0.001 * t; 
		const HEIGHT : f32 = 0.8660254037844386;
		let width = (1. - HEIGHT).powf(0.5);
		// LIGHT that comes from above has negative Y coordinate
		let light_dir = model_rotation.apply(&(V3::new([light_time.cos() * width, -HEIGHT, light_time.sin() * width]))).normalize();
		// let light_dir = model_rotation.apply(&(V3::new([-1.0, -0.0, -1.0]))).normalize();
		light_dir_uniform.pass(&light_dir);
		mv_uniform.pass(&mv_matrix);
		p_uniform.pass(&projection_matrix);
		camera_pos_uniform.pass(&camera.position());

		program.set_current();
		program.draw_buffer(program::DrawMode::Tris).unwrap();

		// axes.draw(&vp_matrix).unwrap();

		// -- Draw rays
		raydir_uniform.pass(&light_dir);
		ray_vp_uniform.pass(&vp_matrix);

		ray_program.draw_buffer(program::DrawMode::Points).unwrap();

		window.gl_swap_window();
	}
}
