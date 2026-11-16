use macroquad::math::Vec2;

use crate::constants::{BALL_MASS, BALL_PHYSICS_RADIUS, BALL_ROLLING_DECELERATION};

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

    pub(crate) fn update(&mut self, dt: f32) {
        if self.velocity == Vec2::ZERO {
            return;
        }

        // calculate retardation
        let friction_dir = -self.velocity.normalize_or_zero();
        let retardation = friction_dir * BALL_ROLLING_DECELERATION;

        let old_velocity = self.velocity;

        // semi-implicit euler
        self.velocity += retardation * dt;

        // check for ball starting to reverse instead of just stopping and staying still
        if self.velocity.dot(old_velocity) < 0.0 {
            self.velocity = Vec2::ZERO;
        }

        self.position += self.velocity * dt;
    }
}
