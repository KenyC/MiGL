use std::path::Path;
use std::path::PathBuf;

use migl::attributes::GLType;
use migl::buffer::BufferBld;
use migl::frame::FrameBufferBuilder;
use migl::math3d::Point;
use migl::texture::TexFormat;
use migl::texture::Texture;
use migl::uniform::Uniform;
use migl::utils::camera::CylinderCamera;
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
	// -- INIT SDL AND OPENGL CONTEXT
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

	// -- LOAD PROGRAM
	let program =
		ProgramBuilder::new(
			Shader::<Vertex>::from_file("resources/shaders/cube/vert.glsl").unwrap(), 
			Shader::<Fragment>::from_file("resources/shaders/cube/frag.glsl").unwrap(), 
		)
		.build()
		.unwrap();
	let mvp_uniform : Uniform<M44> = program.uniform("mvp").unwrap();


	// 1st organization : (PPP) (CCC)
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


	// -- CREATE FRAMEBUFFERS
	let color_texture = Texture::allocate(WIDTH, HEIGHT, TexFormat::Rgb).unwrap();
	let depth_texture = Texture::allocate(WIDTH, HEIGHT, TexFormat::Depth).unwrap();
	let frame_buffer =
		FrameBufferBuilder::new()
		.attach_color(&color_texture)
		.attach_depth(&depth_texture)
		.build()
		.unwrap();

	gl.set_viewport(0, 0, WIDTH as i32, HEIGHT as i32);
	gl.set_clear_color(0.0, 0.0, 0.0, 1.0);
	// gl.set_line_width(5.0);


	let path = std::env::temp_dir().join(Path::new("test.jpg"));



	let mut camera = CylinderCamera::new();
	let projection_matrix = M44::perspective_projection(0.1, 50., 60., 1.);



	let mut event_pump = sdl_context.event_pump().unwrap();

	'main: loop {
		let mut mvp = projection_matrix.dot(&camera.matrix());
		mvp_uniform.pass(&mvp);

		for event in event_pump.poll_iter() {
			match event {
				Event::Quit {..} |
				Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
					break 'main
				},
				Event::KeyDown { keycode: Some(Keycode::I), .. } => {
					frame_buffer.make_current();
					gl.clear();

					program.draw_buffer(program::DrawMode::Tris).unwrap();

					color_texture
						.to_image(GLType::Ubyte)
						.unwrap()
						.save(&path)
						.unwrap()
					;
					println!("Saved file to {}", path.to_str().unwrap_or("non-UTF8 path"));

					gl.default_framebuffer().make_current();
				},
				event => {
					camera.control(&event, 10.);
				},
			}
		}

		gl.clear();

		program.draw_buffer(program::DrawMode::Tris).unwrap();


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