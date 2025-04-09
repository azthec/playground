use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{lighting::CameraLightingBundle, screens::Screen};

use super::entities::Player;

// TODO this camera logic has to be revisited, it should be loaded from the level or it causes
// problems when the level size changes
const ASPECT_RATIO: f32 = 16. / 9.;
const TILE_SIZE: f32 = 16.;
const TILE_WIDTH: f32 = 32.;
const TILE_HEIGHT: f32 = 32.;
const WIDTH: f32 = TILE_SIZE * TILE_WIDTH;
const HEIGHT: f32 = TILE_SIZE * TILE_HEIGHT;
const CAMERA_DECAY_RATE: f32 = 5.;

pub struct CameraPlugin;

#[derive(Default, Component)]
pub struct GameCamera;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
        app.add_systems(
            Update,
            (update_camera, fit_camera, clamp_camera)
                .chain()
                .run_if(in_state(Screen::Gameplay)),
        );
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera"),
        Camera2d,
        OrthographicProjection {
            ..OrthographicProjection::default_2d()
        },
        GameCamera,
        CameraLightingBundle::default()
    ));
}

fn update_camera(
    mut camera: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player: Query<&Transform, (With<Player>, Without<Camera2d>)>,
    time: Res<Time>,
) {
    let Ok(mut camera) = camera.get_single_mut() else {
        return;
    };

    let Ok(player) = player.get_single() else {
        return;
    };

    let Vec3 { x, y, .. } = player.translation;
    let direction = Vec3::new(x, y, camera.translation.z);

    // Applies a smooth effect to camera movement using stable interpolation
    // between the camera position and the player position on the x and y axes.
    camera
        .translation
        .smooth_nudge(&direction, CAMERA_DECAY_RATE, time.delta_secs());
}

fn fit_camera(mut camera_query: Query<&mut OrthographicProjection, (With<GameCamera>, Without<Player>)>) {
    let mut projection = camera_query.single_mut();

    let ratio = WIDTH / HEIGHT;
    if ratio > ASPECT_RATIO {
        let height = (HEIGHT as f32 / 9.).round() * 9.;
        let width = height * ASPECT_RATIO;
        projection.scaling_mode = bevy::render::camera::ScalingMode::Fixed { width, height };
    } else {
        let width = (WIDTH / 18.).round() * 18.;
        let height = width / ASPECT_RATIO;
        projection.scaling_mode = bevy::render::camera::ScalingMode::Fixed { width, height };
    }
}

fn clamp_camera(
    mut camera_query: Query<(&OrthographicProjection, &mut Transform), (With<GameCamera>, Without<Player>)>,
    level_query: Query<&LevelIid, (Without<OrthographicProjection>, Without<Player>)>,
    ldtk_projects: Query<&LdtkProjectHandle>,
    _level_selection: Res<LevelSelection>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
) {
    let (projection, mut camera_transform) = camera_query.single_mut();

    let camera_rect = projection.area;

    for level_iid in &level_query {
        let ldtk_project = ldtk_project_assets
            .get(ldtk_projects.single())
            .expect("Project should be loaded if level has spawned");

        let level = ldtk_project
            .get_raw_level_by_iid(&level_iid.to_string())
            .expect("Spawned level should exist in LDtk project");

        let level_rect = Rect::new(0., 0., level.px_wid as f32, level.px_hei as f32);
        let xy = camera_transform.translation.xy().clamp(
            level_rect.min + camera_rect.half_size(),
            level_rect.max - camera_rect.half_size(),
        );
        camera_transform.translation = xy.extend(camera_transform.translation.z);
    }
}
