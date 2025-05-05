use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::config;

use super::enemies::RespawnEnemies;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerMovedEvent>();
        app.init_resource::<MovementController>();
        app.add_systems(Startup, init);
        app.add_systems(
            Update,
            (
                register_movement,
                register_movement_gamepad,
                update_movement,
                send_movement,
            )
                .chain(),
        );
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Event)]
pub struct PlayerMovedEvent {
    pub x: f32,
    pub y: f32,
}

fn init(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut writer: EventWriter<PlayerMovedEvent>,
) {
    let spawn = 50.;
    commands.spawn((
        Name::new("Player"),
        Player,
        Mesh2d(meshes.add(Circle::new(config::ENTITY_SIZE))),
        MeshMaterial2d(materials.add(Color::hsl(12., 0.95, 0.7))),
        Transform::from_xyz(spawn, spawn, 2.),
        Collider::ball(config::ENTITY_SIZE),
        RigidBody::KinematicPositionBased,
        KinematicCharacterController::default(),
        GravityScale(0.),
        Ccd::enabled(),
        ActiveEvents::COLLISION_EVENTS,
        Sensor,
    ));
    writer.send(PlayerMovedEvent { x: spawn, y: spawn });
}

#[derive(Resource, Default)]
pub struct MovementController {
    pub intents: Vec<PlayerInput>,
}

#[derive(Debug)]
pub enum PlayerInput {
    Up,
    Left,
    Down,
    Right,
}

fn register_movement(
    input: Res<ButtonInput<KeyCode>>,
    mut movement_controller: ResMut<MovementController>,
    mut commands: Commands,
) {
    let mut intents = Vec::new();
    if input.pressed(KeyCode::KeyW) || input.pressed(KeyCode::ArrowUp) {
        intents.push(PlayerInput::Up);
    }
    if input.pressed(KeyCode::KeyS) || input.pressed(KeyCode::ArrowDown) {
        intents.push(PlayerInput::Down);
    }
    if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
        intents.push(PlayerInput::Left);
    }
    if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
        intents.push(PlayerInput::Right);
    }
    if input.pressed(KeyCode::KeyR) {
        // TODO improve this logic / wrap it up and publish to itch.io
        commands.queue(RespawnEnemies);
    }

    movement_controller.intents = intents;
}

fn register_movement_gamepad(
    gamepads: Query<(Entity, &Gamepad)>,
    mut movement_controller: ResMut<MovementController>,
) {
    for (_entity, gamepad) in &gamepads {
        if gamepad.pressed(GamepadButton::DPadUp) {
            movement_controller.intents.push(PlayerInput::Up);
        }
        if gamepad.pressed(GamepadButton::DPadDown) {
            movement_controller.intents.push(PlayerInput::Down);
        }
        if gamepad.pressed(GamepadButton::DPadLeft) {
            movement_controller.intents.push(PlayerInput::Left);
        }
        if gamepad.pressed(GamepadButton::DPadRight) {
            movement_controller.intents.push(PlayerInput::Right);
        }
    }
}

fn update_movement(
    movement_controller: Res<MovementController>,
    timer: Res<Time>,
    mut controllers: Query<&mut KinematicCharacterController>,
) {
    if movement_controller.intents.is_empty() {
        return;
    }

    for mut controller in controllers.iter_mut() {
        let mut direction = Vec2::new(0., 0.);

        for intent in &movement_controller.intents {
            match intent {
                PlayerInput::Up => direction.y += config::PLAYER_SPEED * timer.delta_secs(),
                PlayerInput::Down => direction.y -= config::PLAYER_SPEED * timer.delta_secs(),
                PlayerInput::Right => direction.x += config::PLAYER_SPEED * timer.delta_secs(),
                PlayerInput::Left => direction.x -= config::PLAYER_SPEED * timer.delta_secs(),
            }
        }

        controller.translation = Some(direction);
    }
}

fn send_movement(
    mut query: Query<&Transform, (With<Player>, Changed<Transform>)>,
    mut writer: EventWriter<PlayerMovedEvent>,
) {
    for transform in query.iter_mut() {
        let x = transform.translation.x;
        let y = transform.translation.y;

        writer.send(PlayerMovedEvent { x, y });
    }
}
