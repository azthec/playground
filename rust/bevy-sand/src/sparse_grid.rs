use std::collections::HashMap;

use crate::position::Position;

pub struct SparseGrid<T> {
    data: HashMap<Position, T>,
}

impl<T> SparseGrid<T> {
    pub fn new() -> Self {
        SparseGrid {
            data: HashMap::new(),
        }
    }

    pub fn insert(&mut self, position: Position, value: T) {
        self.data.insert(position, value);
    }

    pub fn get(&self, position: &Position) -> Option<&T> {
        self.data.get(position)
    }

    pub fn contains(&self, position: &Position) -> bool {
        self.data.get(position).is_some()
    }

    pub fn remove(&mut self, position: &Position) -> Option<T> {
        self.data.remove(position)
    }
}
