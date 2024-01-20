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


tag!(GRAPHICS, TILE_AIR, "air");

//pub const TILE_AIR: &Tag = GRAPHICS.tags().get("air");
pub const TILE_WALL: &Tag = GRAPHICS.tags().get("wall");
pub const TILE_ORE: &Tag = GRAPHICS.tags().get("ore");

pub const GOLD: &Tag = GRAPHICS.tags().get("gold");
pub const IRON: &Tag = GRAPHICS.tags().get("iron");
pub const EMERALD: &Tag = GRAPHICS.tags().get("emerald");
pub const RUBY: &Tag = GRAPHICS.tags().get("ruby");
