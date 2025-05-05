use crate::materials::gradient::GradientMaterial;
use bevy_rapier2d::prelude::*;
use std::f32::consts::PI;

use bevy::{math::VectorSpace, prelude::*};

use crate::{
    config,
    simplegrid::{EdgeType, Grid},
};

use super::levels::level1;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LevelLoadedEvent>();
        app.init_resource::<Level>();
        app.add_systems(Startup, setup_level);
    }
}

#[allow(dead_code)]
#[derive(Component)]
pub struct Position {
    x: isize,
    y: isize,
}

#[derive(Component)]
pub struct Tile;

#[derive(Component)]
pub struct Edge {
    // only full / dotted for now
    // pub edge_type: bool,
    pub orientation: EdgeOrientation,
}

#[derive(PartialEq)]
pub enum EdgeOrientation {
    Vertical,
    Horizontal,
}

#[derive(Resource)]
pub struct Level {
    grid: Grid,
}

impl Default for Level {
    fn default() -> Self {
        Level {
            grid: Grid::from_tiles(level1::level1()),
        }
    }
}

#[derive(Event)]
pub struct LevelLoadedEvent;

fn setup_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut gradient_materials: ResMut<Assets<GradientMaterial>>,
    level: Res<Level>,
    mut writer: EventWriter<LevelLoadedEvent>,
) {
    for row in 0..level.grid.rows {
        for col in 0..level.grid.columns {
            if let Some(tile) = level.grid.get(row, col) {
                let color = config::COLOR_TILE;
                let tilesize = config::TILE_SIZE as f32;
                let x = col;
                let y = row;
                commands.spawn((
                    Name::new(format!("Tile({x},{y})")),
                    Mesh2d(meshes.add(Rectangle::new(tilesize, tilesize))),
                    MeshMaterial2d(materials.add(color)),
                    Transform::from_xyz(
                        (col as f32 * tilesize) + tilesize / 2.,
                        (row as f32 * tilesize) + tilesize / 2.,
                        -1.,
                    ),
                    Tile,
                    Position {
                        x: x as isize,
                        y: y as isize,
                    },
                ));

                if x == 0
                    || tile
                        .south_edge
                        .as_ref()
                        .is_some_and(|edge| edge != &EdgeType::Empty)
                {
                    spawn_edge(
                        &mut commands,
                        &mut meshes,
                        &mut gradient_materials,
                        x,
                        y,
                        EdgeOrientation::Vertical,
                    );
                }
                if y == 0
                    || tile
                        .west_edge
                        .as_ref()
                        .is_some_and(|edge| edge != &EdgeType::Empty)
                {
                    spawn_edge(
                        &mut commands,
                        &mut meshes,
                        &mut gradient_materials,
                        x,
                        y,
                        EdgeOrientation::Horizontal,
                    );
                }
                if x == level.grid.columns - 1 {
                    spawn_edge(
                        &mut commands,
                        &mut meshes,
                        &mut gradient_materials,
                        x + 1,
                        y,
                        EdgeOrientation::Vertical,
                    );
                }
                if y == level.grid.rows - 1 {
                    spawn_edge(
                        &mut commands,
                        &mut meshes,
                        &mut gradient_materials,
                        x,
                        y + 1,
                        EdgeOrientation::Horizontal,
                    );
                }
            }
        }
    }
    writer.send(LevelLoadedEvent);
}

fn spawn_edge(
    commands: &mut Commands<'_, '_>,
    meshes: &mut ResMut<'_, Assets<Mesh>>,
    gradient_materials: &mut ResMut<'_, Assets<GradientMaterial>>,
    x: usize,
    y: usize,
    orientation: EdgeOrientation,
) {
    const LINE_WIDTH: f32 = config::LINE_WIDTH;
    let color = config::COLOR_EDGE;
    let tilesize = config::TILE_SIZE as f32;

    let (transform_x, transform_y, rotation) = if orientation == EdgeOrientation::Vertical {
        (
            x as f32 * tilesize,
            y as f32 * tilesize + tilesize / 2.,
            PI / 2.,
        )
    } else {
        (x as f32 * tilesize + tilesize / 2., y as f32 * tilesize, 0.)
    };

    commands.spawn((
        Name::new(format!("Edge({x},{y})")),
        Mesh2d(meshes.add(Rectangle::new(tilesize, LINE_WIDTH))),
        MeshMaterial2d(gradient_materials.add(GradientMaterial {
            color: color.to_linear(),
            origin: Vec2::ZERO,
            max_distance: 200.
        })),
        Transform::from_xyz(transform_x, transform_y, 0.)
            .with_rotation(Quat::from_rotation_z(rotation)),
        Edge { orientation },
        Position {
            x: x as isize,
            y: y as isize,
        },
        Collider::cuboid(tilesize / 2., LINE_WIDTH / 2.),
        RigidBody::Fixed,
        LockedAxes::ROTATION_LOCKED,
    ));
}
