use crate::GRID_HEIGHT;
use crate::GRID_WIDTH;
use bevy::prelude::*;
use rand::prelude::SliceRandom;
use rand::prelude::*;

use crate::{
    game::{
        grid::{Position, Size},
        snake::Head,
    },
    AppSet, COLOR_PORTAL,
};

#[derive(Component)]
struct Portal {
    to: Position,
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn);
    app.add_systems(FixedUpdate, (teleport).in_set(AppSet::PreUpdate));
}

fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let pos1 = Position {
        x: thread_rng().gen_range(0..GRID_WIDTH),
        y: thread_rng().gen_range(0..GRID_HEIGHT),
    };
    let pos2 = Position {
        x: thread_rng().gen_range(0..GRID_WIDTH),
        y: thread_rng().gen_range(0..GRID_HEIGHT),
    };
    commands
        // .spawn(SpriteBundle {
        //     sprite: Sprite {
        //         color: COLOR_PORTAL,
        //         ..default()
        //     },
        //     ..default()
        // })
        .spawn(PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            material: materials.add(COLOR_PORTAL),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        })
        .insert(Portal { to: pos1 })
        .insert(pos2)
        .insert(Size::square(0.5));
    commands
        // .spawn(SpriteBundle {
        //     sprite: Sprite {
        //         color: COLOR_PORTAL,
        //         ..default()
        //     },
        //     ..default()
        // })
        .spawn(PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            material: materials.add(COLOR_PORTAL),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        })
        .insert(Portal { to: pos2 })
        .insert(pos1)
        .insert(Size::square(0.5));
}

fn teleport(
    portals: Query<(&Portal, &Position), Without<Head>>,
    mut heads: Query<(Entity, &mut Position), With<Head>>,
) {
    if let Some((_, mut position)) = heads.iter_mut().next() {
        for (portal, pos) in portals.iter() {
            if position.x == pos.x && position.y == pos.y {
                position.x = portal.to.x;
                position.y = portal.to.y;
                break;
            }
        }
    }
}
