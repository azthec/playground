// fn gcd(mut a: f32, mut b: f32) -> f32 {
//     while b != 0. {
//         let t = b;
//         b = a % b;
//         a = t;
//     }
//     a
// }
// const GRID_WIDTH: i32 = (RESOLUTION_WIDTH / gdc_resolution(RESOLUTION_HEIGHT, RESOLUTION_WIDTH).floor() as i32;
// const GRID_HEIGHT: i32 = (RESOLUTION_HEIGHT / gdc_resolution(RESOLUTION_HEIGHT, RESOLUTION_WIDTH).floor() as i32;

use crate::config::*;
use bevy::{prelude::*, window::WindowResolution};

// TODO look into https://github.com/bevyengine/bevy/blob/main/examples/window/window_settings.rs
// TODO run GDC by screen resolution on startup
pub(super) fn plugin(app: &mut App) {
    app.add_plugins(
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "bevy-game".into(),
                resolution: WindowResolution::new(RESOLUTION_WIDTH, RESOLUTION_HEIGHT)
                    .with_scale_factor_override(1.),
                resizable: false,
                ..default()
            }),
            ..default()
        }),
    );
}
