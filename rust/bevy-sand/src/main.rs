mod colorscheme;
mod debug;
mod position;
mod size;
mod sparse_grid;
mod walls;

use bevy::prelude::*;
use bevy::sprite::Mesh2dHandle;
use bevy::window::WindowResolution;
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, sprite::MaterialMesh2dBundle};
use bevy_framepace::Limiter;
use debug::{debug_handler, debug_setup};
use position::Position;
use size::Size;

pub const RESOLUTION_WIDTH: f32 = 1920.;
pub const RESOLUTION_HEIGHT: f32 = 1080.;
pub const GRID_WIDTH: u32 = 192;
pub const GRID_HEIGHT: u32 = 108;

#[derive(Component)]
struct DebugText;

#[derive(Component)]
struct Particle;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "bevy-game".into(),
                    resolution: WindowResolution::new(RESOLUTION_WIDTH, RESOLUTION_HEIGHT),
                    // .with_scale_factor_override(1.),
                    resizable: false,
                    ..default()
                }),
                ..default()
            }),
            FrameTimeDiagnosticsPlugin,
        ))
        .add_plugins(bevy_framepace::FramepacePlugin)
        .add_systems(Startup, setup)
        .add_systems(PreUpdate, (despawn_particle))
        .add_systems(Update, (debug_handler, spawn_particle, input_handler))
        .add_systems(PostUpdate, (size_scaling, position_translation))
        .run();
}

fn setup(
    mut settings: ResMut<bevy_framepace::FramepaceSettings>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    settings.limiter = Limiter::from_framerate(15.);
    commands.spawn(Camera2dBundle::default());
    debug_setup(&mut commands);

    let _ = commands
        .spawn((MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(1., 1.))),
            material: materials.add(Color::Srgba(colorscheme::PALE_GOLD)),
            ..default()
        },))
        .insert(Particle)
        .insert(Position { x: 0, y: 0 })
        .insert(Size::square(1.0))
        .id();
}

fn spawn_particle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
}

fn despawn_particle(mut commands: Commands, particles: Query<(Entity, &Position), With<Particle>>) {
    for (entity, position) in particles.iter() {
        if position.y < 0 {
            commands.entity(entity).despawn();
        }
    }
}

fn input_handler(
    mut commands: Commands,
    windows: Query<&Window>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // This only works for square scales of 1.0
    fn convert(visual_pos: f32, bound_window: f32, bound_game: f32) -> i32 {
        let tile_size = bound_window / bound_game;
        ((visual_pos - (tile_size / 2.) + (bound_window / 2.)) * bound_game / bound_window).round()
            as i32
    }
    let window = windows.single();

    if let Some(cursor_position) = window.cursor_position() {
        if mouse_button_input.pressed(MouseButton::Left) {
            let x = convert(
                cursor_position.x - window.width() / 2.,
                window.width() as f32,
                GRID_WIDTH as f32,
            );
            let y = convert(
                -cursor_position.y + window.height() / 2.,
                window.height() as f32,
                GRID_HEIGHT as f32,
            );
            let _ = commands
                .spawn((MaterialMesh2dBundle {
                    mesh: Mesh2dHandle(meshes.add(Rectangle::new(1., 1.))),
                    material: materials.add(Color::Srgba(colorscheme::PALE_GOLD)),
                    ..default()
                },))
                .insert(Particle)
                .insert(Position { x, y })
                .insert(Size::square(1.0))
                .id();
        }
    }
}

fn size_scaling(windows: Query<&Window>, mut q: Query<(&Size, &mut Transform)>) {
    let window = windows.single();
    for (sprite_size, mut transform) in q.iter_mut() {
        transform.scale = Vec3::new(
            sprite_size.width / GRID_WIDTH as f32 * window.width() as f32,
            sprite_size.height / GRID_HEIGHT as f32 * window.height() as f32,
            1.0,
        );
    }
}

fn position_translation(windows: Query<&Window>, mut q: Query<(&mut Position, &mut Transform)>) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }
    let window = windows.single();
    for (mut pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width() as f32, GRID_WIDTH as f32),
            convert(pos.y as f32, window.height() as f32, GRID_HEIGHT as f32),
            0.0,
        );
        // TODO temporary debug, may move to own logic later
        pos.y -= 1;
    }
}
