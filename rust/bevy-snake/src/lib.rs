mod colorscheme;
mod config;
mod debug;
mod game;
mod limited_queue;
mod window;

use bevy::{
    asset::AssetMetaCheck,
    audio::{AudioPlugin, Volume},
    prelude::*,
};
use bevy_framepace::Limiter;

use crate::config::*;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (AppSet::TickTimers, AppSet::RecordInput, AppSet::Update).chain(),
        );
        app.configure_sets(
            FixedUpdate,
            (FixedSet::Pre, FixedSet::Cur, FixedSet::Post).chain(),
        );

        app.add_systems(Startup, setup);
        app.add_plugins(window::plugin);
        app.add_plugins(bevy_framepace::FramepacePlugin);

        app.add_plugins(game::plugin);

        // Add other plugins.
        // app.add_plugins((
        //     asset_tracking::plugin,
        //     demo::plugin,
        //     screens::plugin,
        //     theme::plugin,
        // ));

        // Enable dev tools for dev builds.
        // #[cfg(feature = "dev")]
        // app.add_plugins(dev_tools::plugin);
    }
}

/// High-level groupings of systems for the app in the `Update` schedule.
/// When adding a new variant, make sure to order it in the `configure_sets`
/// call above.
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum AppSet {
    TickTimers,
    RecordInput,
    Update,
}

#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum FixedSet {
    Pre,
    Cur,
    Post,
}

// fn setup(mut commands: Commands) {
fn setup(mut settings: ResMut<bevy_framepace::FramepaceSettings>, mut commands: Commands) {
    settings.limiter = Limiter::from_framerate(60.);
    commands.spawn((
        Name::new("Camera"),
        Camera2dBundle {
            camera: Camera {
                clear_color: ClearColorConfig::Custom(COLOR_BACKGROUND),
                ..default()
            },
            ..default()
        },
        IsDefaultUiCamera,
    ));
}
