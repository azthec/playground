//! consider using [fixed timestep](https://github.com/bevyengine/bevy/blob/main/examples/movement/physics_in_fixed_timestep.rs).

use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;

use crate::{physics::ground::GroundDetection, AppSet};

use super::{entities::Player, input::PlayerInput};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<MovementController>();
    app.add_systems(
        Update,
        (apply_movement, apply_movement_player).in_set(AppSet::Update),
    );
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct MovementController {
    pub intents: Vec<PlayerInput>,
    pub intent: Vec2,
    pub max_speed: f32,
    pub jump_speed: f32,
    pub fall_speed: f32,
}

impl Default for MovementController {
    fn default() -> Self {
        Self {
            intents: Vec::new(),
            intent: Vec2::ZERO,
            max_speed: 200.,
            jump_speed: 400.,
            fall_speed: 200.,
        }
    }
}

// MovementController is where the entity wants to move,
// Transform is the actual rendered model location
fn apply_movement(
    mut query: Query<(&MovementController, &GroundDetection, &mut Velocity), Without<Player>>,
) {
    for (controller, _ground, mut velocity) in &mut query {
        let intended_velocity = controller.max_speed * controller.intent;
        velocity.linvel = intended_velocity;
    }
}

fn apply_movement_player(
    mut query: Query<(&MovementController, &GroundDetection, &mut Velocity), With<Player>>,
) {
    for (controller, ground, mut velocity) in &mut query {
        velocity.linvel.x = 0.;
        for intent in controller.intents.iter() {
            match intent {
                PlayerInput::Left => {
                    velocity.linvel.x = controller.max_speed * -1.0;
                }
                PlayerInput::Right => {
                    velocity.linvel.x = controller.max_speed * 1.0;
                }
                PlayerInput::Fall => {
                    velocity.linvel.y = controller.fall_speed * -1.0;
                }
                PlayerInput::Jump => {
                    if ground.on_ground {
                        velocity.linvel.y = controller.jump_speed * 1.0;
                    }
                }
                _ => {
                    velocity.linvel.y = velocity
                        .linvel
                        .y
                        .clamp(-controller.fall_speed, controller.jump_speed);
                }
            }
        }
    }
}
