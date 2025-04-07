use bevy::prelude::*;

use bevy_rapier2d::prelude::*;

#[derive(Component, Default)]
pub struct Spike;

#[derive(Component, Default)]
struct SpikeCollider;

#[derive(Clone, Default, Component)]
pub struct Spikeable {}

#[derive(Event)]
pub struct SpikeCollisionEvent;

fn detect_spike_collision(
    mut collisions: EventReader<CollisionEvent>,
    mut writer: EventWriter<SpikeCollisionEvent>,
    spikes: Query<Entity, With<SpikeCollider>>,
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

// spikes are half a tile and their colliders must be offset
fn spawn_spike_child_colliders(mut commands: Commands, spike_query: Query<Entity, Added<Spike>>) {
    for entity in &spike_query {
        commands.entity(entity).with_children(|children| {
            children.spawn((
                Collider::capsule_x(5., 4.),
                Transform::from_xyz(0., -4.5, 0.),
                ActiveEvents::COLLISION_EVENTS,
                Sensor,
                SpikeCollider,
            ));
        });
    }
}

pub struct SpikeDetectionPlugin;

impl Plugin for SpikeDetectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpikeCollisionEvent>();
        app.add_systems(
            Update,
            (spawn_spike_child_colliders, detect_spike_collision),
        );
    }
}
