use std::f32::consts::PI;

use macroquad::color::Color;
use macroquad::prelude::color_u8;

use super::physics::GRAVITY;
use super::window::PIXELS_PER_METER;

pub(crate) const BALL_PHYSICS_RADIUS: f32 = 0.11; // meters
pub(crate) const BALL_RENDER_RADIUS: f32 = 1.0 * BALL_PHYSICS_RADIUS * PIXELS_PER_METER; // pixels
pub(crate) const BALL_MASS: f32 = 0.43; // kg
pub(crate) const BALL_COLOR: Color = color_u8!(240, 255, 0, 255);

pub(crate) const BALL_FRICTION_COEFFICIENT: f32 = 0.08;
pub(crate) const BALL_ROLLING_DECELERATION: f32 = BALL_FRICTION_COEFFICIENT * GRAVITY; // m/s^2

// Aerodynamics
pub(crate) const BALL_DIAMETER: f32 = 2.0 * BALL_PHYSICS_RADIUS; // meters
pub(crate) const BALL_CROSS_SECTION_AREA: f32 = PI * BALL_PHYSICS_RADIUS * BALL_PHYSICS_RADIUS; // m^2

// Drag crisis: Cd falls from the subcritical to the supercritical value as Reynolds number
// rises through the transition. Approximate values for a football; exact numbers vary by ball.
pub(crate) const CD_SUBCRITICAL: f32 = 0.45;
pub(crate) const CD_SUPERCRITICAL: f32 = 0.20;
pub(crate) const RE_CRISIS_START: f32 = 1.5e5;
pub(crate) const RE_CRISIS_END: f32 = 3.5e5;
