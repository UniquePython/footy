use macroquad::color::{Color, WHITE};
use macroquad::prelude::color_u8;

use super::window::PIXELS_PER_METER;

pub(crate) const PITCH_WIDTH: f32 = 105.0; // meters
pub(crate) const PITCH_HEIGHT: f32 = 70.0; // meters

pub(crate) const HALF_PITCH_WIDTH: f32 = PITCH_WIDTH / 2.0; // meters
pub(crate) const HALF_PITCH_HEIGHT: f32 = PITCH_HEIGHT / 2.0; // meters

pub(crate) const PENALTY_AREA_HEIGHT: f32 = 40.32;
pub(crate) const PENALTY_AREA_WIDTH: f32 = 16.5;
pub(crate) const GOAL_AREA_HEIGHT: f32 = 18.32;
pub(crate) const GOAL_AREA_WIDTH: f32 = 5.5;
pub(crate) const PENALTY_SPOT_DISTANCE: f32 = 11.0;

pub(crate) const CENTER_CIRCLE_RADIUS: f32 = 9.15; // meters
pub(crate) const SPOT_RADIUS: f32 = 0.15; // meters

pub(crate) const CORNER_ARC_RADIUS: f32 = 1.0; // meters
pub(crate) const PENALTY_ARC_RADIUS: f32 = CENTER_CIRCLE_RADIUS; // meters
pub(crate) const PENALTY_ARC_HALF_ANGLE: f32 = 53.0518; // degrees, derived from box geometry
pub(crate) const PENALTY_ARC_SWEEP: f32 = 2.0 * PENALTY_ARC_HALF_ANGLE; // degrees

pub(crate) const PITCH_COLOR: Color = color_u8!(126, 175, 52, 255);
pub(crate) const MARKING_COLOR: Color = WHITE;
pub(crate) const MARKING_THICKNESS: f32 = 0.12 * PIXELS_PER_METER; // pixels
