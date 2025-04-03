use bevy::prelude::*;

use crate::AppSet;

use super::{entities::Player, level::{RespawnLevel, SpawnLevelOffset}, movement::MovementController};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (record_player_directional_input, respawn_level).in_set(AppSet::RecordInput),
    );
}

#[derive(Debug, Reflect)]
pub enum PlayerInput {
    Up,
    Left,
    Down,
    Right,
    Jump,
    Fall,
}

fn record_player_directional_input(
    input: Res<ButtonInput<KeyCode>>,
    mut controller_query: Query<&mut MovementController, With<Player>>,
) {
    let Ok(mut controller) = controller_query.get_single_mut() else {
        return;
    };

    // Collect directional input.
    let mut intents = Vec::new();

    if input.pressed(KeyCode::KeyW)
        || input.pressed(KeyCode::ArrowUp)
        || input.pressed(KeyCode::Space)
    {
        intents.push(PlayerInput::Up);
    }
    if input.pressed(KeyCode::KeyS) || input.pressed(KeyCode::ArrowDown) {
        intents.push(PlayerInput::Down);
    }
    if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
        intents.push(PlayerInput::Left);
    }
    if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
        intents.push(PlayerInput::Right);
    }
    if input.just_pressed(KeyCode::Space) {
        intents.push(PlayerInput::Jump);
    }
    if input.just_released(KeyCode::Space) {
        intents.push(PlayerInput::Fall);
    }

    controller.intents = intents;
}

fn respawn_level(mut commands: Commands, input: Res<ButtonInput<KeyCode>>) {
    if input.just_pressed(KeyCode::KeyR) {
        commands.queue(RespawnLevel);
    }
    if input.just_pressed(KeyCode::KeyN) {
       commands.queue(SpawnLevelOffset { offset: 1 });
    }
    if input.just_pressed(KeyCode::KeyP) {
       commands.queue(SpawnLevelOffset { offset: -1 });
    }

}
