mod colorscheme;
mod position;
use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_framepace::Limiter;
use position::Position;
use std::collections::HashMap;

pub const PARTICLE_SIZE: f32 = 12.0;

#[derive(Resource)]
struct Grid(HashMap<Position, Entity>);

impl Default for Grid {
    fn default() -> Self {
        Grid(HashMap::new())
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "bevy-game".into(),
                resolution: (1920., 1080.).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(bevy_framepace::FramepacePlugin)
        .init_resource::<Grid>()
        .add_systems(Startup, setup)
        .add_systems(Update, mouse_handler)
        .add_systems(Update, physics_system)
        .run();
}

#[derive(Component)]
struct Particle;

#[derive(Component)]
struct Speed;

fn setup(
    mut settings: ResMut<bevy_framepace::FramepaceSettings>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    settings.limiter = Limiter::from_framerate(60.0);
    commands.spawn(Camera2dBundle::default());
    commands.spawn((MaterialMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(Circle::new(50.0))),
        material: materials.add(Color::Srgba(colorscheme::PALE_GOLD)),
        // transform: Transform::from_xyz(450., -200., 0.0),
        ..default()
    },));
}

fn mouse_handler(
    mut commands: Commands,
    windows: Query<&Window>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut grid: ResMut<Grid>,
) {
    let window = windows.single();

    if let Some(cursor_position) = window.cursor_position() {
        if mouse_button_input.pressed(MouseButton::Left) {
            let (x, y) = cursor_to_grid_position(
                cursor_position.x,
                cursor_position.y,
                window.width(),
                window.height(),
            );
            let position = Position { x, y };
            if !grid.0.contains_key(&position) {
                let particle = commands
                    .spawn((MaterialMesh2dBundle {
                        mesh: Mesh2dHandle(
                            meshes.add(Rectangle::new(PARTICLE_SIZE, PARTICLE_SIZE)),
                        ),
                        material: materials.add(Color::Srgba(colorscheme::PALE_GOLD)),
                        transform: Transform::from_xyz(position.x, position.y, 0.0),
                        ..default()
                    },))
                    .insert(Particle)
                    .insert(Speed)
                    .insert(position)
                    .id();

                grid.0.insert(position.clone(), particle);
            }
        }
    }
}

fn cursor_to_grid_position(
    cursor_x: f32,
    cursor_y: f32,
    window_width: f32,
    window_height: f32,
) -> (f32, f32) {
    // from cursor_position coordinates to camera projection coordinates
    let x = cursor_x - window_width / 2.;
    let y = -cursor_y + window_height / 2.;

    // place particles in a grid
    let grid_x = (x / PARTICLE_SIZE).floor() * PARTICLE_SIZE;
    let grid_y = (y / PARTICLE_SIZE).floor() * PARTICLE_SIZE;

    (grid_x, grid_y)
}

fn physics_system(
    mut commands: Commands,
    windows: Query<&Window>,
    mut particle_query: Query<(Entity, &mut Transform), (With<Particle>, With<Speed>)>,
    stopped_query: Query<&Transform, (With<Position>, With<Particle>, Without<Speed>)>,
    mut grid: ResMut<Grid>,
) {
    let window = windows.single();
    let floor_y = -window.height() / 2.0;

    for (entity, mut transform) in particle_query.iter_mut() {
        let mut stop = false;

        if transform.translation.y <= floor_y {
            stop = true;
        }

        for stopped_transform in stopped_query.iter() {
            if (transform.translation.y - stopped_transform.translation.y).abs() < PARTICLE_SIZE
                && (transform.translation.x - stopped_transform.translation.x).abs() < PARTICLE_SIZE
            {
                stop = true;
                break;
            }
        }

        if stop {
            commands.entity(entity).remove::<Speed>();
        } else {
            let position = Position {
                x: transform.translation.x,
                y: transform.translation.y,
            };
            let entity = grid.0.remove(&position).unwrap();
            grid.0.insert(position - Position { x: 0., y: 6. }, entity);
            transform.translation.y -= 6.0; // Move down
        }
    }
}
