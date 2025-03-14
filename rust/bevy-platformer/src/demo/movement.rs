//! consider using [fixed timestep](https://github.com/bevyengine/bevy/blob/main/examples/movement/physics_in_fixed_timestep.rs).

use bevy::prelude::*;

use crate::AppSet;

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
            max_speed: 400.0,
        }
    }
}

// MovementController is where the entity wants to move,
// GridCoords is it's position in the game grid world,
// Translation is the actual rendered model location
fn apply_movement(
    time: Res<Time>,
    mut movement_query: Query<(&MovementController, &mut Transform)>,
) {
    for (controller, mut transform) in &mut movement_query {
        let velocity = controller.max_speed * controller.intent;
        transform.translation += velocity.extend(0.0) * time.delta_secs();
    }
}
