use bevy::prelude::*;

use crate::AppSet;

use super::snake::{DespawnSnakeEvent, SpawnSnakeEvent};

#[derive(Event)]
pub struct GameOverEvent;

#[derive(Event)]
pub struct GamePauseEvent;

#[derive(Resource, Default)]
pub struct Score(pub usize);

pub(super) fn plugin(app: &mut App) {
    app.add_event::<GameOverEvent>();
    app.add_event::<GamePauseEvent>();
    app.insert_resource(Score::default());
    app.add_systems(Startup, spawn);
    app.add_systems(Update, toggle_pause);
    app.add_systems(FixedUpdate, (game_over).in_set(AppSet::PostUpdate));
}

fn spawn(mut snake_spawn_writer: EventWriter<SpawnSnakeEvent>) {
    snake_spawn_writer.send(SpawnSnakeEvent);
}

fn game_over(
    mut reader: EventReader<GameOverEvent>,
    mut snake_spawn_writer: EventWriter<SpawnSnakeEvent>,
    mut snake_despawn_writer: EventWriter<DespawnSnakeEvent>,
    mut score: ResMut<Score>,
) {
    if reader.read().next().is_some() {
        score.0 = 0;
        snake_spawn_writer.send(SpawnSnakeEvent);
        snake_despawn_writer.send(DespawnSnakeEvent);
    }
}

fn toggle_pause(
    mut game_pause_reader: EventReader<GamePauseEvent>,
    mut time: ResMut<Time<Virtual>>,
) {
    if game_pause_reader.read().next().is_some() {
        if time.is_paused() {
            time.unpause();
        } else {
            time.pause();
        }
    }
}
