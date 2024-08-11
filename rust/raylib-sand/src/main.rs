pub mod colorscheme;
pub mod game;
pub mod grid;

use crate::game::*;
use colorscheme::*;
use rand::prelude::*;
use raylib::prelude::*;

struct Settings {
    screen_width: usize,
    screen_height: usize,
    pixel_size: usize,
}

fn main() {
    let settings = Settings {
        screen_width: 1280,
        screen_height: 720,
        pixel_size: 8,
    };

    let (mut rl, thread) = raylib::init()
        .size(
            settings.screen_width.try_into().unwrap(),
            settings.screen_height.try_into().unwrap(),
        )
        .title("raylib-sand")
        .build();
    rl.set_target_fps(60);

    let mut game = Game::init(&settings);

    while !rl.window_should_close() {
        // TODO refactor this interact logic
        let mut clicked: Option<(usize, usize)> = None;
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            let mouse_position = rl.get_mouse_position();
            clicked = Some((
                mouse_position.x as usize / settings.pixel_size,
                mouse_position.y as usize / settings.pixel_size,
            ));
        }

        game = game.update(&rl, clicked);

        let mut d: RaylibDrawHandle = rl.begin_drawing(&thread);
        d.clear_background(PALE_LIME);
        game.draw(d);
    }
}
