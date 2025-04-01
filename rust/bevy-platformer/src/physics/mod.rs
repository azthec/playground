pub mod colliders;
pub mod ground;
pub mod spike;
mod walls;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::screens::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0));
    app.add_plugins(RapierDebugRenderPlugin::default().disabled());
    app.add_systems(Startup, setup);
    app.add_systems(Update, (toggle_debug).run_if(in_state(Screen::Gameplay)));
    app.add_plugins((walls::WallPlugin, ground::GroundDetectionPlugin, spike::SpikeDetectionPlugin));
}

pub fn setup(mut rapier_config: Query<&mut RapierConfiguration>) {
    rapier_config.single_mut().gravity = Vec2::new(0.0, -2000.0);
}

pub fn toggle_debug(input: Res<ButtonInput<KeyCode>>, mut ctx: ResMut<DebugRenderContext>) {
    if input.just_pressed(KeyCode::Backquote) {
        ctx.enabled = !ctx.enabled;
    }
}
