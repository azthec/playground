use bevy::{ecs::query, prelude::*};
use bevy_rapier2d::prelude::*;
use log::error;

use crate::{config, materials::gradient::GradientMaterial};

use super::player::Player;

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EnemyCollisionEvent>();
        app.add_systems(Startup, init);
        app.add_systems(
            Update,
            (respawn, enemy_vision_system, enemy_movement_system).chain(),
        );
    }
}

#[derive(Component)]
pub struct Enemy;

fn init(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut gradient_materials: ResMut<Assets<GradientMaterial>>,
) {
    commands.spawn(EnemiesLevel);
    spawn_level(commands, meshes, gradient_materials);
}

fn respawn(
    respawn_query: Query<Entity, With<Respawn>>,
    enemies: Query<(Entity), With<Enemy>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut gradient_materials: ResMut<Assets<GradientMaterial>>,
) {
    if let Ok(respawn_entity) = respawn_query.get_single() {
        commands.entity(respawn_entity).remove::<Respawn>();
        let entities: Vec<Entity> = enemies.iter().collect();
        for entity in entities {
            commands.entity(entity).despawn();
        }
        spawn_level(commands, meshes, gradient_materials);
    }
}

#[derive(Component)]
pub struct EnemiesLevel;

#[derive(Component)]
pub struct Respawn;

#[derive(Debug)]
pub struct RespawnEnemies;

impl Command for RespawnEnemies {
    fn apply(self, world: &mut World) {
        let mut query = world.query_filtered::<Entity, With<EnemiesLevel>>();
        let entity: Entity = query.single(&world);
        world.commands().entity(entity).insert(Respawn);
    }
}

fn spawn_level(
    mut commands: Commands<'_, '_>,
    mut meshes: ResMut<'_, Assets<Mesh>>,
    mut gradient_materials: ResMut<'_, Assets<GradientMaterial>>,
) {
    let spawns: [(f32, f32); 16] = [
        (400., 50.),
        (400., 1000.),
        (800., 800.),
        (800., 1000.),
        (1000., 100.),
        (1000., 400.),
        (1000., 800.),
        (1200., 100.),
        (1200., 800.),
        (1300., 400.),
        (1500., 400.),
        (1600., 800.),
        (1650., 100.),
        (1650., 400.),
        (1800., 400.),
        (1800., 800.),
    ];

    for (spawn_x, spawn_y) in spawns {
        commands.spawn((
            Name::new("Enemy"),
            Enemy,
            Mesh2d(meshes.add(Circle::new(config::ENTITY_SIZE))),
            MeshMaterial2d(gradient_materials.add(GradientMaterial {
                color: config::COLOR_ENEMY.to_linear(),
                origin: Vec2::ZERO,
                max_distance: 250.,
            })),
            Transform::from_xyz(spawn_x, spawn_y, 2.),
            RigidBody::Fixed,
            GravityScale(0.),
            Collider::ball(config::ENTITY_SIZE),
            ActiveEvents::COLLISION_EVENTS,
            Visibility::Hidden,
            Sensor,
        ));
    }
}

#[derive(Event)]
struct EnemyCollisionEvent;

fn enemy_vision_system(
    mut _gizmos: Gizmos,
    mut enemy_query: Query<(Entity, &GlobalTransform, &mut Visibility), With<Enemy>>,
    player_query: Query<(Entity, &GlobalTransform), With<Player>>,
    rapier_context: ReadRapierContext,
    mut writer: EventWriter<EnemyCollisionEvent>,
) {
    let _enemies: Vec<Entity> = enemy_query.iter().map(|(entity, _, _)| entity).collect();
    for (enemy_entity, enemy_transform, mut visibility) in enemy_query.iter_mut() {
        for (player_entity, player_transform) in player_query.iter() {
            let origin = enemy_transform.translation().truncate();
            let target = player_transform.translation().truncate();
            let direction = target - origin;
            let max_toi = direction.length();

            let solid = true;
            let filter = QueryFilter::default().exclude_collider(enemy_entity);

            // TODO if debug draw raycast
            // _gizmos.line_2d(origin, target, config::COLOR_BURNT);

            let context = rapier_context.single();
            if let Some((entity_hit, _toi)) = context.cast_ray(
                origin,
                direction.normalize_or_zero(),
                max_toi,
                solid,
                filter,
            ) {
                *visibility = if Some(entity_hit) == Some(player_entity) {
                    Visibility::Visible
                } else {
                    Visibility::Hidden
                }
            }
            // collision detection
            if let Some((entity_hit, _toi)) = context.cast_ray(
                origin,
                direction.normalize_or_zero(),
                config::ENTITY_SIZE * 2.,
                solid,
                filter,
            ) {
                if Some(entity_hit) == Some(player_entity) {
                    error!("collided!");
                    writer.send(EnemyCollisionEvent);
                }
            }
        }
    }
}

fn enemy_movement_system(
    mut enemy_query: Query<(&mut Transform, &Visibility), (With<Enemy>, Without<Player>)>,
    player_query: Query<&Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        let player_pos = player_transform.translation;

        for (mut enemy_transform, enemy_visibility) in enemy_query.iter_mut() {
            // only move enemies if the player is visible
            // TODO this should be an event and not a byproduct of visibility
            if *enemy_visibility != Visibility::Visible {
                continue;
            }

            let enemy_pos = enemy_transform.translation;

            let direction = (player_pos - enemy_pos).truncate().normalize_or_zero();

            enemy_transform.translation.x += direction.x * config::ENEMY_SPEED * time.delta_secs();
            enemy_transform.translation.y += direction.y * config::ENEMY_SPEED * time.delta_secs();
        }
    }
}
