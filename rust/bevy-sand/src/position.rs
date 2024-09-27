use std::{
    hash::{Hash, Hasher},
    ops::{Add, Sub},
};

use bevy::prelude::Component;

#[derive(Component, Copy, Clone)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    fn distance(&self, other: &Position) -> f32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Position {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

// note that it compares floored floats
impl Hash for Position {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.x.floor().to_bits(), self.y.floor().to_bits()).hash(state);
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        (self.x - other.x).abs() < f32::EPSILON && (self.y - other.y).abs() < f32::EPSILON
    }
}

impl Eq for Position {}
