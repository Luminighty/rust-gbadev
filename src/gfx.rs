use agb::{include_aseprite, display::object::{Graphics, Tag}};

pub const GRAPHICS: &Graphics = include_aseprite!(
	"gfx/tiles.aseprite",
	"gfx/ores.aseprite",
	"gfx/ui.aseprite"
);

macro_rules! tag {
	($graphics:expr, $name:ident, $tag:literal) => {
		pub const $name: &Tag = $graphics.tags().get($tag);
	};
}

// tag!(GRAPHICS, TILE_, "");
