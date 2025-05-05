use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tile {
    pub q: i32,
    pub r: i32,
}

impl Tile {
    pub fn new(q: i32, r: i32) -> Self {
        Self { q, r }
    }

    pub fn bordering_edges(&self) -> [Edge; 4] {
        [
            Edge::new(self.q, self.r, EdgeDirection::N), // North edge of this tile
            Edge::new(self.q, self.r, EdgeDirection::W), // West edge of this tile
            Edge::new(self.q, self.r + 1, EdgeDirection::N), // South edge (= North edge of tile below)
            Edge::new(self.q + 1, self.r, EdgeDirection::W), // East edge (= West edge of tile to right)
        ]
    }

    pub fn neighbors(&self) -> [Tile; 4] {
        [
            Tile::new(self.q, self.r - 1), // North neighbour
            Tile::new(self.q - 1, self.r), // West neighbour
            Tile::new(self.q, self.r + 1), // South neighbour
            Tile::new(self.q + 1, self.r), // East neighbour
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EdgeDirection {
    N,
    W,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Edge {
    pub q: i32,
    pub r: i32,
    pub edge_direction: EdgeDirection,
}

impl Edge {
    pub fn new(q: i32, r: i32, edge_direction: EdgeDirection) -> Self {
        Self {
            q,
            r,
            edge_direction,
        }
    }

    pub fn joining_tiles(&self) -> [Tile; 2] {
        match self.edge_direction {
            EdgeDirection::N => [
                Tile::new(self.q, self.r - 1), // Tile above the edge
                Tile::new(self.q, self.r),     // Tile below the edge
            ],
            EdgeDirection::W => [
                Tile::new(self.q - 1, self.r), // Tile to the left of the edge
                Tile::new(self.q, self.r),     // Tile to the right of the edge
            ],
        }
    }
}

pub struct Grid {
    tiles: HashSet<Tile>,
    edges: HashSet<Edge>,
    tile_data: HashMap<Tile, TileData>,
    edge_data: HashMap<Edge, EdgeData>,
}

#[derive(Debug, Clone)]
pub struct TileData {
    pub tile_type: TileType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileType {
    Blue,
    Red,
    Black,
}

#[derive(Debug, Clone)]
pub struct EdgeData {
    pub edge_type: EdgeType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeType {
    Empty,
    Full,
    Dotted,
}

impl Grid {
    pub fn new() -> Self {
        Self {
            tiles: HashSet::new(),
            edges: HashSet::new(),
            tile_data: HashMap::new(),
            edge_data: HashMap::new(),
        }
    }

    pub fn new_sized(width: i32, height: i32) -> Self {
        let mut grid = Self::new();
        for x in 0..height {
            for y in 0..width {
                grid.add_tile(
                    Tile::new(x, y),
                    TileData {
                        tile_type: TileType::Black,
                    },
                );
                grid.add_edge(
                    Edge::new(x, y, EdgeDirection::N),
                    EdgeData {
                        edge_type: EdgeType::Empty,
                    },
                );
                grid.add_edge(
                    Edge::new(x, y, EdgeDirection::W),
                    EdgeData {
                        edge_type: EdgeType::Empty,
                    },
                );
            }
        }
        for x in 0..height {
            grid.add_edge(
                Edge::new(x, width, EdgeDirection::N),
                EdgeData {
                    edge_type: EdgeType::Full,
                },
            );
        }
        for y in 0..width {
            grid.add_edge(
                Edge::new(height, y, EdgeDirection::W),
                EdgeData {
                    edge_type: EdgeType::Full,
                },
            );
        }

        return grid;
    }

    pub fn tiles(&self) -> &HashSet<Tile> {
        &self.tiles
    }

    pub fn edges(&self) -> &HashSet<Edge> {
        &self.edges
    }

    pub fn add_tile(&mut self, tile: Tile, data: TileData) -> &mut Self {
        self.tiles.insert(tile);
        self.tile_data.insert(tile, data);
        self
    }

    pub fn add_edge(&mut self, edge: Edge, data: EdgeData) -> &mut Self {
        self.edges.insert(edge);
        self.edge_data.insert(edge, data);
        self
    }

    pub fn tile_exists(&self, tile: &Tile) -> bool {
        self.tiles.contains(tile)
    }

    pub fn edge_exists(&self, edge: &Edge) -> bool {
        self.edges.contains(edge)
    }

    pub fn get_tile_data(&self, tile: &Tile) -> Option<&TileData> {
        self.tile_data.get(tile)
    }

    pub fn get_edge_data(&self, edge: &Edge) -> Option<&EdgeData> {
        self.edge_data.get(edge)
    }

    pub fn get_tile_data_mut(&mut self, tile: &Tile) -> Option<&mut TileData> {
        self.tile_data.get_mut(tile)
    }

    pub fn get_edge_data_mut(&mut self, edge: &Edge) -> Option<&mut EdgeData> {
        self.edge_data.get_mut(edge)
    }

    pub fn get_tile_edges(&self, tile: &Tile) -> Vec<Edge> {
        tile.bordering_edges()
            .into_iter()
            .filter(|edge| self.edge_exists(edge))
            .collect()
    }

    pub fn get_connected_tiles(&self, tile: &Tile) -> Vec<Tile> {
        self.get_tile_edges(tile)
            .iter()
            .flat_map(|edge| edge.joining_tiles())
            .filter(|t| t != tile && self.tile_exists(t))
            .collect()
    }
}

pub fn example() {
    let mut grid = Grid::new();

    grid.add_tile(
        Tile::new(0, 0),
        TileData {
            tile_type: TileType::Blue,
        },
    );

    grid.add_tile(
        Tile::new(0, 1),
        TileData {
            tile_type: TileType::Red,
        },
    );

    grid.add_edge(
        Edge::new(0, 1, EdgeDirection::N),
        EdgeData {
            edge_type: EdgeType::Empty,
        },
    );

    if let Some(tile_data) = grid.get_tile_data(&Tile::new(0, 0)) {
        println!("Tile at (0,0) is {:?}", tile_data.tile_type);
    }

    let connected = grid.get_connected_tiles(&Tile::new(0, 0));
    println!("Tiles connected to (0,0): {:?}", connected);
}
