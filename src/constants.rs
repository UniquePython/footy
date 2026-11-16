use macroquad::color::{Color, WHITE};

pub const WINDOW_WIDTH: f32 = 1380.0; // pixels
pub const WINDOW_HEIGHT: f32 = 960.0; // pixels

pub(crate) const PADDING_EACH_SIDE: f32 = 60.0; // pixels
pub(crate) const PADDING_ALONG_AXIS: f32 = 2.0 * PADDING_EACH_SIDE; // pixels

pub(crate) const PITCH_DRAWING_WIDTH: f32 = WINDOW_WIDTH - PADDING_ALONG_AXIS; // pixels
pub(crate) const PITCH_DRAWING_HEIGHT: f32 = WINDOW_HEIGHT - PADDING_ALONG_AXIS; // pixels

pub(crate) const WINDOW_CENTER_X: f32 = WINDOW_WIDTH / 2.0; // pixels
pub(crate) const WINDOW_CENTER_Y: f32 = WINDOW_HEIGHT / 2.0; // pixels

pub(crate) const PITCH_WIDTH: f32 = 105.0; // meters
pub(crate) const PITCH_HEIGHT: f32 = 70.0; // meters

pub(crate) const HALF_PITCH_WIDTH: f32 = PITCH_WIDTH / 2.0; // meters
pub(crate) const HALF_PITCH_HEIGHT: f32 = PITCH_HEIGHT / 2.0; // meters

const HORIZONTAL_PIXELS_PER_METER: f32 = PITCH_DRAWING_WIDTH / PITCH_WIDTH; // pixels / meter
const VERTICAL_PIXELS_PER_METER: f32 = PITCH_DRAWING_HEIGHT / PITCH_HEIGHT; // pixels / meter
const _: () = assert!(HORIZONTAL_PIXELS_PER_METER == VERTICAL_PIXELS_PER_METER);
// pixels / meter (horizontal/vertical doesn't matter; assert gurantees both are same)
pub(crate) const PIXELS_PER_METER: f32 = HORIZONTAL_PIXELS_PER_METER;

pub(crate) const MARKING_COLOR: Color = WHITE;
pub(crate) const MARKING_THICKNESS: f32 = 0.12 * PIXELS_PER_METER; // pixels

pub(crate) const CENTER_CIRCLE_RADIUS: f32 = 9.15; // meters
pub(crate) const CENTER_SPOT_RADIUS: f32 = 0.15; // meters
