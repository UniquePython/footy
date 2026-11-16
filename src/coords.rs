use macroquad::math::Vec2;

use crate::constants::{
    HORIZONTAL_PIXELS_PER_METER, VERTICAL_PIXELS_PER_METER, WINDOW_CENTER_X, WINDOW_CENTER_Y,
};

pub(crate) fn world_to_screen(pos: Vec2) -> Vec2 {
    let mut result = pos;

    // scale to pixels
    result.x *= HORIZONTAL_PIXELS_PER_METER;
    result.y *= VERTICAL_PIXELS_PER_METER;

    // flip Y (world has +Y upwards, but in screen +Y is downwards)
    result.y *= -1.0;

    // shift by screen space center
    result.x += WINDOW_CENTER_X;
    result.y += WINDOW_CENTER_Y;

    result
}
