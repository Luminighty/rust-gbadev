macro_rules! object_sprite {
	($oam:expr, $tag:expr) => {
		$oam.object_sprite($tag.sprite(0))
	};
	($oam:expr, $tag:expr, $sprite:literal) => {
		$oam.object_sprite($tag.sprite($sprite))
	};
}

macro_rules! VBLANK {
	() => { agb::display::busy_wait_for_vblank(); };
}