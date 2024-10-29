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
            (
                AppSet::Setup,
                AppSet::PreUpdate,
                AppSet::Update,
                AppSet::PostUpdate,
                AppSet::Cleanup,
            )
                .chain(),
        );
        app.configure_sets(
            FixedUpdate,
            (
                AppSet::Setup,
                AppSet::PreUpdate,
                AppSet::Update,
                AppSet::PostUpdate,
                AppSet::Cleanup,
            )
                .chain(),
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
    Setup,
    PreUpdate,
    Update,
    PostUpdate,
    Cleanup,
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
