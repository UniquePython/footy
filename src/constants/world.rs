use super::pitch::{HALF_PITCH_HEIGHT, HALF_PITCH_WIDTH};
use super::stands::{STAND_DEPTH, STAND_GAP};

pub(crate) const END_MARGIN: f32 = 5.75; // meters, space behind each goal line inside the world

// Extent of the whole world (pitch + end margins + stands), centered on the origin
pub(crate) const WORLD_HALF_WIDTH: f32 = HALF_PITCH_WIDTH + END_MARGIN; // meters
pub(crate) const WORLD_HALF_HEIGHT: f32 = HALF_PITCH_HEIGHT + STAND_GAP + STAND_DEPTH; // meters
