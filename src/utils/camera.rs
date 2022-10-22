use crate::math3d::*;



#[derive(Debug)]
pub struct CylinderCamera {
	pub center: V3,
	pub angle:  f32, // 0° is aligned with X axis
	pub height_angle: f32,
	pub radius: f32,
	pub speed:  f32,
}

impl CylinderCamera {
	pub const DEFAULT_SPEED : f32 = 0.003;

	pub fn new() -> Self {
		Self {
			center : V3::ZERO,
			angle:   0.,
			height_angle: 2.,
			radius: 3.,
			speed: Self::DEFAULT_SPEED,
		}
	}

	pub fn matrix(&self) -> M44 {
		let pos_camera = self.position();
		M44::look_at(
			&pos_camera,
			&self.center,
			&V3::E_Y,
		)
	}

	pub fn position(&self) -> V3 {
		V3::new([
			self.radius * self.angle.cos(),
			self.height_angle * self.radius,
			self.radius * self.angle.sin(),
		])
	}


	#[cfg(feature = "sdl2")]
	pub fn control(&mut self, event : &sdl2::event::Event, dt : f32) {
		use sdl2::keyboard::Keycode;

		if let sdl2::event::Event::KeyDown {keycode : Some(keycode), .. } = event {
			match keycode {
				Keycode::Left  => self.rotate(dt, true),
				Keycode::Right => self.rotate(dt, false),
				Keycode::Up    => self.zoom(dt, true),
				Keycode::Down  => self.zoom(dt, false),
				Keycode::Z     => self.rise(dt, true),
				Keycode::S     => self.rise(dt, false),
				_ => (),
			}
		}
	}


	pub fn rotate(&mut self, dt : f32, clockwise: bool) {
		if clockwise {
			self.angle  -= self.speed * dt;
		}
		else {
			self.angle  += self.speed * dt;	
		}
	}

	pub fn zoom(&mut self, dt : f32, zoom_in: bool) {
		if zoom_in {
			self.radius /= 1. + self.speed * dt;
		}
		else {
			self.radius *= 1. + self.speed * dt;
		}
	}

	pub fn rise(&mut self, dt : f32, up : bool) {
		if up {
			self.height_angle += self.speed * dt;
		}
		else {
			self.height_angle -= self.speed * dt;
		}
	}


}

pub struct TurntableCamera {
	view_matrix : M44,
	orbit_speed : f32,
	zoom_speed  : f32,
	// guarantee: center is in the direction of the camera
	// view_matrix * vec4(center, 1) = vec4(0, 0, -dist_to_center, _) 
	center : V3,    
	dist_to_center : f32,
	start_drag : Option<InitialDrag>,
}

struct InitialDrag {
	view_matrix : M44,
	screen_pos  : V2,
}

impl TurntableCamera {
	const DEFAULT_ORBIT_SPEED : f32 = 0.01;
	const DEFAULT_ZOOM_SPEED  : f32 = 0.1;

    pub fn new(center: V3, pos_camera : V3, up : V3) -> Self { 
    	let view_matrix = M44::look_at(&pos_camera, &center, &up);
    	let dist_to_center = (pos_camera - center).norm();
    	let orbit_speed = Self::DEFAULT_ORBIT_SPEED;
    	let zoom_speed  = Self::DEFAULT_ZOOM_SPEED;
    	Self { view_matrix, orbit_speed, center, start_drag : None, zoom_speed, dist_to_center } 
    }

    pub fn pos_camera(&self) -> V3 {
    	let view_rotation = self.view_matrix.extract_rotation();
    	let view_translation = - self.view_matrix.extract_translation();
    	view_rotation.transpose().apply(&view_translation)
    }

    pub fn zoom(&mut self, factor : f32) {
    	// TODO : how should zooming and orbiting interact ?
    	// For now, let's forbid such interactions from happening
    	if self.start_drag.is_none() {
    		let dist_to_center = self.view_matrix.apply_homo(&self.center).0[2];
    		let scaled_factor  = self.zoom_speed * factor;
    		let exp_factor = scaled_factor.exp2();
    		self.view_matrix.0[2][3] +=  (1.0 - exp_factor) * dist_to_center;
    		self.dist_to_center *= exp_factor; 
    	}
    }


    pub fn matrix(&self) -> &M44 {
    	&self.view_matrix
    }

	pub fn start_drag(&mut self, start_pos : V2) {
		self.start_drag = Some(InitialDrag {
			view_matrix: self.view_matrix.clone(),
			screen_pos:  start_pos,
		});
	}

	pub fn end_drag(&mut self, end_pos : V2) {
		if let Some(initial_drag) = self.start_drag.take() {
			self.view_matrix = self.update_camera(&initial_drag, end_pos);
		}
	}

	pub fn update_pos(&mut self, current_pos: V2) {
		if let Some(initial_drag) = &self.start_drag {
			self.view_matrix = self.update_camera(initial_drag, current_pos);
		}
	}

	fn update_camera(&self, initial_drag : &InitialDrag, current_pos: V2) -> M44 {
		let InitialDrag { view_matrix: matrix, screen_pos } = initial_drag;
		let mut pos_diff = current_pos - screen_pos.clone();
		pos_diff.0[1] = - pos_diff.0[1];

		/*
		Rotation works as follows:
		 - we take v to be a vector in the plane spanned by the (x, y) camera axes, whose coordinates are proportional to mouse movement coordinates
		 - we consider the normal to the plane that contains the center, the camera position and v
		 - we'll then simply rotate the camera using the normal as axis, taking the center as center.

		*/

		let view_translation  = matrix.extract_translation();
		let view_rotation     = matrix.extract_rotation();
		// let view_rotation_inv = view_rotation.transpose();

		let (Point([mut x, mut y]), norm) = pos_diff.unit_norm();
		// Weid stuff happen at small values
		if norm < 1e-5 {
			x = 1.;
			y = 0.;
		}
		let axis_rotation = V3::new(view_rotation.0[0]).scale(-y) + V3::new(view_rotation.0[1]).scale(x);
		let angle = norm * self.orbit_speed;

		let rotation_matrix = M33::rotation(&axis_rotation, angle);

		let new_view_rotation    = view_rotation.dot(&rotation_matrix);
		let new_view_translation = view_translation + (view_rotation - new_view_rotation).apply(&self.center);

		M44::from_rotation_translation(&new_view_rotation, &new_view_translation)
	}
}

