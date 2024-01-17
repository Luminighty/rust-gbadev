#![no_std]
#![no_main]

#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

use agb::{input::Button, println};
use ball::Ball;
use paddle::Paddle;

pub mod gfx;
mod ball;
mod paddle;
mod collision;

#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
	setup(gba);
}

const BALL_VELX: i32 = 1;
const BALL_VELY: i32 = 1;
const PADDLE_VELY: i32 = 2;

fn setup(mut gba: agb::Gba) -> ! {
	let oam = gba.display.object.get_managed();
	let mut input = agb::input::ButtonController::new();
	
	const BALL_X: i32 = 10;
	const BALL_Y: i32 = 10;

	let mut ball = ball::Ball::new(&oam, BALL_X, BALL_Y, BALL_VELX, BALL_VELY);
	let mut paddle = paddle::Paddle::new(&oam, 10, agb::display::HEIGHT / 2 - 8);
	let mut sad = oam.object_sprite(gfx::SAD.sprite(0));
	sad.set_x(agb::display::WIDTH as u16 / 2 - 8).set_y(agb::display::HEIGHT as u16 / 2 - 8);

	loop {
		ball.x = BALL_X;
		ball.y = BALL_X;
		ball.vx = BALL_VELX;
		ball.vy = BALL_VELX;
		loop {
			let y_tri = input.y_tri() as i32;
			
			if ball.vx < 0 && collision::ball_intersects(&ball, &paddle) {
				ball.vx *= -1;
				ball.vx += 1;
				ball.vy += ball.vy.signum();
			}
			
			paddle.step(y_tri * PADDLE_VELY);
			ball.step();
			
			paddle.draw();
			ball.draw();
	
			agb::display::busy_wait_for_vblank();
			oam.commit();
			input.update();

			if !ball.is_on_screen() {
				sad.show();
				break;
			}
		}
	
		loop {
			agb::display::busy_wait_for_vblank();
			oam.commit();
			input.update();	

			if input.is_pressed(Button::A) {
				break;
			}
		}
		sad.hide();
	}
}
