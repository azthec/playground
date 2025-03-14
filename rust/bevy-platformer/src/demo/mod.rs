use bevy::prelude::*;

use crate::screens::Screen;

mod animation;
pub mod level;
mod movement;
mod entities;
mod input;
mod camera;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, camera::camera_fit_inside_current_level.run_if(in_state(Screen::Gameplay)));
    app.add_plugins((
        // animation::plugin,
        entities::plugin,
        input::plugin,
        level::plugin,
        movement::plugin,
    ));
}
