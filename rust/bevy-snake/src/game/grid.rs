use crate::config::*;
use bevy::prelude::*;

pub const RESOLUTION_GCD: i32 = 120;

// const GRID_WIDTH: i32 = (RESOLUTION_WIDTH / RESOLUTION_GCD).floor() as i32;
// const GRID_HEIGHT: i32 = (RESOLUTION_HEIGHT / RESOLUTION_GCD).floor() as i32;
pub const GRID_WIDTH: i32 = 16;
pub const GRID_HEIGHT: i32 = 9;

#[derive(Component)]
pub struct Size {
    width: f32,
    height: f32,
}
impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(PostUpdate, (size_scaling, position_translation));
}

fn size_scaling(mut q: Query<(&Size, &mut Transform)>) {
    for (sprite_size, mut transform) in q.iter_mut() {
        transform.scale = Vec3::new(
            sprite_size.width / GRID_WIDTH as f32 * RESOLUTION_WIDTH,
            sprite_size.height / GRID_HEIGHT as f32 * RESOLUTION_HEIGHT,
            1.0,
        );
    }
}

fn position_translation(mut q: Query<(&Position, &mut Transform)>) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, RESOLUTION_WIDTH, GRID_WIDTH as f32),
            convert(pos.y as f32, RESOLUTION_HEIGHT, GRID_HEIGHT as f32),
            0.0,
        );
    }
}
