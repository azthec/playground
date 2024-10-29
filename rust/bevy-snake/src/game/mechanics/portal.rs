use bevy::prelude::*;

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

fn spawn(mut commands: Commands) {
    let pos1 = Position { x: 2, y: 4 };
    let pos2 = Position { x: 7, y: 7 };
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: COLOR_PORTAL,
                ..default()
            },
            ..default()
        })
        .insert(Portal { to: pos1 })
        .insert(pos2)
        .insert(Size::square(0.5));
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: COLOR_PORTAL,
                ..default()
            },
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
