use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::physics::{colliders::ColliderBundle, ground::GroundDetection};

use super::{animation::PlayerAnimation, movement::MovementController};

pub(super) fn plugin(app: &mut App) {
    app.register_ldtk_entity::<PlayerBundle>("Player");
    app.register_ldtk_entity::<GoalBundle>("Goal");
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Player;

#[derive(Default, Bundle, LdtkEntity)]
struct PlayerBundle {
    // TODO how can the asset server be used here instead
    #[sprite_sheet("images/alien-beige.png", 18, 18, 6, 2, 0, 0, 0)]
    sprite_sheet: Sprite,
    player: Player,
    #[from_entity_instance]
    pub collider_bundle: ColliderBundle,
    controller: MovementController,
    ground_detection: GroundDetection,
    animation: PlayerAnimation,
}

#[derive(Default, Bundle, LdtkEntity)]
struct GoalBundle {
    #[sprite_sheet]
    sprite_sheet: Sprite,
}
