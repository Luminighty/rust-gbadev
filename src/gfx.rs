use agb::{include_aseprite, display::object::{Graphics, Tag}};

pub const GRAPHICS: &Graphics = include_aseprite!(
	"gfx/player.aseprite",
	"gfx/tiles.aseprite",
	"gfx/bg.aseprite"
);

macro_rules! tag {
	($graphics:expr, $name:ident) => {
		pub const $name: &Tag = $graphics.tags().get(stringify!($name));
	};
}

tag!(GRAPHICS, PLAYER);
tag!(GRAPHICS, BG);
tag!(GRAPHICS, BG_COLUMN);
tag!(GRAPHICS, DIRT);
tag!(GRAPHICS, DIRT_LONG);
tag!(GRAPHICS, DIRT_TALL);
tag!(GRAPHICS, DIRT_BIG);
tag!(GRAPHICS, BRICK);
tag!(GRAPHICS, DIRT_SLOPE);
tag!(GRAPHICS, PIPE);
tag!(GRAPHICS, BLOCK);

