use std::collections::HashMap;
use std::path::Path;

use image::ImageFormat;
use migl::buffer::BufferBld;
use migl::program::DrawMode;
use migl::texture::TexFormat;
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

	let window = video_subsystem.window("Jug", WIDTH, HEIGHT)
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
	let mut jug_program =
		ProgramBuilder::new(
			Shader::<Vertex>::from_file("resources/shaders/jug/vert.glsl").unwrap(),
			Shader::<Fragment>::from_file("resources/shaders/jug/frag.glsl").unwrap(),
		)
		.build()
		.unwrap();



	let jug_data = 
		ObjLoader::new()
		.load(Path::new("resources/model/jug/jug.obj"))
		.expect("Can't load object")
		.into_vertex_normals();
	let jug_buffer = BufferBld::array().data(&jug_data).unwrap();

	jug_program.bind("position",   jug_buffer.view(field!(vertex))).unwrap();
	jug_program.bind("normal",     jug_buffer.view(field!(normal))).unwrap();
	jug_program.bind("tex_coords", jug_buffer.view(field!(tex_coords))).unwrap();

	// textures
	let file = std::io::BufReader::new(std::fs::File::open("resources/model/jug/textures/diffuse.jpg").unwrap());
	let format_img = ImageFormat::Jpeg;
	let image = image::load(file, format_img).unwrap();
	let diffuse_texture = texture::Texture::new(&image).unwrap();
	jug_program.texture("diffuse_texture", diffuse_texture).unwrap();

	let file = std::io::BufReader::new(std::fs::File::open("resources/model/jug/textures/roughness.jpg").unwrap());
	let image = image::load(file, format_img).unwrap();
	let rougness_texture = texture::Texture::new_stored_as(&image, TexFormat::Monochrome).unwrap();
	jug_program.texture("roughness_texture", rougness_texture).unwrap();


	// Creating uniform
	jug_program.uniform("ambient_strength").unwrap().pass(&0.3);
	jug_program.uniform("specular_strength").unwrap().pass(&0.3);
	jug_program.uniform("diffuse_strength").unwrap().pass(&1.0);
	jug_program.uniform("light_strength").unwrap().pass(&0.8);
	jug_program.uniform("light_direction").unwrap().pass(&(V3::new([1.0, -1.0, 1.0]).normalize()));
	let camera_pos_uniform = jug_program.uniform("camera_pos").unwrap();



	let mv_uniform = jug_program.uniform("model_view").unwrap();
	let p_uniform  = jug_program.uniform("projection").unwrap();
	let model_matrix  = M44::scaling(15.);
	let projection_matrix = M44::perspective_projection(0.1, 50., 60., (WIDTH as f32) / (HEIGHT as f32));


	let mut camera = CylinderCamera::new();
	camera.height_angle = 0.8;
	camera.radius = 8.;
	camera.angle = -75_f32.to_radians();


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

		jug_program.set_current();
		jug_program.draw_buffer(DrawMode::Tris).unwrap();



		window.gl_swap_window();
	}
}

