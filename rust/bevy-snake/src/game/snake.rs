use bevy::prelude::*;

use crate::*;

use super::grid::{Position, Size};

#[derive(Component)]
pub struct Head {
    direction: Direction,
}

#[derive(Component)]
pub struct Tail;

#[derive(Resource, Default)]
pub struct TailSegments(Vec<Entity>);

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(TailSegments::default());
    app.add_systems(Startup, spawn);
}

fn spawn(mut commands: Commands, mut segments: ResMut<TailSegments>) {
    *segments = TailSegments(vec![
        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: COLOR_SNAKE_HEAD,
                    ..default()
                },
                ..default()
            })
            // .insert(Head {
            //     direction: Direction::Up,
            // })
            .insert(Tail)
            .insert(Position { x: 3, y: 3 })
            .insert(Size::square(0.8))
            .id(),
        spawn_segment(commands, Position { x: 3, y: 2 }),
    ]);
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
