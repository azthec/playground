use bevy::color::palettes::basic;
use bevy::prelude::*;

use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;

use crate::game::camera::GameCamera;
use crate::AppSet;

// fn raycast(mut commands: Commands, rapier_context: ReadRapierContext) {
//     let ray_pos = Vec2::new(1.0, 2.0);
//     let ray_dir = Vec2::new(0.0, 1.0);
//     let max_toi = 4.0;
//     let solid = true;
//     let filter = QueryFilter::default();
//
//     let context = rapier_context.single();
//     if let Some((entity, toi)) = context.cast_ray(ray_pos, ray_dir, max_toi, solid, filter) {
//         // The first collider hit has the entity `entity` and it hit after
//         // the ray travelled a distance equal to `ray_dir * toi`.
//         let hit_point = ray_pos + ray_dir * toi;
//         println!("Entity {:?} hit at point {}", entity, hit_point);
//     }
// }

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
        let (entity, ray_source) = visibility_ray_source.single();
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
            mouse,
            f32::MAX,
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
    }
}
