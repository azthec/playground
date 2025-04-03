use bevy::prelude::*;

use crate::{physics::{goal::GoalCollisionEvent, spike::SpikeCollisionEvent}, AppSet};

use super::level::{RespawnLevel, SpawnNextLevel};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, (next_level, reset_on_spiked).in_set(AppSet::Update));
}

pub fn reset_on_spiked(mut commands: Commands, mut reader: EventReader<SpikeCollisionEvent>) {
    if reader.read().next().is_some() {
        commands.queue(RespawnLevel);
    }
}

// TODO move or rename
pub fn next_level(mut commands: Commands, mut reader: EventReader<GoalCollisionEvent>) {
    if reader.read().next().is_some() {
        commands.queue(SpawnNextLevel);
    }
}
