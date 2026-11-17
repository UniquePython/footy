use macroquad::color::Color;
use macroquad::prelude::color_u8;

use super::pitch::{PITCH_HEIGHT, PITCH_WIDTH};

pub(crate) const PADDING_SIDELINE: f32 = 136.5; // pixels
pub(crate) const PADDING_GOAL_END: f32 = 60.0; // pixels
pub(crate) const PADDING_ALONG_X_AXIS: f32 = 2.0 * PADDING_GOAL_END; // pixels
pub(crate) const PADDING_ALONG_Y_AXIS: f32 = 2.0 * PADDING_SIDELINE; // pixels

pub(crate) const PIXELS_PER_METER: f32 = 10.5;

pub const WINDOW_WIDTH: f32 = (PITCH_WIDTH * PIXELS_PER_METER) + PADDING_ALONG_X_AXIS; // pixels
pub const WINDOW_HEIGHT: f32 = (PITCH_HEIGHT * PIXELS_PER_METER) + PADDING_ALONG_Y_AXIS; // pixels

pub(crate) const PITCH_DRAWING_WIDTH: f32 = WINDOW_WIDTH - PADDING_ALONG_X_AXIS; // pixels
pub(crate) const PITCH_DRAWING_HEIGHT: f32 = WINDOW_HEIGHT - PADDING_ALONG_Y_AXIS; // pixels

const CHECK_HORIZONTAL_PIXELS_PER_METER: f32 = PITCH_DRAWING_WIDTH / PITCH_WIDTH;
const CHECK_VERTICAL_PIXELS_PER_METER: f32 = PITCH_DRAWING_HEIGHT / PITCH_HEIGHT;
const _: () = assert!(CHECK_HORIZONTAL_PIXELS_PER_METER == PIXELS_PER_METER);
const _: () = assert!(CHECK_VERTICAL_PIXELS_PER_METER == PIXELS_PER_METER);

pub(crate) const BG_COLOR: Color = color_u8!(96, 145, 22, 255);
