use bevy::{math::Vec3, prelude::Component};


#[derive(Debug, Component, Default)]
pub struct Velocity(pub Vec3);
impl Velocity {
    pub fn new(velocity: Vec3) -> Self {
        Self(Vec3::new(velocity.x, velocity.y, 0.0).normalize() * velocity.length())
    }
}
