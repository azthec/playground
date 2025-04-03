use bevy::prelude::*;

use bevy_rapier2d::prelude::*;

#[derive(Component, Default)]
pub struct Goal;

#[derive(Clone, Default, Component)]
pub struct GoalDetection {}

#[derive(Event)]
pub struct GoalCollisionEvent;

pub fn detect_goal_collision(
    mut collisions: EventReader<CollisionEvent>,
    mut writer: EventWriter<GoalCollisionEvent>,
    goal_entities: Query<Entity, With<Goal>>,
    goal_colliders: Query<&GoalDetection>,
) {
    for collision in collisions.read() {
        match collision {
            CollisionEvent::Started(collider_a, collider_b, _) => {
                if let (Ok(_), Ok(_)) = (goal_colliders.get(*collider_a), goal_entities.get(*collider_b)) {
                    writer.send(GoalCollisionEvent);
                }
                if let (Ok(_), Ok(_)) = (goal_entities.get(*collider_a), goal_colliders.get(*collider_b)) {
                    writer.send(GoalCollisionEvent);
                };
            }
            CollisionEvent::Stopped(_, _, _) => (),
        }
    }
}

pub struct GoalDetectionPlugin;

impl Plugin for GoalDetectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GoalCollisionEvent>();
        app.add_systems(Update, detect_goal_collision);
    }
}
