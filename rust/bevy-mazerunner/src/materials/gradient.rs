use bevy::{
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{AlphaMode2d, Material2d, Material2dPlugin},
};

use crate::game::player::PlayerMovedEvent;

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct GradientMaterial {
    #[uniform(0)]
    pub color: LinearRgba,
    #[uniform(1)]
    pub origin: Vec2,
    #[uniform(2)]
    pub max_distance: f32,

}

const SHADER_ASSET_PATH: &str = "shaders/gradient_material.wgsl";

impl Material2d for GradientMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}

pub struct GradientMaterialPlugin;

impl Plugin for GradientMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<GradientMaterial>::default());
        app.add_systems(Update, update_origin);
    }
}

fn update_origin(
    mut reader: EventReader<PlayerMovedEvent>,
    mut materials: ResMut<Assets<GradientMaterial>>,
) {
    if let Some(event) = reader.read().next() {
        for (_handle, material) in materials.iter_mut() {
            material.origin = Vec2::new(event.x, event.y);
        }
    }
}
