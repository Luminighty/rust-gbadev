use agb::display::object::{Object, OamManaged};

use crate::gfx;

pub struct Player<'oam> {
	object: Object<'oam>,
	x: i32,
	y: i32,
}

impl<'oam> Player<'oam> {
	pub fn new(oam: &'oam OamManaged, x: i32, y: i32) -> Self {
		let mut object = object_sprite!(oam, gfx::PLAYER);
		object.show();
		object.set_x(x as u16);
		object.set_y(y as u16);
		Self { object, x, y }
	}

	pub fn set_position(&mut self, x: i32, y: i32) {
		self.x = x;
		self.y = y;
		self.object.set_x(x as u16);
		self.object.set_y(y as u16);
	}

	pub fn move_player(&mut self, dx: i32, dy: i32) {
		self.set_position(self.x + dx, self.y + dy);
	}

	pub fn flip_h(&mut self, flip_h: bool) {
		self.object.set_hflip(flip_h);
	}
}