mod colorscheme;

use bevy::{prelude::*, window::WindowResolution};
use bevy_framepace::Limiter;
use rand::random;

const COLOR_BACKGROUND: Color = Color::Srgba(colorscheme::DEEP_SEA_BLUE);
const COLOR_SNAKE_HEAD: Color = Color::Srgba(colorscheme::PEACH);
const COLOR_SNAKE_TAIL: Color = Color::Srgba(colorscheme::PALE_LIME);
const COLOR_FOOD: Color = Color::Srgba(colorscheme::CHESTNUT_RED);

pub const RESOLUTION_WIDTH: f32 = 2880.;
pub const RESOLUTION_HEIGHT: f32 = 1800.;

const GRID_WIDTH: i32 = 20;
const GRID_HEIGHT: i32 = 20;
const GRID_SIZE: i32 = 20;

#[derive(Component)]
struct Head {
    direction: Direction,
}

#[derive(Component)]
struct Tail;

#[derive(Resource, Default)]
struct TailSegments(Vec<Entity>);

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

#[derive(PartialEq, Copy, Clone)]
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

fn main() {
    App::new()
        .add_plugins((DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "bevy-game".into(),
                resolution: WindowResolution::new(RESOLUTION_WIDTH, RESOLUTION_HEIGHT)
                    .with_scale_factor_override(1.),
                resizable: false,
                ..default()
            }),
            ..default()
        }),))
        .add_plugins(bevy_framepace::FramepacePlugin)
        .insert_resource(ClearColor(COLOR_BACKGROUND))
        .insert_resource(Time::<Fixed>::from_seconds(0.20))
        .insert_resource(TailSegments::default())
        .insert_resource(LastTailPosition::default())
        .add_event::<GrowthEvent>()
        .add_event::<GameOverEvent>()
        .add_systems(Startup, (setup, spawn_snake))
        .add_systems(
            FixedUpdate,
            (
                snake_eating,
                snake_movement.before(snake_eating),
                snake_growth.after(snake_eating),
                game_over.after(snake_movement),
            ),
        )
        .add_systems(Update, (input_handler, food_spawner))
        .add_systems(PostUpdate, (size_scaling, position_translation))
        .run();
}

fn setup(mut settings: ResMut<bevy_framepace::FramepaceSettings>, mut commands: Commands) {
    settings.limiter = Limiter::from_framerate(60.);
    commands.spawn(Camera2dBundle::default());
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

fn input_handler(input: Res<ButtonInput<KeyCode>>, mut heads: Query<&mut Head, With<Head>>) {
    if let Some(mut head) = heads.iter_mut().next() {
        let dir: Direction = if input.pressed(KeyCode::ArrowLeft) {
            Direction::Left
        } else if input.pressed(KeyCode::ArrowDown) {
            Direction::Down
        } else if input.pressed(KeyCode::ArrowUp) {
            Direction::Up
        } else if input.pressed(KeyCode::ArrowRight) {
            Direction::Right
        } else {
            head.direction
        };
        if dir != head.direction.opposite() {
            head.direction = dir;
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
            convert(pos.y as f32, RESOLUTION_HEIGHT, GRID_SIZE as f32),
            0.0,
        );
    }
}

fn food_spawner(mut commands: Commands, food_query: Query<(&Food)>) {
    let food_count = food_query.into_iter().count();
    if food_count < 5 {
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
                x: (random::<f32>() * GRID_WIDTH as f32) as i32,
                y: (random::<f32>() * GRID_HEIGHT as f32) as i32,
            })
            .insert(Size::square(0.8));
    }
}

fn snake_movement(
    segments: ResMut<TailSegments>,
    mut heads: Query<(Entity, &Head)>,
    mut positions: Query<&mut Position>,
    mut last_tail_position: ResMut<LastTailPosition>,
    mut game_over_writer: EventWriter<GameOverEvent>,
) {
    if let Some((head_entity, head)) = heads.iter_mut().next() {
        let segment_positions = segments
            .0
            .iter()
            .map(|e| *positions.get_mut(*e).unwrap())
            .collect::<Vec<Position>>();
        let mut head_pos = positions.get_mut(head_entity).unwrap();
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
) {
    if growth_reader.read().next().is_some() {
        segments
            .0
            .push(spawn_segment(commands, last_tail_position.0.unwrap()));
    }
}

fn game_over(
    mut commands: Commands,
    mut reader: EventReader<GameOverEvent>,
    segments_res: ResMut<TailSegments>,
    food: Query<Entity, With<Food>>,
    segments: Query<Entity, With<Tail>>,
) {
    if reader.read().next().is_some() {
        for ent in food.iter().chain(segments.iter()) {
            commands.entity(ent).despawn();
        }
        spawn_snake(commands, segments_res);
    }
}
