mod colorscheme;
mod debug;
mod limited_queue;

use crate::debug::*;
use crate::limited_queue::LimitedQueue;
use rand::seq::SliceRandom;
use std::collections::HashSet;

use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, window::WindowResolution};
use bevy_framepace::Limiter;

const COLOR_BACKGROUND: Color = Color::Srgba(colorscheme::BASE);
const COLOR_SNAKE_HEAD: Color = Color::Srgba(colorscheme::TEXT);
const COLOR_SNAKE_TAIL: Color = Color::Srgba(colorscheme::SUBTEXT0);
const COLOR_FOOD: Color = Color::Srgba(colorscheme::PEACH);

// fn gcd(mut a: f32, mut b: f32) -> f32 {
//     while b != 0. {
//         let t = b;
//         b = a % b;
//         a = t;
//     }
//     a
// }
// const GRID_WIDTH: i32 = (RESOLUTION_WIDTH / gdc_resolution).floor() as i32;
// const GRID_HEIGHT: i32 = (RESOLUTION_HEIGHT / gdc_resolution).floor() as i32;

// TODO use SystemParam for this
pub const RESOLUTION_WIDTH: f32 = 2880.;
pub const RESOLUTION_HEIGHT: f32 = 1800.;
// pub const RESOLUTION_WIDTH: f32 = 1920.;
// pub const RESOLUTION_HEIGHT: f32 = 1080.;
pub const RESOLUTION_GCD: i32 = 120;

// const GRID_WIDTH: i32 = (RESOLUTION_WIDTH / RESOLUTION_GCD).floor() as i32;
// const GRID_HEIGHT: i32 = (RESOLUTION_HEIGHT / RESOLUTION_GCD).floor() as i32;
const GRID_WIDTH: i32 = 16;
const GRID_HEIGHT: i32 = 9;

#[derive(Component)]
struct Head {
    direction: Direction,
}

#[derive(Component)]
struct Tail;

#[derive(Resource, Default)]
struct TailSegments(Vec<Entity>);

#[derive(Resource)]
struct InputBuffer(LimitedQueue<Direction>);

impl Default for InputBuffer {
    fn default() -> Self {
        InputBuffer(LimitedQueue::new(3))
    }
}

#[derive(Component)]
struct Food;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Size {
    width: f32,
    height: f32,
}
impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }
}

#[derive(Event)]
struct GrowthEvent;

#[derive(Resource, Default)]
struct LastTailPosition(Option<Position>);

#[derive(Event)]
struct GameOverEvent;

#[derive(Event)]
struct GamePauseEvent;

#[derive(Resource, Default)]
struct Score(usize);

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "bevy-game".into(),
                    resolution: WindowResolution::new(RESOLUTION_WIDTH, RESOLUTION_HEIGHT)
                        .with_scale_factor_override(1.),
                    resizable: false,
                    ..default()
                }),
                ..default()
            }),
            FrameTimeDiagnosticsPlugin,
        ))
        .add_plugins(bevy_framepace::FramepacePlugin)
        .insert_resource(Msaa::Off)
        .insert_resource(Time::<Fixed>::from_seconds(0.08))
        .insert_resource(TailSegments::default())
        .insert_resource(LastTailPosition::default())
        .insert_resource(InputBuffer::default())
        .insert_resource(Score::default())
        .add_event::<GrowthEvent>()
        .add_event::<GameOverEvent>()
        .add_event::<GamePauseEvent>()
        .add_systems(Startup, (setup, spawn_snake, debug_setup))
        .add_systems(
            FixedUpdate,
            (
                snake_eating,
                snake_movement.before(snake_eating),
                snake_growth.after(snake_eating),
                game_over.after(snake_movement),
            ),
        )
        .add_systems(Update, (debug_handler, input_handler, food_spawner, toggle_pause))
        .add_systems(PostUpdate, (size_scaling, position_translation))
        .run();
}

fn setup(mut settings: ResMut<bevy_framepace::FramepaceSettings>, mut commands: Commands) {
    settings.limiter = Limiter::from_framerate(60.);
    commands.spawn(Camera2dBundle {
        camera: Camera {
            clear_color: ClearColorConfig::Custom(COLOR_BACKGROUND),
            ..default()
        },
        ..default()
    });
}

