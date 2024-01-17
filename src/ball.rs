use agb::display::object::{Object, OamManaged};
use crate::gfx;

pub struct Ball<'oam> {
	pub spr: Object<'oam>,
	pub x: i32,
	pub y: i32,
	pub vx: i32,
	pub vy: i32,
}

impl<'oam> Ball<'oam> {
	pub fn new(oam: &'oam OamManaged, x: i32, y: i32, vx: i32, vy: i32) -> Self {
		let mut spr = oam.object_sprite(gfx::BALL.sprite(0));
		spr.show();
		Self { spr, x, y, vx, vy }
	}

	pub fn step(&mut self) {
		const MAX_X: i32 = agb::display::WIDTH - 16;
		const MAX_Y: i32 = agb::display::HEIGHT - 16;
		self.x = (self.x + self.vx).clamp(i32::MIN, MAX_X);
		self.y = (self.y + self.vy).clamp(0, MAX_Y);

		if self.vx > 0 && self.x >= MAX_X {
			self.vx *= -1;
		}
		if self.vy > 0 && self.y >= MAX_Y {
			self.vy *= -1;
		}
		if self.vy < 0 && self.y <= 0 {
			self.vy *= -1;
		}
	}

	pub fn draw(&mut self) {
		self.spr.set_x(self.x as u16).set_y(self.y as u16);
	}

	pub fn is_on_screen(&self) -> bool {
		self.x > -18
	}
}