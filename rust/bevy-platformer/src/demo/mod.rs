use bevy::prelude::*;

mod animation;
pub mod level;
mod movement;
mod entities;
mod input;
mod camera;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        // animation::plugin,
        entities::plugin,
        input::plugin,
        level::plugin,
        movement::plugin,
        camera::CameraPlugin
    ));
}
