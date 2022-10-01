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