fn spawn_snake(mut commands: Commands, mut segments: ResMut<TailSegments>) {
    *segments = TailSegments(vec![
        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: COLOR_SNAKE_HEAD,
                    ..default()
                },
                ..default()
            })
            .insert(Head {
                direction: Direction::Up,
            })
            .insert(Tail)
            .insert(Position { x: 3, y: 3 })
            .insert(Size::square(0.8))
            .id(),
        spawn_segment(commands, Position { x: 3, y: 2 }),
    ]);
}

fn input_handler(
    input: Res<ButtonInput<KeyCode>>,
    mut input_buffer: ResMut<InputBuffer>,
    heads: Query<&Head, With<Head>>,
    mut game_pause_writer: EventWriter<GamePauseEvent>,
) {
    if input.just_pressed(KeyCode::Space) {
        game_pause_writer.send(GamePauseEvent);
    }
    if let Some(head) = heads.iter().next() {
        let pressed_direction = if input.pressed(KeyCode::ArrowLeft)
            || input.pressed(KeyCode::KeyA)
            || input.pressed(KeyCode::KeyH)
        {
            Some(Direction::Left)
        } else if input.pressed(KeyCode::ArrowDown)
            || input.pressed(KeyCode::KeyS)
            || input.pressed(KeyCode::KeyJ)
        {
            Some(Direction::Down)
        } else if input.pressed(KeyCode::ArrowUp)
            || input.pressed(KeyCode::KeyW)
            || input.pressed(KeyCode::KeyK)
        {
            Some(Direction::Up)
        } else if input.pressed(KeyCode::ArrowRight)
            || input.pressed(KeyCode::KeyD)
            || input.pressed(KeyCode::KeyL)
        {
            Some(Direction::Right)
        } else {
            None
        };

        if let Some(direction) = pressed_direction {
            if !input_buffer.0.contains(direction) {
                if input_buffer.0.len() == 0 {
                    if direction != head.direction && direction != head.direction.opposite() {
                        input_buffer.0.push(direction);
                    }
                } else {
                    input_buffer.0.push(direction);
                }
            }
        }
    }
}

fn toggle_pause(
    mut game_pause_reader: EventReader<GamePauseEvent>,
    mut time: ResMut<Time<Virtual>>,
) {
    if game_pause_reader.read().next().is_some() {
        if time.is_paused() {
            time.unpause();
        } else {
            time.pause();
        }
    }
}

fn size_scaling(mut q: Query<(&Size, &mut Transform)>) {
    for (sprite_size, mut transform) in q.iter_mut() {
        transform.scale = Vec3::new(
            sprite_size.width / GRID_WIDTH as f32 * RESOLUTION_WIDTH,
            sprite_size.height / GRID_HEIGHT as f32 * RESOLUTION_HEIGHT,
            1.0,
        );
    }
}

fn position_translation(mut q: Query<(&Position, &mut Transform)>) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, RESOLUTION_WIDTH, GRID_WIDTH as f32),
            convert(pos.y as f32, RESOLUTION_HEIGHT, GRID_HEIGHT as f32),
            0.0,
        );
    }
}

fn food_spawner(
    food_query: Query<&Food>,
    segments: ResMut<TailSegments>,
    mut commands: Commands,
    heads: Query<&Position, With<Head>>,
    positions: Query<&Position, With<Tail>>,
) {
    let food_count = food_query.into_iter().count();
    if food_count < 5 {
        if let Some(head_position) = heads.iter().next() {
            let all_pos: HashSet<(i32, i32)> = (0..GRID_HEIGHT)
                .flat_map(|y| (0..GRID_WIDTH).map(move |x| (x, y)))
                .collect();
            let head_pos: HashSet<(i32, i32)> = HashSet::from([(head_position.x, head_position.y)]);
            let tail_pos: HashSet<(i32, i32)> = segments
                .0
                .iter()
                .map(|e| *positions.get(*e).unwrap())
                .collect::<Vec<Position>>()
                .iter()
                .map(|pos| (pos.x, pos.y))
                .collect();
            let valid_pos: Vec<(i32, i32)> = all_pos
                .difference(&head_pos)
                .cloned()
                .collect::<HashSet<(i32, i32)>>()
                .difference(&tail_pos)
                .cloned()
                .collect();

            for random_position in
                valid_pos.choose_multiple(&mut rand::thread_rng(), 5 - food_count)
            {
                commands
                    .spawn(SpriteBundle {
                        sprite: Sprite {
                            color: COLOR_FOOD,
                            ..default()
                        },
                        ..default()
                    })
                    .insert(Food)
                    .insert(Position {
                        x: random_position.0,
                        y: random_position.1,
                    })
                    .insert(Size::square(0.5));
            }
        }
    }
}

