use migl::buffer::BufferBld;
use migl::math3d::Point;
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

	let mut gl = GLWrap::new(video_subsystem);

	gl.set_viewport(0, 0, WIDTH as i32, HEIGHT as i32);
	gl.set_clear_color(0.0, 0.0, 0.0, 1.0);
	gl.set_line_width(5.0);

	// Load program
	let program =
		ProgramBuilder::new(
			Shader::<Vertex>::from_file("resources/shaders/cube/vert.glsl").unwrap(), 
			Shader::<Fragment>::from_file("resources/shaders/cube/frag.glsl").unwrap(), 
		)
		.attributes(&["position", "color"])
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


	// 2nd organization : (PPPCCC)
	let mut data = Vec::with_capacity(VERTICES.len() + COLORS.len());
	data.extend_from_slice(&VERTICES);
	data.extend_from_slice(&COLORS);
	let buffer_vertices_colors_juxtaposed =
		BufferBld::array()
		.data(&data)
		.unwrap()
	;

	let n_vertices = VERTICES.len();
	let program2 = program.duplicate().unwrap();
	program2.bind("position", buffer_vertices_colors_juxtaposed.view_range(0  .. n_vertices)).unwrap();
	program2.bind("color",    buffer_vertices_colors_juxtaposed.view_range(n_vertices .. 2 * n_vertices)).unwrap();

	// 2nd organization : (PCPCPC)
	struct ColoredVertex {
		position: V3,
		color:    V3,
	}
	let data = 
		VERTICES.iter()
		.zip(COLORS.iter())
		.map(|(position, color)|
			ColoredVertex{ position : position.clone() , color : color.clone() }
		)
		.collect::<Vec<_>>();
	let buffer_vertices_colors_interspersed =
		BufferBld::array()
		.data(&data)
		.unwrap()
	;

	let program3 = program.duplicate().unwrap();
	program3.bind("position", buffer_vertices_colors_interspersed.view(field!(position))).unwrap();
	program3.bind("color",    buffer_vertices_colors_interspersed.view(field!(color))).unwrap();


	let mut camera = CylinderCamera::new();
	let projection_matrix = M44::perspective_projection(0.1, 50., 90., 1.);


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

		let mut mvp = projection_matrix.dot(&camera.matrix());
		mvp_uniform.pass(&mvp);
		program.draw_buffer(program::DrawMode::Tris).unwrap();

		mvp = mvp.dot(&M44::translation(V3::new([1., 2., 3.])));
		mvp_uniform.pass(&mvp);
		program2.draw_buffer(program::DrawMode::Tris).unwrap();


		mvp = mvp.dot(&M44::translation(V3::new([-1.5, 0., -1.])));
		mvp_uniform.pass(&mvp);
		program3.draw_buffer(program::DrawMode::Tris).unwrap();


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