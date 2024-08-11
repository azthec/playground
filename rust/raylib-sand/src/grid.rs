use std::borrow::Cow;

use crate::*;

// consider defining a copy on write matrix and using that
type Matrix<'a, T> = Vec<Vec<Cow<'a, T>>>;

#[derive(Clone, Copy)]
pub struct Cell {
    color: u8,
}

impl Cell {
    pub fn empty() -> Self {
        Self { color: 0 }
    }
    pub fn is_empty(&self) -> bool {
        self.color == 0
    }
}

// consider implementing a sparse grid
#[derive(Clone)]
pub struct Grid {
    cells: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
    cell_size: usize,
}

impl Grid {
    // consider keeping a reference to a single empty cell object
    pub fn new(width: usize, height: usize, cell_size: usize) -> Self {
        Self {
            cells: vec![vec![Cell::empty(); height]; width],
            width,
            height,
            cell_size,
        }
    }
    pub fn update(&mut self, rl: &RaylibHandle, clicked: Option<(usize, usize)>) -> &mut Grid {
        if let Some((x, y)) = clicked {
            self.cells[x][y].color += 5;
        }
        for col in (0..self.height - 1).rev() {
            for row in 0..self.width - 2 {
                if !self.cells[row][col].is_empty() {
                    if (self.cells[row][col + 1].is_empty()) {
                        self.cells[row][col + 1] = self.cells[row][col];
                        self.cells[row][col] = Cell::empty();
                    }
                }
            }
        }
        self
    }

    pub fn draw(&self, mut d: RaylibDrawHandle) {
        for row in 0..self.width {
            for col in 0..self.height {
                let cell = self.cells[row][col];
                if !cell.is_empty() {
                    match cell.color % 3 {
                        0 => d.draw_rectangle(
                            (row * self.cell_size) as i32,
                            (col * self.cell_size) as i32,
                            self.cell_size as i32,
                            self.cell_size as i32,
                            DEEP_SEA_BLUE,
                        ),
                        1 => d.draw_rectangle(
                            (row * self.cell_size) as i32,
                            (col * self.cell_size) as i32,
                            self.cell_size as i32,
                            self.cell_size as i32,
                            SOFT_CORAL,
                        ),
                        2 => d.draw_rectangle(
                            (row * self.cell_size) as i32,
                            (col * self.cell_size) as i32,
                            self.cell_size as i32,
                            self.cell_size as i32,
                            PEACH,
                        ),
                        _ => unreachable!(),
                    }
                }
            }
        }
    }
}
