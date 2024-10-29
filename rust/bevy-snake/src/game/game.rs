use bevy::prelude::*;

use crate::AppSet;

use super::snake::{DespawnSnakeEvent, SpawnSnakeEvent};

#[derive(Event)]
pub struct GameOverEvent;

#[derive(Event)]
pub struct GamePauseEvent;

#[derive(Event)]
pub struct ScoreEvent {
    pub amount: usize,
}

#[derive(Resource, Default)]
pub struct Score(pub usize);

pub(super) fn plugin(app: &mut App) {
    app.add_event::<GameOverEvent>();
    app.add_event::<GamePauseEvent>();
    app.add_event::<ScoreEvent>();
    app.insert_resource(Score::default());
    app.add_systems(Startup, spawn);
    app.add_systems(Update, toggle_pause);
    app.add_systems(
        FixedUpdate,
        (score, game_over).chain().in_set(AppSet::PostUpdate),
    );
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

fn score(mut reader: EventReader<ScoreEvent>, mut score: ResMut<Score>) {
    if let Some(score_increase) = reader.read().next() {
        score.0 += score_increase.amount;
    }
}
