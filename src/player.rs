use macroquad::math::Vec2;

use crate::attributes::Attributes;
use crate::constants::PLAYER_PHYSICS_RADIUS;
use crate::controller::Controller;

pub(crate) struct Player {
    pub(crate) position: Vec2,
    pub(crate) velocity: Vec2,
    pub(crate) radius: f32,
    pub(crate) attributes: Attributes,
    pub(crate) controller: Box<dyn Controller>,
}

impl Player {
    pub(crate) fn new(
        position: Vec2,
        attributes: Attributes,
        controller: Box<dyn Controller>,
    ) -> Self {
        Self {
            position,
            velocity: Vec2::ZERO,
            radius: PLAYER_PHYSICS_RADIUS,
            attributes,
            controller,
        }
    }

    pub(crate) fn update(&mut self, dt: f32) {
        let target = self.controller.desired_direction() * self.attributes.pace_value();
        let diff = target - self.velocity;
        let delta = self.attributes.acceleration_value() * dt;

        if diff.length() < delta {
            self.velocity = target;
        } else {
            self.velocity += diff.normalize_or_zero() * delta;
        }

        self.position += self.velocity * dt;
    }
}
