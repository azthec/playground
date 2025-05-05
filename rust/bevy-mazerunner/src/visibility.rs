use std::collections::LinkedList;

use crate::config;
use crate::game::level::{Edge, EdgeOrientation, LevelLoadedEvent};
use crate::game::player::PlayerMovedEvent;
use crate::materials::gradient::GradientMaterial;
use bevy::app::Update;
use bevy::asset::Assets;
use bevy::color::Color;
use bevy::core::Name;
use bevy::prelude::{
    App, Commands, Component, Entity, Mesh, Mesh2d, Plugin, Query, Res, ResMut, Resource,
    Transform, Triangle2d, Vec2,
};
use bevy::prelude::{EventReader, With};
use bevy::sprite::MeshMaterial2d;
use bevy_visibility::map::*;
use bevy_visibility::structs::*;

pub struct VisibilityPlugin;

impl Plugin for VisibilityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (init_map, update_visibility));
    }
}

#[derive(Resource)]
pub struct MapResource {
    pub map: Map,
    pub precalculated_segments: LinkedList<Segment>,
}

impl MapResource {
    fn from_rects(rectangles: LinkedList<Rectangle>) -> Self {
        let map = Map::new(2000., 0., LinkedList::new(), rectangles);
        let precalculated_segments = map.load_map();
        return MapResource {
            map,
            precalculated_segments,
        };
    }
}

fn init_map(
    mut commands: Commands,
    mut reader: EventReader<LevelLoadedEvent>,
    edges: Query<(&Transform, &Edge)>,
) {
    if let Some(_) = reader.read().next() {
        let mut rectangles = LinkedList::new();

        for (transform, edge) in edges.iter() {
            let (width, height) = if edge.orientation == EdgeOrientation::Vertical {
                (config::LINE_WIDTH, config::TILE_SIZE as f32)
            } else {
                (config::TILE_SIZE as f32, config::LINE_WIDTH)
            };

            let translation = transform.translation;

            rectangles.push_back(Rectangle {
                x: translation.x,
                y: translation.y,
                width,
                height,
            });
        }

        commands.insert_resource(MapResource::from_rects(rectangles));
    }
}

#[derive(Component)]
struct Triangle;

fn update_visibility(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut gradient_materials: ResMut<Assets<GradientMaterial>>,
    mut reader: EventReader<PlayerMovedEvent>,
    triangle_query: Query<Entity, With<Triangle>>,
    maybe_map_resource: Option<Res<MapResource>>,
) {
    if let Some(map_resource) = maybe_map_resource {
        if let Some(event) = reader.read().next() {
            triangle_query.iter().for_each(|entity| {
                commands.entity(entity).despawn();
            });

            let (x, y) = (event.x, event.y);
            let precalculated_position_segments =
                map_resource
                    .map
                    .set_light_location(&map_resource.precalculated_segments, x, y);

            let output = map_resource
                .map
                .calculate_visibility_cleaned(Point::new(x, y), precalculated_position_segments);

            // draw the visibility triangles
            for (p1, p2) in output {
                spawn_triangle(
                    &mut commands,
                    &mut meshes,
                    &mut gradient_materials,
                    Vec2::new(p1.x, p1.y),
                    Vec2::new(p2.x, p2.y),
                    Vec2::new(x, y),
                );
            }
        }
    }
}

fn spawn_triangle(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    gradient_materials: &mut ResMut<Assets<GradientMaterial>>,
    p1: Vec2,
    p2: Vec2,
    origin: Vec2,
) {
    commands.spawn((
        Name::new("Triangle"),
        Triangle,
        Mesh2d(meshes.add(Triangle2d::new(p1, p2, origin))),
        MeshMaterial2d(gradient_materials.add(GradientMaterial {
            color: Color::hsl(12., 0.3, 0.7).to_linear(),
            origin,
            max_distance: 250.
        })),
    ));
}
