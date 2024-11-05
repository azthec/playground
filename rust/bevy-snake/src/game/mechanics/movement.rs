use crate::GRID_HEIGHT;
use crate::GRID_WIDTH;
use bevy::prelude::*;

use crate::game::input::InputBuffer;
use crate::game::snake::LastTailPosition;
use crate::types::direction::Direction;
use crate::{
    game::{
        game::GameOverEvent,
        grid::Position,
        snake::{Head, TailSegments},
    },
    AppSet,
};

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(InputBuffer::default());
    app.insert_resource(LastTailPosition::default());
    app.add_systems(FixedUpdate, (snake_movement).in_set(AppSet::PreUpdate));
}

fn snake_movement(
    segments: ResMut<TailSegments>,
    mut input_buffer: ResMut<InputBuffer>,
    mut heads: Query<(Entity, &mut Head)>,
    mut positions: Query<&mut Position>,
    mut last_tail_position: ResMut<LastTailPosition>,
    mut game_over_writer: EventWriter<GameOverEvent>,
) {
    if let Some((head_entity, mut head)) = heads.iter_mut().next() {
        let segment_positions = segments
            .0
            .iter()
            .map(|e| *positions.get_mut(*e).unwrap())
            .collect::<Vec<Position>>();
        let mut head_pos = positions.get_mut(head_entity).unwrap();
        head.direction = input_buffer.0.pop().unwrap_or(head.direction);
        match &head.direction {
            Direction::Left => {
                head_pos.x -= 1;
            }
            Direction::Right => {
                head_pos.x += 1;
            }
            Direction::Up => {
                head_pos.y += 1;
            }
            Direction::Down => {
                head_pos.y -= 1;
            }
        };
        if head_pos.x < 0 || head_pos.y < 0 || head_pos.x >= GRID_WIDTH || head_pos.y >= GRID_HEIGHT
        {
            game_over_writer.send(GameOverEvent);
        }
        if segment_positions.contains(&head_pos) {
            game_over_writer.send(GameOverEvent);
        }
        segment_positions
            .iter()
            .zip(segments.0.iter().skip(1))
            .for_each(|(pos, segment)| {
                *positions.get_mut(*segment).unwrap() = *pos;
            });
        *last_tail_position = LastTailPosition(Some(*segment_positions.last().unwrap()));
    }
}
