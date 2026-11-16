use macroquad::math::Vec2;

use crate::constants::{BALL_MASS, BALL_PHYSICS_RADIUS};

#[derive(Debug, Clone, Copy)]
pub(crate) struct Ball {
    pub(crate) position: Vec2, // meters, world space
    pub(crate) velocity: Vec2, // meters / sec
    pub(crate) radius: f32,    // meters
    pub(crate) mass: f32,      // kg
}

impl Ball {
    pub(crate) fn new(position: Vec2) -> Self {
        Self {
            position,
            velocity: Vec2::ZERO,
            radius: BALL_PHYSICS_RADIUS,
            mass: BALL_MASS,
        }
    }
}
