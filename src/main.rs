#![no_std]
#![no_main]

#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

use agb::input::Tri;


#[macro_use]
mod macros;
mod gfx;

mod player;

#[agb::entry]
fn entry(mut gba: agb::Gba) -> ! {
	main(gba)
}

fn main(mut gba: agb::Gba) -> ! {
	let oam = gba.display.object.get_managed();
	let mut input = agb::input::ButtonController::new();

	let mut player = player::Player::new(&oam, 50, 50);
	loop {
		let tri_x = input.x_tri();
		let tri_y = input.y_tri();

		player.move_player(tri_x as i32, tri_y as i32);
		match tri_x {
			Tri::Positive => player.flip_h(true),
			Tri::Negative => player.flip_h(false),
			_ => {},
		}

		oam.commit();
		input.update();
		VBLANK!();
	}
}
