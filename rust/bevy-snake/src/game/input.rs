use bevy::{
    app::{App, Update},
    input::ButtonInput,
    prelude::{EventWriter, KeyCode, Query, Res, ResMut, Resource, With},
};

use crate::{limited_queue::LimitedQueue, types::direction::*};

use super::{game::GamePauseEvent, snake::Head};

#[derive(Resource)]
pub struct InputBuffer(pub LimitedQueue<Direction>);

impl Default for InputBuffer {
    fn default() -> Self {
        InputBuffer(LimitedQueue::new(3))
    }
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, input_handler);
}

fn input_handler(
    input: Res<ButtonInput<KeyCode>>,
    mut input_buffer: ResMut<InputBuffer>,
    heads: Query<&Head, With<Head>>,
    mut game_pause_writer: EventWriter<GamePauseEvent>,
) {
    if input.just_pressed(KeyCode::Space) {
        game_pause_writer.send(GamePauseEvent);
    }
    if let Some(head) = heads.iter().next() {
        let pressed_direction = if input.pressed(KeyCode::ArrowLeft)
            || input.pressed(KeyCode::KeyA)
            || input.pressed(KeyCode::KeyH)
        {
            Some(Direction::Left)
        } else if input.pressed(KeyCode::ArrowDown)
            || input.pressed(KeyCode::KeyS)
            || input.pressed(KeyCode::KeyJ)
        {
            Some(Direction::Up) // TODO extremely hacky perspective fix
        } else if input.pressed(KeyCode::ArrowUp)
            || input.pressed(KeyCode::KeyW)
            || input.pressed(KeyCode::KeyK)
        {
            Some(Direction::Down) // TODO extremely hacky perspective fix
        } else if input.pressed(KeyCode::ArrowRight)
            || input.pressed(KeyCode::KeyD)
            || input.pressed(KeyCode::KeyL)
        {
            Some(Direction::Right)
        } else {
            None
        };

        if let Some(direction) = pressed_direction {
            if !input_buffer.0.contains(direction) {
                if input_buffer.0.len() == 0 {
                    if direction != head.direction && direction != head.direction.opposite() {
                        input_buffer.0.push(direction);
                    }
                } else {
                    input_buffer.0.push(direction);
                }
            }
        }
    }
}
