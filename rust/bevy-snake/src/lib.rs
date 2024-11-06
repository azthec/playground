mod colorscheme;
mod config;
mod game;
mod limited_queue;
mod types;
mod window;

use bevy::prelude::*;
use bevy_framepace::Limiter;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

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
        app.insert_resource(Time::<Fixed>::from_seconds(0.38)); // set fixed run time
        app.add_plugins(window::plugin);
        app.add_plugins(bevy_framepace::FramepacePlugin);

        app.add_plugins(game::plugin);

        app.add_plugins(WorldInspectorPlugin::new());
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

fn setup(
    mut settings: ResMut<bevy_framepace::FramepaceSettings>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    settings.limiter = Limiter::from_framerate(60.);

    commands.spawn((
        Name::new("PointLight"),
        PointLightBundle {
            point_light: PointLight {
                // shadows_enabled: true,
                intensity: 10_000_000.,
                range: 100.0,
                shadow_depth_bias: 0.2,
                ..default()
            },
            transform: Transform::from_xyz(8.0, 12.0, 1.0),
            ..default()
        },
    ));

    commands.spawn((
        Name::new("Plane"),
        PbrBundle {
            mesh: meshes.add(Rectangle::new(GRID_WIDTH as f32, GRID_HEIGHT as f32)),
            material: materials.add(COLOR_BACKGROUND),
            transform: Transform::from_rotation(Quat::from_rotation_x(
                -std::f32::consts::FRAC_PI_2,
            ))
            .with_translation(Vec3::new(
                GRID_WIDTH as f32 / 2. - 0.5,
                0.,
                GRID_HEIGHT as f32 / 2. - 0.5,
            )),
            ..default()
        },
    ));

    commands.spawn((
        Name::new("Camera"),
        Camera3dBundle {
            camera: Camera {
                clear_color: ClearColorConfig::Custom(COLOR_BACKGROUND),
                ..default()
            },
            transform: Transform::from_xyz(2.5 * 2., 4.5 * 2., 9.0 * 2.).looking_at(
                Vec3::new(GRID_WIDTH as f32 / 2., 0., GRID_HEIGHT as f32 / 2.),
                Vec3::Y,
            ),
            ..default()
        },
        IsDefaultUiCamera,
    ));
}
