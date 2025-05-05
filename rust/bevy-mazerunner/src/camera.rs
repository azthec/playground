use crate::config;
use bevy::{prelude::*, render::camera::ScalingMode};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
        app.add_systems(Update, update_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera"),
        Camera2d,
        OrthographicProjection {
            scaling_mode: ScalingMode::Fixed {
                width: config::RESOLUTION_WIDTH,
                height: config::RESOLUTION_HEIGHT,
            },
            ..OrthographicProjection::default_2d()
        },
        IsDefaultUiCamera,
    ));
}

fn update_camera(
    mut camera_transform: Query<&mut Transform, With<Camera>>,
) {
    let mut transform = camera_transform.single_mut();
    transform.translation.x = config::RESOLUTION_WIDTH / 2.;
    transform.translation.y = config::RESOLUTION_HEIGHT / 2.;
}
