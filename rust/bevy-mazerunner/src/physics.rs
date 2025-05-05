use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0));
        app.add_plugins(RapierDebugRenderPlugin::default().disabled());
        app.add_systems(Startup, setup);
        app.add_systems(Update, toggle_debug);
    }
}

pub fn setup(mut rapier_config: Query<&mut RapierConfiguration>) {
    rapier_config.single_mut().gravity = Vec2::new(0.0, -2000.0);
}

pub fn toggle_debug(input: Res<ButtonInput<KeyCode>>, mut ctx: ResMut<DebugRenderContext>) {
    if input.just_pressed(KeyCode::Backquote) {
        ctx.enabled = !ctx.enabled;
    }
}
