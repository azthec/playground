use mazerunner::AppPlugin;

use bevy::prelude::*;

fn main() -> AppExit {
    App::new().add_plugins(AppPlugin).run()
}

