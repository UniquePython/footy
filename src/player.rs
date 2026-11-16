use macroquad::math::Vec2;

use crate::attributes::Attributes;
use crate::constants::PLAYER_RADIUS;

pub(crate) struct Player {
    pub(crate) position: Vec2,
    pub(crate) velocity: Vec2,
    pub(crate) radius: f32,
    pub(crate) attributes: Attributes,
}

impl Player {
    pub(crate) fn new(position: Vec2, attributes: Attributes) -> Self {
        Self {
            position,
            velocity: Vec2::ZERO,
            radius: PLAYER_RADIUS,
            attributes,
        }
    }

    pub(crate) fn update(&mut self, towards: Vec2, dt: f32) {
        let target = towards * self.attributes.pace_value();
        let diff = target - self.velocity;
        let delta = self.attributes.acceleration_value() * dt;

        if diff.length() < delta {
            self.velocity = target;
        } else {
            self.velocity += diff.normalize_or_zero() * delta;
        }
    }
}
