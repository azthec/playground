use bevy::prelude::*;

use crate::*;

use super::grid::{Position, Size};
use crate::types::direction::Direction;

#[derive(Component)]
pub struct Head {
    pub direction: Direction,
}

#[derive(Component)]
pub struct Tail;

#[derive(Resource, Default)]
pub struct TailSegments(pub Vec<Entity>);

#[derive(Resource, Default)]
pub struct LastTailPosition(pub Option<Position>);

#[derive(Event)]
pub struct SpawnSnakeEvent;

#[derive(Event)]
pub struct DespawnSnakeEvent;

#[derive(Event)]
pub struct GrowSnakeEvent;

pub(super) fn plugin(app: &mut App) {
    app.add_event::<SpawnSnakeEvent>();
    app.add_event::<DespawnSnakeEvent>();
    app.add_event::<GrowSnakeEvent>();
    app.insert_resource(TailSegments::default());
    app.add_systems(PreUpdate, (despawn, spawn).chain());
    app.add_systems(FixedUpdate, (grow).in_set(FixedSet::Post));
}

fn spawn(
    mut commands: Commands,
    mut reader: EventReader<SpawnSnakeEvent>,
    mut segments: ResMut<TailSegments>,
) {
    if reader.read().next().is_some() {
        *segments = TailSegments(vec![
            commands
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        color: COLOR_SNAKE_HEAD,
                        ..default()
                    },
                    ..default()
                })
                .insert(Head {
                    direction: Direction::Up,
                })
                .insert(Tail)
                .insert(Position { x: 3, y: 3 })
                .insert(Size::square(0.8))
                .id(),
            spawn_segment(commands, Position { x: 3, y: 2 }),
        ]);
    }
}

fn despawn(
    mut commands: Commands,
    mut reader: EventReader<DespawnSnakeEvent>,
    snake_entities: Query<Entity, With<Tail>>,
) {
    if reader.read().next().is_some() {
        for segment in snake_entities.iter() {
            commands.entity(segment).despawn();
        }
    }
}

fn grow(
    commands: Commands,
    mut reader: EventReader<GrowSnakeEvent>,
    last_tail_position: Res<LastTailPosition>,
    mut segments: ResMut<TailSegments>,
) {
    if reader.read().next().is_some() {
        segments
            .0
            .push(spawn_segment(commands, last_tail_position.0.unwrap()));
    }
}

fn spawn_segment(mut commands: Commands, position: Position) -> Entity {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: COLOR_SNAKE_TAIL,
                ..default()
            },
            ..default()
        })
        .insert(Tail)
        .insert(position)
        .insert(Size::square(0.65))
        .id()
}
