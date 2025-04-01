use bevy::{prelude::*, time::Stopwatch};
use bevy_rapier2d::prelude::Velocity;

use crate::{physics::ground::GroundDetection, AppSet};

use super::{entities::Player, input::PlayerInput};

const COYOTE_TIME: f32 = 0.1;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<MovementController>();
    app.add_systems(
        Update,
        (tick_jump_timer, reset_jump_timer)
            .chain()
            .in_set(AppSet::TickTimers),
    );
    app.add_systems(Update, (apply_movement_player).in_set(AppSet::Update));
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct MovementController {
    pub intents: Vec<PlayerInput>,
    pub max_speed: f32,
    pub jump_speed: f32,
    pub jump_stopwatch: Stopwatch,
    pub fall_speed: f32,
}

impl Default for MovementController {
    fn default() -> Self {
        Self {
            intents: Vec::new(),
            max_speed: 200.,
            jump_speed: 400.,
            jump_stopwatch: Stopwatch::new(),
            fall_speed: 200.,
        }
    }
}

fn reset_jump_timer(mut query: Query<(&mut MovementController, &GroundDetection), With<Player>>) {
    for (mut controller, ground) in &mut query {
        if ground.on_ground {
            controller.jump_stopwatch.reset();
        }
    }
}

fn tick_jump_timer(time: Res<Time>, mut query: Query<&mut MovementController, With<Player>>) {
    let Ok(mut controller) = query.get_single_mut() else {
        return;
    };
    controller.jump_stopwatch.tick(time.delta());
}

// MovementController is where the entity wants to move,
// Transform is the actual rendered model location
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
                    if ground.on_ground || controller.jump_stopwatch.elapsed_secs() <= COYOTE_TIME {
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
