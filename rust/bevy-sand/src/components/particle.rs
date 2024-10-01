use bevy::prelude::Component;

#[derive(Debug, Component)]
pub struct Particle {}

impl Default for Particle {
    fn default() -> Self {
        Self {}
    }
}

