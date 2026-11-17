use macroquad::math::Vec2;

use crate::aerodynamics::drag_acceleration;
use crate::constants::{BALL_MASS, BALL_PHYSICS_RADIUS, BALL_ROLLING_DECELERATION};
use crate::environment::Environment;

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

    pub(crate) fn update(&mut self, dt: f32, env: &Environment) {
        if self.velocity == Vec2::ZERO {
            return;
        }

        // Air drag acts on the velocity relative to the air
        let drag = drag_acceleration(self.velocity - env.wind, env);
        self.velocity += drag * dt;

        // Rolling friction opposes ground-relative velocity and can only stop the ball,
        // never reverse it
        let old_velocity = self.velocity;
        let friction_dir = -self.velocity.normalize_or_zero();
        self.velocity += friction_dir * BALL_ROLLING_DECELERATION * dt;

        if self.velocity.dot(old_velocity) < 0.0 {
            self.velocity = Vec2::ZERO;
        }

        self.position += self.velocity * dt;
    }
}
