use raylib::drawing::RaylibDrawHandle;
use raylib::RaylibHandle;

use crate::grid::Grid;
use crate::Settings;

#[derive(Clone, Copy)]
pub enum GameState {
    Playing,
}

pub struct Game {
    state: GameState,
    grid: Grid,
}

impl Game {
    pub fn init(settings: &Settings) -> Game {
        Game {
            state: GameState::Playing,
            grid: Grid::new(
                settings.screen_width / settings.pixel_size,
                settings.screen_height / settings.pixel_size,
                settings.pixel_size,
            ),
        }
    }

    pub fn update(&mut self, rl: &RaylibHandle, clicked: Option<(usize, usize)>) -> Game {
        Game {
            state: self.state,
            grid: self.grid.update(rl, clicked).clone(),
        }
    }

    pub fn draw(&self, d: RaylibDrawHandle) -> () {
        match self.state {
            GameState::Playing => self.grid.draw(d),
        }
    }
}
