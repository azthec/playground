pub mod camera;
pub mod colorscheme;
pub mod config;
pub mod game;
pub mod grid;
pub mod materials;
pub mod physics;
pub mod simplegrid;
pub mod visibility;

use bevy::{
    asset::AssetMetaCheck,
    audio::{AudioPlugin, Volume},
    prelude::*,
};
use camera::CameraPlugin;
use game::{enemies::EnemiesPlugin, level::LevelPlugin, player::PlayerPlugin};
use materials::gradient::GradientMaterialPlugin;
use physics::PhysicsPlugin;
use visibility::VisibilityPlugin;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        // Order new `AppStep` variants by adding them here:
        app.configure_sets(
            Update,
            (AppSet::TickTimers, AppSet::RecordInput, AppSet::Update).chain(),
        );

        // Add Bevy plugins.
        app.add_plugins((
            DefaultPlugins
                .set(AssetPlugin {
                    // Wasm builds will check for meta files (that don't exist) if this isn't set.
                    // This causes errors and even panics on web build on itch.
                    // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Window {
                        title: "Bevy Mazerunner".to_string(),
                        canvas: Some("#bevy".to_string()),
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: true,
                        ..default()
                    }
                    .into(),
                    ..default()
                })
                .set(AudioPlugin {
                    global_volume: GlobalVolume {
                        volume: Volume::new(0.3),
                    },
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        ));

        // Add other plugins.
        app.add_plugins((
            CameraPlugin,
            LevelPlugin,
            PhysicsPlugin,
            VisibilityPlugin,
            PlayerPlugin,
            GradientMaterialPlugin,
            EnemiesPlugin,
        ));
    }
}

/// High-level groupings of systems for the app in the `Update` schedule.
/// When adding a new variant, make sure to order it in the `configure_sets`
/// call above.
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum AppSet {
    /// Tick timers.
    TickTimers,
    /// Record player input.
    RecordInput,
    /// Do everything else (consider splitting this into further variants).
    Update,
}
