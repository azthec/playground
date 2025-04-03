//! Spawn the main level.

use bevy::prelude::*;
use bevy_ecs_ldtk::{
    assets::{LdtkProject, LevelIndices},
    LdtkPlugin, LdtkWorldBundle, LevelIid, LevelSelection, Respawn,
};

use crate::{asset_tracking::LoadResource, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(LdtkPlugin);
    app.register_type::<Level>();
    app.load_resource::<LevelAssets>();
}

// TODO rework
#[derive(Resource)]
pub struct CurrentLevel {
    level: Level,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Level {
    id: usize,
}

impl Level {
    pub const ONE: Level = Level { id: 0 };
    pub const TWO: Level = Level { id: 1 };
    pub const THREE: Level = Level { id: 2 };
}

#[derive(Debug)]
pub struct SpawnLevel {
    pub level: Level,
}

impl Command for SpawnLevel {
    fn apply(self, world: &mut World) {
        let level_assets = world.get_resource::<LevelAssets>();
        if let Some(level_assets) = level_assets {
            world.spawn((
                LdtkWorldBundle {
                    ldtk_handle: level_assets.ldtk_project.clone().into(),
                    ..Default::default()
                },
                StateScoped(Screen::Gameplay),
            ));
            world.insert_resource(CurrentLevel { level: self.level });
            world.insert_resource(LevelSelection::index(self.level.id));
        }
    }
}

#[derive(Debug)]
// TODO rename
pub struct SpawnNextLevel;

impl Command for SpawnNextLevel {
    fn apply(self, world: &mut World) {
        let level = world.get_resource::<CurrentLevel>();
        if let Some(level) = level {
            let new_level = Level {
                id: level.level.id + 1,
            };
            world.remove_resource::<CurrentLevel>();
            world.insert_resource(CurrentLevel { level: new_level });
            world.remove_resource::<LevelSelection>();
            world.insert_resource(LevelSelection::index(new_level.id));
        }
    }
}

#[derive(Debug)]
pub struct RespawnLevel;

impl Command for RespawnLevel {
    fn apply(self, world: &mut World) {
        let mut query = world.query_filtered::<Entity, With<LevelIid>>();
        let entity: Entity = query.single(&world);
        world.commands().entity(entity).insert(Respawn);
    }
}

#[derive(Resource, Asset, Reflect, Clone)]
pub struct LevelAssets {
    #[dependency]
    pub ldtk_project: Handle<LdtkProject>,
    #[dependency]
    pub audio_steps: Vec<Handle<AudioSource>>,
}

impl LevelAssets {
    pub const PATH_LTDK_PROJECT: &'static str = "platformer.ldtk";
    pub const PATH_STEP_1: &'static str = "audio/sound_effects/step1.ogg";
    pub const PATH_STEP_2: &'static str = "audio/sound_effects/step2.ogg";
    pub const PATH_STEP_3: &'static str = "audio/sound_effects/step3.ogg";
    pub const PATH_STEP_4: &'static str = "audio/sound_effects/step4.ogg";
}

impl FromWorld for LevelAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            ldtk_project: assets.load(LevelAssets::PATH_LTDK_PROJECT),
            audio_steps: vec![
                assets.load(LevelAssets::PATH_STEP_1),
                assets.load(LevelAssets::PATH_STEP_2),
                assets.load(LevelAssets::PATH_STEP_3),
                assets.load(LevelAssets::PATH_STEP_4),
            ],
        }
    }
}
