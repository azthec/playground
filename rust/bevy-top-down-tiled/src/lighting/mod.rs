use bevy::prelude::*;
use bevy_lit::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(Lighting2dPlugin);
}

#[derive(Clone, Bundle)]
pub struct CameraLightingBundle {
    pub lighting_settings: Lighting2dSettings,
}

impl Default for CameraLightingBundle {
    fn default() -> Self {
        CameraLightingBundle {
            lighting_settings: Lighting2dSettings::default(),
        }
    }
}

#[derive(Clone, Bundle)]
pub struct PointLight2dBundle {
    pub point_light: PointLight2d,
}

impl Default for PointLight2dBundle {
    fn default() -> Self {
        PointLight2dBundle {
            point_light: PointLight2d {
                color: Color::WHITE,
                intensity: 5.0,
                radius: 50.0,
                falloff: 2.0,
                ..default()
            },
        }
    }
}

#[derive(Clone, Debug, Bundle)]
pub struct LightOccluder2dBundle {
    pub light_occluder: LightOccluder2d,
}

impl Default for LightOccluder2dBundle {
    fn default() -> Self {
        LightOccluder2dBundle {
            light_occluder: LightOccluder2d::default(),
        }
    }
}
