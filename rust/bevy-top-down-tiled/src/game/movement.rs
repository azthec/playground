use bevy::{math::VectorSpace, prelude::*};
use bevy_rapier2d::prelude::Velocity;

use crate::AppSet;

use super::{entities::Player, input::PlayerInput};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<MovementController>();
    app.add_systems(Update, (apply_movement_player).in_set(AppSet::Update));
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct MovementController {
    pub intents: Vec<PlayerInput>,
    pub max_speed: f32,
}

impl Default for MovementController {
    fn default() -> Self {
        Self {
            intents: Vec::new(),
            max_speed: 100.,
        }
    }
}
fn apply_movement_player(
    mut query: Query<(&MovementController, &mut Velocity), With<Player>>,
) {
    for (controller, mut velocity) in &mut query {
        velocity.linvel = Vec2::ZERO;
        for intent in controller.intents.iter() {
            match intent {
                PlayerInput::Up => {
                    velocity.linvel.y = controller.max_speed * 1.0;
                }
                PlayerInput::Down => {
                    velocity.linvel.y = controller.max_speed * -1.0;
                }
                PlayerInput::Left => {
                    velocity.linvel.x = controller.max_speed * -1.0;
                }
                PlayerInput::Right => {
                    velocity.linvel.x = controller.max_speed * 1.0;
                }
            }
        }
    }
}
