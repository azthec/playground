use bevy::prelude::*;

mod animation;
mod camera;
mod entities;
mod hazards;
mod input;
mod movement;
pub mod level;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        animation::plugin,
        entities::plugin,
        hazards::plugin,
        input::plugin,
        level::plugin,
        movement::plugin,
        camera::CameraPlugin
    ));
}
