use std::usize;

#[derive(Clone, PartialEq, Debug)]
pub enum EdgeType {
    Empty,
    Solid,
    Dotted,
}

#[derive(Clone)]
pub struct Tile {
    pub south_edge: Option<EdgeType>,
    pub west_edge: Option<EdgeType>,
}

impl Default for Tile {
    fn default() -> Self {
        Tile {
            south_edge: None,
            west_edge: None,
        }
    }
}

impl Tile {
    pub fn new(south_edge: Option<EdgeType>, west_edge: Option<EdgeType>) -> Self {
        Tile {
            south_edge,
            west_edge,
        }
    }
}

pub struct Grid {
    pub rows: usize,
    pub columns: usize,
    pub tiles: Vec<Vec<Tile>>,
}

impl Grid {
    pub fn new(rows: usize, columns: usize) -> Self {
        Self {
            rows,
            columns,
            tiles: vec![vec![Tile::default(); columns]; rows],
        }
    }

    pub fn from_tiles(tiles: Vec<Vec<Tile>>) -> Self {
        let rows = tiles.len();
        let columns = tiles.first().map_or(0, |row| row.len());
        Self {
            rows,
            columns,
            tiles,
        }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&Tile> {
        self.tiles.get(row)?.get(col)
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut Tile> {
        self.tiles.get_mut(row)?.get_mut(col)
    }

    pub fn neighbour_south(&self, row: usize, col: usize) -> Option<(usize, usize)> {
        if row > 0 && row < self.rows && col < self.columns {
            Some((row - 1, col))
        } else {
            None
        }
    }

    pub fn neighbour_east(&self, row: usize, col: usize) -> Option<(usize, usize)> {
        if row < self.rows && col + 1 < self.columns {
            Some((row, col + 1))
        } else {
            None
        }
    }

    pub fn neighbour_north(&self, row: usize, col: usize) -> Option<(usize, usize)> {
        if row + 1 < self.rows && col < self.columns {
            Some((row + 1, col))
        } else {
            None
        }
    }

    pub fn neighbour_west(&self, row: usize, col: usize) -> Option<(usize, usize)> {
        if row < self.rows && col > 0 {
            Some((row, col - 1))
        } else {
            None
        }
    }

    pub fn reachable_south(&self, row: usize, col: usize) -> bool {
        self.get(row, col)
            .and_then(|tile| tile.south_edge.as_ref())
            .is_none_or(|edge_type| edge_type == &EdgeType::Empty)
    }

    pub fn reachable_east(&self, row: usize, col: usize) -> bool {
        self.neighbour_east(row, col)
            .is_some_and(|(r, c)| self.reachable_west(r, c))
    }
    pub fn reachable_north(&self, row: usize, col: usize) -> bool {
        self.neighbour_north(row, col)
            .is_some_and(|(r, c)| self.reachable_south(r, c))
    }

    pub fn reachable_west(&self, row: usize, col: usize) -> bool {
        self.get(row, col)
            .and_then(|tile| tile.west_edge.as_ref())
            .is_none_or(|edge_type| edge_type == &EdgeType::Empty)
    }
}
