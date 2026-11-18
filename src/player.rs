use macroquad::math::Vec2;

use crate::attributes::Attributes;
use crate::constants::{
    FACING_RETURN_RATE, FACING_TURN_RATE, MAX_FACING_OFFSET, PLAYER_PHYSICS_RADIUS,
};
use crate::controller::Controller;
use crate::facing_turn::FacingTurn;

pub(crate) struct Player {
    pub(crate) position: Vec2,
    pub(crate) velocity: Vec2,
    pub(crate) radius: f32,
    pub(crate) attributes: Attributes,
    pub(crate) controller: Box<dyn Controller>,

    // Direction the player last moved in, world space, normalized. Facing is
    // expressed relative to this, so it needs a value even before the player
    // has ever moved; see `facing_reference` for that default.
    last_movement_direction: Vec2,
    // Angle (radians) of facing relative to `last_movement_direction`. Positive
    // rotates counter-clockwise (left), negative clockwise (right), matching
    // Vec2::rotate's convention.
    facing_offset: f32,
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
            last_movement_direction: Self::facing_reference(),
            facing_offset: 0.0,
        }
    }

    // Default "forward" before the player has ever moved. Eventually this
    // should point toward the opponent's goal; -Y stands in until goals exist.
    fn facing_reference() -> Vec2 {
        -Vec2::Y
    }

    // World-space direction the player is currently facing/aiming.
    pub(crate) fn facing(&self) -> Vec2 {
        self.last_movement_direction
            .rotate(Vec2::from_angle(self.facing_offset))
    }

    pub(crate) fn update(&mut self, dt: f32) {
        let desired_direction = self.controller.desired_direction();

        let target = desired_direction * self.attributes.pace_value();
        let diff = target - self.velocity;
        let delta = self.attributes.acceleration_value() * dt;

        if diff.length() < delta {
            self.velocity = target;
        } else {
            self.velocity += diff.normalize_or_zero() * delta;
        }

        self.position += self.velocity * dt;

        if desired_direction != Vec2::ZERO {
            self.last_movement_direction = desired_direction;
        }

        self.update_facing_offset(dt);
    }

    fn update_facing_offset(&mut self, dt: f32) {
        match self.controller.facing_turn() {
            FacingTurn::Left => {
                self.facing_offset =
                    (self.facing_offset + FACING_TURN_RATE * dt).min(MAX_FACING_OFFSET);
            }
            FacingTurn::Right => {
                self.facing_offset =
                    (self.facing_offset - FACING_TURN_RATE * dt).max(-MAX_FACING_OFFSET);
            }
            FacingTurn::None => {
                let step = FACING_RETURN_RATE * dt;
                if self.facing_offset.abs() <= step {
                    self.facing_offset = 0.0;
                } else {
                    self.facing_offset -= step * self.facing_offset.signum();
                }
            }
        }
    }
}
