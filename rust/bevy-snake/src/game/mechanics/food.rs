use crate::game::game::GameOverEvent;
use crate::game::game::Score;
use crate::game::grid::Position;
use crate::game::grid::Size;
use crate::game::grid::GRID_HEIGHT;
use crate::game::grid::GRID_WIDTH;
use crate::game::snake::GrowSnakeEvent;
use crate::game::snake::Head;
use crate::game::snake::Tail;
use crate::game::snake::TailSegments;
use crate::AppSet;
use crate::COLOR_FOOD;
use bevy::prelude::*;
use rand::prelude::SliceRandom;
use std::collections::HashSet;

#[derive(Event)]
struct GrowthEvent;

#[derive(Component)]
struct Food;

pub(super) fn plugin(app: &mut App) {
    app.add_event::<GrowthEvent>();
    app.add_systems(
        FixedUpdate,
        (
            (snake_eating, snake_growth, food_spawner).chain().in_set(AppSet::Update),
            (despawn).in_set(AppSet::Cleanup),
        ),
    );
}

fn snake_eating(
    mut commands: Commands,
    mut growth_writer: EventWriter<GrowthEvent>,
    food_positions: Query<(Entity, &Position), With<Food>>,
    head_positions: Query<&Position, With<Head>>,
) {
    for head_pos in head_positions.iter() {
        for (ent, food_pos) in food_positions.iter() {
            if food_pos == head_pos {
                commands.entity(ent).despawn();
                growth_writer.send(GrowthEvent);
            }
        }
    }
}

fn snake_growth(
    mut growth_reader: EventReader<GrowthEvent>,
    mut writer: EventWriter<GrowSnakeEvent>,
    mut score: ResMut<Score>,
) {
    if growth_reader.read().next().is_some() {
        writer.send(GrowSnakeEvent);
        score.0 += 1;
    }
}

fn food_spawner(
    food_query: Query<&Food>,
    segments: ResMut<TailSegments>,
    mut commands: Commands,
    heads: Query<&Position, With<Head>>,
    positions: Query<&Position, With<Tail>>,
) {
    let food_count = food_query.into_iter().count();
    if food_count < 5 {
        if let Some(head_position) = heads.iter().next() {
            let all_pos: HashSet<(i32, i32)> = (0..GRID_HEIGHT)
                .flat_map(|y| (0..GRID_WIDTH).map(move |x| (x, y)))
                .collect();

            let mut occupied_pos: HashSet<(i32, i32)> = segments
                .0
                .iter()
                .map(|segment| *positions.get(*segment).unwrap())
                .map(|pos| (pos.x, pos.y))
                .collect();
            occupied_pos.insert((head_position.x, head_position.y));

            let valid_pos: Vec<(i32, i32)> = all_pos.difference(&occupied_pos).cloned().collect();

            for random_position in
                valid_pos.choose_multiple(&mut rand::thread_rng(), 5 - food_count)
            {
                commands
                    .spawn(SpriteBundle {
                        sprite: Sprite {
                            color: COLOR_FOOD,
                            ..default()
                        },
                        ..default()
                    })
                    .insert(Food)
                    .insert(Position {
                        x: random_position.0,
                        y: random_position.1,
                    })
                    .insert(Size::square(0.5));
            }
        }
    }
}

fn despawn(
    mut commands: Commands,
    mut reader: EventReader<GameOverEvent>,
    food: Query<Entity, With<Food>>,
) {
    if reader.read().next().is_some() {
        for entity in food.iter() {
            commands.entity(entity).despawn();
        }
    }
}
