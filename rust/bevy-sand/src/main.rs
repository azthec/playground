mod colorscheme;
mod components;
mod debug;
mod position;
mod sparse_grid;
mod walls;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::{
    math::{
        bounding::{Aabb2d, IntersectsVolume},
        VectorSpace,
    },
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_framepace::Limiter;
use components::{collider::Collider, particle::Particle, velocity::Velocity};
use debug::{debug_handler, debug_setup};
use position::Position;
use walls::{WallBundle, WallLocation};

pub const PARTICLE_SIZE: f32 = 12.0;

#[derive(Component)]
struct DebugText;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "bevy-game".into(),
                    resolution: (1920., 1080.).into(),
                    ..default()
                }),
                ..default()
            }),
            FrameTimeDiagnosticsPlugin,
        ))
        .add_plugins(bevy_framepace::FramepacePlugin)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                debug_handler,
                particle_spawn,
                particle_transform,
                particle_collisions,
                particle_despawn,
            ),
        )
        .run();
}

fn setup(
    mut settings: ResMut<bevy_framepace::FramepaceSettings>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    settings.limiter = Limiter::from_framerate(60.0);
    commands.spawn(Camera2dBundle::default());

    debug_setup(&mut commands);

    commands.spawn((MaterialMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(Circle::new(50.0))),
        material: materials.add(Color::Srgba(colorscheme::PALE_GOLD)),
        // transform: Transform::from_xyz(450., -200., 0.0),
        ..default()
    },));
    // .insert(Particle {})
    // .insert(Velocity::new(Vec3::ONE));
    commands.spawn(WallBundle::new(WallLocation::Left));
    commands.spawn(WallBundle::new(WallLocation::Right));
    commands.spawn(WallBundle::new(WallLocation::Bottom));
    commands.spawn(WallBundle::new(WallLocation::Top));
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
    (x, y)

    // place particles in a grid
    // let grid_x = (x / PARTICLE_SIZE).floor() * PARTICLE_SIZE + PARTICLE_SIZE / 2.;
    // let grid_y = (y / PARTICLE_SIZE).floor() * PARTICLE_SIZE + PARTICLE_SIZE / 2.;
    //
    // (grid_x, grid_y)
}

fn particle_spawn(
    mut commands: Commands,
    windows: Query<&Window>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
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
            let particle = commands
                .spawn((MaterialMesh2dBundle {
                    mesh: Mesh2dHandle(meshes.add(Rectangle::new(PARTICLE_SIZE, PARTICLE_SIZE))),
                    material: materials.add(Color::Srgba(colorscheme::PALE_GOLD)),
                    transform: Transform::from_xyz(position.x, position.y, 0.0),
                    ..default()
                },))
                .insert(Particle {})
                .insert(Velocity::new(Vec3::NEG_Y * 5.));
        }
    }
}

// TODO
fn particle_despawn() {}

fn particle_transform(mut particle_query: Query<(&Particle, &mut Velocity, &mut Transform)>) {
    particle_query
        .par_iter_mut()
        .for_each(|(particle, mut velocity, mut transform)| {
            // velocity.0 = velocity.0 * 1.05;
            transform.translation += velocity.0;
        });
}

/// Detects particle collisions and stops particles from moving further
/// Limitations:
///  - Particles moving fast can go through other particles without crossing bounding boxes
///  - There is no left or right alignment, as particles only fall downwards
fn particle_collisions(
    mut commands: Commands,
    mut particle_query: Query<(&mut Velocity, &mut Transform), (With<Particle>, Without<Collider>)>,
    collider_query: Query<(Entity, &Transform, Option<&Particle>), With<Collider>>,
    // mut collision_events: EventWriter<CollisionEvent>,
) {
    for (mut particle_velocity, mut particle_transform) in &mut particle_query {
        for (collider_entity, collider_transform, maybe_particle) in &collider_query {
            let overlap: Option<Vec2> = overlap(
                bounding_box(&particle_transform),
                bounding_box(collider_transform),
            );

            match overlap {
                Some(overlap) => {
                    if overlap.x < 0.0 {
                        particle_transform.translation.x -= overlap.x;
                    } else {
                        particle_transform.translation.x += overlap.x.abs();
                    }
                    if overlap.y < 0.0 {
                        particle_transform.translation.y -= overlap.y;
                    } else {
                        particle_transform.translation.y += overlap.y.abs();
                    }
                    particle_velocity.0 = Vec3::ZERO;
                }

                None => continue,
            }
        }
    }
}

fn bounding_box(transform: &Transform) -> Aabb2d {
    Aabb2d::new(transform.translation.truncate(), transform.scale.truncate())
}

fn intersects(this: Aabb2d, that: Aabb2d) -> bool {
    this.intersects(&that)
}
fn overlap(this: Aabb2d, that: Aabb2d) -> Option<Vec2> {
    if intersects(this, that) {
        let this_min: Vec2 = this.min;
        let this_max: Vec2 = this.max;
        let that_min: Vec2 = that.min;
        let that_max: Vec2 = that.max;

        let overlap_x = (this_max.x.min(that_max.x) - this_min.x.max(that_min.x)).max(0.0);
        let overlap_y = (this_max.y.min(that_max.y) - this_min.y.max(that_min.y)).max(0.0);

        if overlap_x > 0.0 && overlap_y > 0.0 {
            Some(Vec2::new(overlap_x, overlap_y))
        } else {
            None
        }
    } else {
        None
    }
}
