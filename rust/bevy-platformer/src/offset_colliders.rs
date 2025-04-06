use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use log::{log, Level};

#[derive(Component, Default)]
struct Wall;

#[derive(Clone, Default, Component)]
struct Walleable;

#[derive(Event)]
struct WallCollisionEvent;

fn detect_wall_collision(
    mut collisions: EventReader<CollisionEvent>,
    mut writer: EventWriter<WallCollisionEvent>,
    walls: Query<Entity, With<Wall>>,
    walleables: Query<&Walleable>,
) {
    for collision in collisions.read() {
        log!(Level::Error, "CollisionEvent DETECTED");
        match collision {
            CollisionEvent::Started(collider_a, collider_b, _) => {
                if let (Ok(_), Ok(_)) = (walleables.get(*collider_a), wallss.get(*collider_b)) {
                    writer.send(WallCollisionEvent);
                }
                if let (Ok(_), Ok(_)) = (wallss.get(*collider_a), walleables.get(*collider_b)) {
                    writer.send(WallCollisionEvent);
                };
            }
            CollisionEvent::Stopped(_, _, _) => (),
        }
    }
}

fn resolve_collision(
    mut reader: EventReader<WallCollisionEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for collision in reader.read() {
        log!(Level::Error, "WallCollisionEvent DETECTED");
        commands.spawn((
            Mesh2d(meshes.add(Circle::new(50.0))),
            MeshMaterial2d(materials.add(Color::hsl(360. * 10 as f32 / 10 as f32, 0.95, 0.7))),
            RigidBody::Dynamic,
            GlobalTransform::default(),
            GravityScale(10.),
            ActiveEvents::COLLISION_EVENTS,
        ));
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    log!(Level::Error, "Setting up!");
    commands.spawn((Name::new("Camera"), Camera2d, IsDefaultUiCamera));

    commands
        .spawn((
            Mesh2d(meshes.add(Circle::new(50.0))),
            MeshMaterial2d(materials.add(Color::hsl(360. * 1 as f32 / 10 as f32, 0.95, 0.7))),
            RigidBody::Dynamic,
            GlobalTransform::default(),
            GravityScale(10.),
        ))
        .with_children(|children| {
            children
                .spawn(Collider::ball(25.))
                .insert(Transform::from_xyz(0.0, -25.0, -1.0))
                .insert(Walleable);
        });

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(300., 20.))),
        MeshMaterial2d(materials.add(Color::hsl(360. * 5 as f32 / 10 as f32, 0.95, 0.7))),
        Collider::cuboid(150., 10.),
        Transform::from_xyz(0.0, -250.0, -1.0),
        RigidBody::Fixed,
        GlobalTransform::default(),
        ActiveEvents::COLLISION_EVENTS,
        Wall,
    ));
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Window {
                    title: "Bevy Platformer".to_string(),
                    canvas: Some("#bevy".to_string()),
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: true,
                    ..default()
                }
                .into(),
                ..default()
            }),
        )
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_event::<WallCollisionEvent>()
        .add_systems(Startup, setup)
        .add_systems(Update, (detect_wall_collision, resolve_collision).chain())
        .run();
}
