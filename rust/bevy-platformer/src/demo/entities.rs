use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::physics::{colliders::ColliderBundle, ground::GroundDetection};

use super::movement::MovementController;

pub(super) fn plugin(app: &mut App) {
    app.register_ldtk_entity::<PlayerBundle>("Player");
    app.register_ldtk_entity::<GoalBundle>("Goal");
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Player;

#[derive(Default, Bundle, LdtkEntity)]
struct PlayerBundle {
    #[sprite_sheet]
    sprite_sheet: Sprite,
    player: Player,
    #[from_entity_instance]
    pub collider_bundle: ColliderBundle,
    controller: MovementController,
    ground_detection: GroundDetection,
    // animation: PlayerAnimation, // animation = PlayerAnimation::new();
}

#[derive(Default, Bundle, LdtkEntity)]
struct GoalBundle {
    #[sprite_sheet]
    sprite_sheet: Sprite,
}
