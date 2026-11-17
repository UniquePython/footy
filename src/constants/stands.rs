use macroquad::color::Color;
use macroquad::prelude::color_u8;

use super::window::{PADDING_SIDELINE, PIXELS_PER_METER, WINDOW_HALF_WIDTH_METERS};

pub(crate) const STAND_GAP: f32 = 5.0; // meters
pub(crate) const STAND_DEPTH: f32 = (PADDING_SIDELINE / PIXELS_PER_METER) - STAND_GAP; // meters

pub(crate) const SEAT_PITCH: f32 = 0.4; // meters, center-to-center spacing
pub(crate) const SEAT_SIZE: f32 = 0.3; // meters, drawn size of each seat
pub(crate) const STAND_BG_COLOR: Color = color_u8!(75, 60, 60, 255);
pub(crate) const STAND_ROW_COUNT: usize = 4;
pub(crate) const ROW_DEPTH: f32 = STAND_DEPTH / STAND_ROW_COUNT as f32;
pub(crate) const SEATS_PER_ROW_DEPTH: usize = (ROW_DEPTH / SEAT_PITCH).floor() as usize;
pub(crate) const SEAT_COLUMNS: usize =
    ((2.0 * WINDOW_HALF_WIDTH_METERS) / SEAT_PITCH).floor() as usize;
pub(crate) const STAND_ROW_COLOR: &[Color; STAND_ROW_COUNT] = &[
    color_u8!(225, 15, 15, 255),
    color_u8!(195, 30, 30, 255),
    color_u8!(165, 45, 45, 255),
    color_u8!(135, 60, 60, 255),
];
