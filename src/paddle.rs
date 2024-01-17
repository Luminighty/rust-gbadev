use agb::display::object::{Object, OamManaged};

use crate::gfx;

const SPRITEH: i32 = 16;

pub struct Paddle<'oam> {
	pub top: Object<'oam>,
	pub mid: Object<'oam>,
	pub bot: Object<'oam>,
	pub x: i32,
	pub y: i32,
}

impl<'oam> Paddle<'oam> {

	pub fn new(oam: &'oam OamManaged, x: i32, y: i32) -> Self {
		let mut top = oam.object_sprite(gfx::PADDLE_END.sprite(0));
		let mut mid = oam.object_sprite(gfx::PADDLE_MID.sprite(0));
		let mut bot = oam.object_sprite(gfx::PADDLE_END.sprite(0));
		bot.show().set_vflip(true);
		top.show();
		mid.show();
		Self { top, mid, bot, x, y }
	}

	pub fn step(&mut self, dy: i32) {
		self.y = (self.y + dy).clamp(SPRITEH, agb::display::HEIGHT - SPRITEH * 2);
	}

	pub fn draw(&mut self) {
		self.top.set_x(self.x as u16).set_y((self.y - SPRITEH) as u16);
		self.mid.set_x(self.x as u16).set_y(self.y as u16);
		self.bot.set_x(self.x as u16).set_y((self.y + SPRITEH) as u16);
	}
}
