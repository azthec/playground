use crate::colorscheme;
use bevy::prelude::*;

// options
pub const RESOLUTION_WIDTH: f32 = 2880.;
pub const RESOLUTION_HEIGHT: f32 = 1800.;
// pub const RESOLUTION_WIDTH: f32 = 1920.;
// pub const RESOLUTION_HEIGHT: f32 = 1280.;

pub const GRID_WIDTH: i32 = 16;
pub const GRID_HEIGHT: i32 = 9;

// game
pub const COLOR_BACKGROUND: Color = Color::Srgba(colorscheme::BASE);
pub const COLOR_SNAKE_HEAD: Color = Color::Srgba(colorscheme::TEXT);
pub const COLOR_SNAKE_TAIL: Color = Color::Srgba(colorscheme::SUBTEXT0);
pub const COLOR_FOOD: Color = Color::Srgba(colorscheme::PEACH);
pub const COLOR_PORTAL: Color = Color::Srgba(colorscheme::MAUVE);
