#![no_std]
#![no_main]

#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

use agb::{println, fixnum::Vector2D};

mod gfx;
#[macro_use]
mod macros;


#[agb::entry]
fn entry(mut gba: agb::Gba) -> ! {
	main(gba)
}



fn main(mut gba: agb::Gba) -> ! {
	let oam = gba.display.object.get_managed();

	let mut gold = object_sprite!(oam, gfx::GOLD);
	let mut iron = object_sprite!(oam, gfx::IRON);
	let mut ruby = object_sprite!(oam, gfx::RUBY);
	let mut emerald = object_sprite!(oam, gfx::EMERALD);

	gold.show().set_x(50).set_y(50);
	iron.show().set_x(70).set_y(50);
	ruby.show().set_x(90).set_y(50);
	emerald.show().set_x(110).set_y(50);

	oam.commit();
	loop {
		VBLANK!();
	}
}
