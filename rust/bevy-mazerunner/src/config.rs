use crate::colorscheme;
use bevy::prelude::*;


pub const RESOLUTION_WIDTH: f32 = 1920.;
pub const RESOLUTION_HEIGHT: f32 = 1080.;
pub const TILE_SIZE: i32 = 60;
pub const LINE_WIDTH: f32 = 8.;

pub const PLAYER_SPEED: f32 = 250.;
pub const ENEMY_SPEED: f32 = 200.;
pub const ENTITY_SIZE: f32 = 10.;

pub const COLOR_TILE: Color = Color::Srgba(colorscheme::SURFACE0);
pub const COLOR_EDGE: Color = Color::Srgba(colorscheme::SUBTEXT0);
pub const COLOR_TEXT: Color = Color::Srgba(colorscheme::TEXT);
pub const COLOR_SUBTEXT: Color = Color::Srgba(colorscheme::SUBTEXT0);
pub const COLOR_BURNT: Color = Color::Srgba(colorscheme::BURNT_ORANGE);
pub const COLOR_PEACH: Color = Color::Srgba(colorscheme::PEACH);
pub const COLOR_MAUVE: Color = Color::Srgba(colorscheme::MAUVE);
pub const COLOR_ENEMY: Color = Color::Srgba(colorscheme::CRUST);
