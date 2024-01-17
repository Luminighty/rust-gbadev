use agb::{include_aseprite, display::object::{Graphics, Tag}};

pub const GRAPHICS: &Graphics = include_aseprite!("gfx/sprites.aseprite");

pub const PADDLE_END: &Tag = GRAPHICS.tags().get("Paddle End");
pub const PADDLE_MID: &Tag = GRAPHICS.tags().get("Paddle Mid");
pub const BALL: &Tag = GRAPHICS.tags().get("Ball");
pub const SAD: &Tag = GRAPHICS.tags().get("Sad");
