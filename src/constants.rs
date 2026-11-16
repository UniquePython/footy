pub(crate) const WINDOW_WIDTH: f32 = 1380.0; // pixels
pub(crate) const WINDOW_HEIGHT: f32 = 960.0; // pixels

pub(crate) const PADDING_EACH_SIDE: f32 = 60.0; // pixels
pub(crate) const PADDING_ALONG_AXIS: f32 = 2.0 * PADDING_EACH_SIDE; // pixels

pub(crate) const PITCH_DRAWING_WIDTH: f32 = WINDOW_WIDTH - PADDING_ALONG_AXIS; // pixels
pub(crate) const PITCH_DRAWING_HEIGHT: f32 = WINDOW_HEIGHT - PADDING_ALONG_AXIS; // pixels

pub(crate) const WINDOW_CENTER_X: f32 = WINDOW_WIDTH / 2.0; // pixels
pub(crate) const WINDOW_CENTER_Y: f32 = WINDOW_HEIGHT / 2.0; // pixels

pub(crate) const PITCH_WIDTH: f32 = 105.0; // meters
pub(crate) const PITCH_HEIGHT: f32 = 70.0; // meters

pub(crate) const HORIZONTAL_PIXELS_PER_METER: f32 = PITCH_DRAWING_WIDTH / PITCH_WIDTH; // pixels / meter
pub(crate) const VERTICAL_PIXELS_PER_METER: f32 = PITCH_DRAWING_HEIGHT / PITCH_HEIGHT; // pixels / meter
