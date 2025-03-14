pub mod colliders;
pub mod ground;
mod walls;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0));
    app.add_systems(Startup, setup);
    app.add_plugins((walls::WallPlugin, ground::GroundDetectionPlugin));
}

pub fn setup(mut rapier_config: Query<&mut RapierConfiguration>) {
    rapier_config.single_mut().gravity = Vec2::new(0.0, -2000.0);
}
