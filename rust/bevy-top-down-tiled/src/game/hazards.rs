use bevy::prelude::*;

use crate::{physics::spike::SpikeCollisionEvent, AppSet};

use super::level::RespawnLevel;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, (reset_on_spiked).in_set(AppSet::Update));
}

pub fn reset_on_spiked(mut commands: Commands, mut reader: EventReader<SpikeCollisionEvent>) {
    if reader.read().next().is_some() {
        commands.queue(RespawnLevel);
    }
}

