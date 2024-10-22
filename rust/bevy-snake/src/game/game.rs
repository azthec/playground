use bevy::prelude::*;

use super::snake::TailSegments;

#[derive(Event)]
pub struct GameOverEvent;

#[derive(Event)]
pub struct GamePauseEvent;

#[derive(Resource, Default)]
pub struct Score(usize);

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(TailSegments::default());
    app.add_systems(Startup, spawn);
}

fn game_over(
    mut commands: Commands,
    mut reader: EventReader<GameOverEvent>,
    segments_res: ResMut<TailSegments>,
    // food: Query<Entity, With<Food>>,
    // segments: Query<Entity, With<Tail>>,
    mut score: ResMut<Score>,
) {
    if reader.read().next().is_some() {
        // for ent in food.iter().chain(segments.iter()) {
        //     commands.entity(ent).despawn();
        // }
        score.0 = 0;
        // spawn_snake(commands, segments_res);
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

