use bevy::prelude::*;
use bevy_snake::AppPlugin;

fn main() -> AppExit {
    App::new().add_plugins(AppPlugin).run()
}
