mod game;
mod grid;
mod input;
mod mechanics;
mod snake;
mod debug;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(grid::plugin);
    app.add_plugins(snake::plugin);
    app.add_plugins(game::plugin);
    app.add_plugins(mechanics::plugin);
    app.add_plugins(input::plugin);
    app.add_plugins(debug::plugin);
}
