//! consider using [fixed timestep](https://github.com/bevyengine/bevy/blob/main/examples/movement/physics_in_fixed_timestep.rs).

use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;

use crate::{physics::ground::GroundDetection, AppSet};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<MovementController>();
    app.add_systems(Update, apply_movement.in_set(AppSet::Update));
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct MovementController {
    pub intent: Vec2,
    pub max_speed: f32,
}

impl Default for MovementController {
    fn default() -> Self {
        Self {
            intent: Vec2::ZERO,
            max_speed: 200.0,
        }
    }
}

// MovementController is where the entity wants to move,
// Transform is the actual rendered model location
fn apply_movement(
    time: Res<Time>,
    mut query: Query<(&MovementController, &GroundDetection, &mut Velocity)>,
) {
    for (controller, ground, mut velocity) in &mut query {
        let intended_velocity = controller.max_speed * controller.intent;
        velocity.linvel = intended_velocity;
    }
}
