use macroquad::color::Color;
use macroquad::shapes::draw_rectangle;

use crate::constants::{
    HALF_PITCH_HEIGHT, ROW_DEPTH, SEAT_COLUMNS, SEAT_PITCH, SEAT_SIZE, SEATS_PER_ROW_DEPTH,
    STAND_BG_COLOR, STAND_DEPTH, STAND_GAP, STAND_ROW_COLOR, WORLD_HALF_WIDTH,
};

pub(crate) fn draw_stands() {
    draw_stand_side(1.0);
    draw_stand_side(-1.0);
}

// Draws one stand (top when `sign` is 1.0, bottom when `sign` is -1.0).
fn draw_stand_side(sign: f32) {
    draw_stand_background(sign);
    draw_stand_seats(sign);
}

fn draw_stand_background(sign: f32) {
    let near_edge = HALF_PITCH_HEIGHT + STAND_GAP;
    let far_edge = near_edge + STAND_DEPTH;

    let y_bottom = (sign * near_edge).min(sign * far_edge);

    draw_rectangle(
        -WORLD_HALF_WIDTH,
        y_bottom,
        2.0 * WORLD_HALF_WIDTH,
        STAND_DEPTH,
        STAND_BG_COLOR,
    );
}

fn draw_stand_seats(sign: f32) {
    for (row_index, color) in STAND_ROW_COLOR.iter().enumerate() {
        let row_near = HALF_PITCH_HEIGHT + STAND_GAP + (row_index as f32) * ROW_DEPTH;

        for seat_row in 0..SEATS_PER_ROW_DEPTH {
            // Center of this seat-row's y-position within the band, in meters.
            let y_center = row_near + (seat_row as f32 + 0.5) * SEAT_PITCH;

            for col in 0..SEAT_COLUMNS {
                let x_center = -WORLD_HALF_WIDTH + (col as f32 + 0.5) * SEAT_PITCH;
                draw_seat(x_center, y_center, sign, *color);
            }
        }
    }
}

fn draw_seat(x_center: f32, y_center: f32, sign: f32, color: Color) {
    let y = sign * y_center;

    draw_rectangle(
        x_center - SEAT_SIZE / 2.0,
        y - SEAT_SIZE / 2.0,
        SEAT_SIZE,
        SEAT_SIZE,
        color,
    );
}
