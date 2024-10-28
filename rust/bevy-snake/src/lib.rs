mod colorscheme;
mod config;
#[cfg(feature = "dev")]
mod debug;
mod game;
mod limited_queue;
mod types;
mod window;

use bevy::prelude::*;
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
        app.insert_resource(Time::<Fixed>::from_seconds(0.08)); // set fixed run time
        app.add_plugins(window::plugin);
        app.add_plugins(bevy_framepace::FramepacePlugin);

        app.add_plugins(game::plugin);

        // Enable dev tools for dev builds.
        #[cfg(feature = "dev")]
        app.add_plugins(debug::plugin);
    }
}

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
