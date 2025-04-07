use bevy::prelude::*;

pub mod camera;
mod entities;
mod hazards;
mod input;
mod movement;
pub mod level;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        entities::plugin,
        hazards::plugin,
        input::plugin,
        level::plugin,
        movement::plugin,
        camera::CameraPlugin
    ));
}