fn snake_movement(
    segments: ResMut<TailSegments>,
    mut input_buffer: ResMut<InputBuffer>,
    mut heads: Query<(Entity, &mut Head)>,
    mut positions: Query<&mut Position>,
    mut last_tail_position: ResMut<LastTailPosition>,
    mut game_over_writer: EventWriter<GameOverEvent>,
) {
    if let Some((head_entity, mut head)) = heads.iter_mut().next() {
        let segment_positions = segments
            .0
            .iter()
            .map(|e| *positions.get_mut(*e).unwrap())
            .collect::<Vec<Position>>();
        let mut head_pos = positions.get_mut(head_entity).unwrap();
        head.direction = input_buffer.0.pop().unwrap_or(head.direction);
        match &head.direction {
            Direction::Left => {
                head_pos.x -= 1;
            }
            Direction::Right => {
                head_pos.x += 1;
            }
            Direction::Up => {
                head_pos.y += 1;
            }
            Direction::Down => {
                head_pos.y -= 1;
            }
        };
        if head_pos.x < 0 || head_pos.y < 0 || head_pos.x >= GRID_WIDTH || head_pos.y >= GRID_HEIGHT
        {
            game_over_writer.send(GameOverEvent);
        }
        if segment_positions.contains(&head_pos) {
            game_over_writer.send(GameOverEvent);
        }
        segment_positions
            .iter()
            .zip(segments.0.iter().skip(1))
            .for_each(|(pos, segment)| {
                *positions.get_mut(*segment).unwrap() = *pos;
            });
        *last_tail_position = LastTailPosition(Some(*segment_positions.last().unwrap()));
    }
}

fn spawn_segment(mut commands: Commands, position: Position) -> Entity {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: COLOR_SNAKE_TAIL,
                ..default()
            },
            ..default()
        })
        .insert(Tail)
        .insert(position)
        .insert(Size::square(0.65))
        .id()
}

fn snake_eating(
    mut commands: Commands,
    mut growth_writer: EventWriter<GrowthEvent>,
    food_positions: Query<(Entity, &Position), With<Food>>,
    head_positions: Query<&Position, With<Head>>,
) {
    for head_pos in head_positions.iter() {
        for (ent, food_pos) in food_positions.iter() {
            if food_pos == head_pos {
                commands.entity(ent).despawn();
                growth_writer.send(GrowthEvent);
            }
        }
    }
}

fn snake_growth(
    commands: Commands,
    last_tail_position: Res<LastTailPosition>,
    mut segments: ResMut<TailSegments>,
    mut growth_reader: EventReader<GrowthEvent>,
    mut score: ResMut<Score>,
) {
    if growth_reader.read().next().is_some() {
        segments
            .0
            .push(spawn_segment(commands, last_tail_position.0.unwrap()));
        score.0 += 1;
    }
}

fn game_over(
    mut commands: Commands,
    mut reader: EventReader<GameOverEvent>,
    segments_res: ResMut<TailSegments>,
    food: Query<Entity, With<Food>>,
    segments: Query<Entity, With<Tail>>,
    mut score: ResMut<Score>,
) {
    if reader.read().next().is_some() {
        for ent in food.iter().chain(segments.iter()) {
            commands.entity(ent).despawn();
        }
        score.0 = 0;
        spawn_snake(commands, segments_res);
    }
}
