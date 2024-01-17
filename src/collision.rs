use agb::println;

use crate::{ball::Ball, paddle::Paddle};

pub fn ball_intersects(ball: &Ball, paddle: &Paddle) -> bool {
	intersects(
		ball.x, ball.y, 16, 16, 
		paddle.x, paddle.y - 16, 16, 16*3
	)
}


pub fn intersects(
	ax: i32, ay: i32, aw: i32, ah: i32, 
	bx: i32, by: i32, bw: i32, bh: i32
) -> bool {
	ax < bx + bw && ax + aw > bx && ay < by + bh && ay + ah > by
}

#[cfg(test)]
mod test {
	use agb::Gba;
	use super::*;

	#[test_case]
	fn test_intersects(_gba: &mut Gba) {
		assert_eq!(1, 1);
	}
}