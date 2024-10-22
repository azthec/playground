use bevy::prelude::*;
use bevy_snake::AppPlugin;

fn main() -> AppExit {
    App::new().add_plugins(AppPlugin).run()
}

// use crate::debug::*;
// use crate::LimitedQueue;
// use rand::seq::SliceRandom;
// use std::collections::HashSet;
//
// use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, window::WindowResolution};
//
// fn main() {
//     App::new()
//         .add_plugins((
//             DefaultPlugins.set(WindowPlugin {
//                 primary_window: Some(Window {
//                     title: "bevy-game".into(),
//                     resolution: WindowResolution::new(RESOLUTION_WIDTH, RESOLUTION_HEIGHT)
//                         .with_scale_factor_override(1.),
//                     resizable: false,
//                     ..default()
//                 }),
//                 ..default()
//             }),
//             FrameTimeDiagnosticsPlugin,
//         ))
//         .insert_resource(Msaa::Off)
//         .insert_resource(Time::<Fixed>::from_seconds(0.08))
//         .insert_resource(LastTailPosition::default())
//         .insert_resource(InputBuffer::default())
//         .insert_resource(Score::default())
//         .add_event::<GrowthEvent>()
//         .add_event::<GameOverEvent>()
//         .add_event::<GamePauseEvent>()
//         .add_systems(Startup, (spawn_snake))
//         .add_systems(
//             FixedUpdate,
//             (
//                 snake_eating,
//                 snake_movement.before(snake_eating),
//                 snake_growth.after(snake_eating),
//                 game_over.after(snake_movement),
//             ),
//         )
//         .add_systems(
//             Update,
//             (input_handler, food_spawner, toggle_pause),
//         )
//         .add_systems(
//              PostUpdate,
//              (size_scaling, position_translation)
//         .run();
// }

