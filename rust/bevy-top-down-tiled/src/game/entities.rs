use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{lighting::PointLight2dBundle, physics::{colliders::ColliderBundle, spike::{Spike, Spikeable}, visibility::VisibilityRaySource}};

use super::movement::MovementController;

pub(super) fn plugin(app: &mut App) {
    app.register_ldtk_entity::<PlayerBundle>("Player");
    app.register_ldtk_entity::<SpikeBundle>("Spike");
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Player;

#[derive(Default, Bundle, LdtkEntity)]
struct PlayerBundle {
    #[sprite_sheet("images/characters.png", 16, 16, 1, 1, 0, 0, 0)]
    sprite_sheet: Sprite,
    player: Player,
    #[from_entity_instance]
    pub collider_bundle: ColliderBundle,
    controller: MovementController,
    spike_detection: Spikeable,
    ray_source: VisibilityRaySource,
    light_source: PointLight2dBundle
}

#[derive(Default, Bundle, LdtkEntity)]
struct SpikeBundle {
    spike: Spike,
    #[sprite_sheet]
    sprite_sheet: Sprite,
}
