use bevy::prelude::*;

use bevy_rapier2d::prelude::*;

#[derive(Component, Default)]
pub struct Spike;

#[derive(Clone, Default, Component)]
pub struct Spikeable {}

#[derive(Event)]
pub struct SpikeCollisionEvent;

pub fn detect_spike_collision(
    mut collisions: EventReader<CollisionEvent>,
    mut writer: EventWriter<SpikeCollisionEvent>,
    spikes: Query<Entity, With<Spike>>,
    spikeables: Query<&Spikeable>,
) {
    for collision in collisions.read() {
        match collision {
            CollisionEvent::Started(collider_a, collider_b, _) => {
                if let (Ok(_), Ok(_)) = (spikeables.get(*collider_a), spikes.get(*collider_b)) {
                    writer.send(SpikeCollisionEvent);
                }
                if let (Ok(_), Ok(_)) = (spikes.get(*collider_a), spikeables.get(*collider_b)) {
                    writer.send(SpikeCollisionEvent);
                };
            }
            CollisionEvent::Stopped(_, _, _) => (),
        }
    }
}

pub struct SpikeDetectionPlugin;

impl Plugin for SpikeDetectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpikeCollisionEvent>();
        app.add_systems(Update, detect_spike_collision);
    }
}
