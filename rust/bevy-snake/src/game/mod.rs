// mod input;
// mod mechanics;
mod grid;
mod snake;
mod game;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(grid::plugin);
    app.add_plugins(snake::plugin);
}

