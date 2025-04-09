use std::f32::consts::PI;

use bevy::color::palettes::basic;
use bevy::prelude::*;

use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;

use crate::game::camera::GameCamera;
use crate::AppSet;

const MAX_DISTANCE: f32 = 50.;

#[derive(Default, Component)]
pub struct VisibilityRaySource;

pub fn raycast(
    mut commands: Commands,
    visibility_ray_source: Query<(Entity, &Transform), With<VisibilityRaySource>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    rapier_context: ReadRapierContext,
    cameras: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
) {
    let window = windows.single();

    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    for (camera, camera_transform) in &cameras {
        let Ok((entity, ray_source)) = visibility_ray_source.get_single() else {
            return;
        };
        let Ok(ray_target) = camera.viewport_to_world(camera_transform, cursor_position) else {
            return;
        };

        let player = ray_source.translation.xy();
        let mouse = ray_target.origin.xy();
        println!("ray_source {:?}", player);
        println!("ray_target {:?}", mouse);

        let context = rapier_context.single();
        let hit = context.cast_ray(
            player,
            (mouse - player).normalize(),
            MAX_DISTANCE,
            true,
            QueryFilter::default().exclude_collider(entity),
        );

        // Color in blue the entity we just hit.
        if let Some((entity, _toi)) = hit {
            let color = basic::BLUE.into();
            commands.entity(entity).insert(ColliderDebugColor(color));
        }
    }
}

#[derive(Default, Component)]
pub struct FovCone;

pub fn setup_cone(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Mesh2d(meshes.add(Cone::new(5., 40.))),
        MeshMaterial2d(materials.add(Color::hsl(360. * 10 as f32 / 10 as f32, 0.95, 0.7))),
        FovCone,
    ));
}
pub fn update_cone(
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
    player_query: Query<&Transform, (With<VisibilityRaySource>, Without<FovCone>)>,
    mut cone_query: Query<&mut Transform, (With<FovCone>, Without<VisibilityRaySource>)>,
) {
    let window = windows.single();

    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    for (camera, camera_transform) in &cameras {
        let Ok(mouse) = camera.viewport_to_world(camera_transform, cursor_position) else {
            return;
        };
        let Ok(player_transform) = player_query.get_single() else {
            return;
        };

        let mouse_world_pos = mouse.origin.xy();
        let player_pos = player_transform.translation.xy();

        // Calculate the direction from the player to the mouse
        let direction = (mouse_world_pos - player_pos).normalize();

        for mut cone_transform in &mut cone_query {
            // Center the cone on the player offset by the direction
            cone_transform.translation.x = player_pos.x + direction.x * 22.5;
            cone_transform.translation.y = player_pos.y + direction.y * 22.5;
            cone_transform.translation.z = 1.;

            // Compute rotation in radians
            let angle = direction.y.atan2(direction.x) + PI / 2.;

            // Apply rotation
            cone_transform.rotation = Quat::from_rotation_z(angle);
        }
    }
}

//////////// TEMPORARY DEBUG

#[derive(Component)]
pub struct DebugText;

fn debug_setup(mut commands: Commands) {
    commands.spawn((Text("Mouse: [] Player: []".into()), DebugText));
}

fn debug_handler(
    mut query: Query<&mut Text, With<DebugText>>,
    visibility_ray_source: Query<
        (Entity, &Transform),
        (With<VisibilityRaySource>, Without<Camera2d>),
    >,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
) {
    let window = windows.single();

    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    for (camera, camera_transform) in &cameras {
        for (_entity, ray_source) in &visibility_ray_source {
            let Ok(ray_target) = camera.viewport_to_world(camera_transform, cursor_position) else {
                return;
            };
            for mut text in &mut query {
                let player = ray_source.translation.xy();
                let mouse = ray_target.origin.xy();
                *text = Text(format!(
                    "Player (source) [{player}] Mouse (target) [{mouse}]"
                ));
            }
        }
    }
}

pub struct VisibilityPlugin;

impl Plugin for VisibilityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, raycast);
        app.add_systems(Startup, debug_setup);
        app.add_systems(FixedUpdate, debug_handler.in_set(AppSet::TickTimers));
        app.add_systems(Startup, setup_cone);
        app.add_systems(Update, update_cone);
    }
}
