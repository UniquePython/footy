use macroquad::color::Color;
use macroquad::math::vec2;
use macroquad::shapes::draw_rectangle;

use crate::constants::{
    HALF_PITCH_HEIGHT, PIXELS_PER_METER, ROW_DEPTH, SEAT_COLUMNS, SEAT_PITCH, SEAT_SIZE,
    SEATS_PER_ROW_DEPTH, STAND_BG_COLOR, STAND_DEPTH, STAND_GAP, STAND_ROW_COLOR,
    WINDOW_HALF_WIDTH_METERS,
};
use crate::coords::world_to_screen;

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

    let y_near = sign * near_edge;
    let y_far = sign * far_edge;

    let top_left_world = vec2(-WINDOW_HALF_WIDTH_METERS, y_near.max(y_far));
    let screen = world_to_screen(top_left_world);

    let width_px = 2.0 * WINDOW_HALF_WIDTH_METERS * PIXELS_PER_METER;
    let height_px = STAND_DEPTH * PIXELS_PER_METER;

    draw_rectangle(screen.x, screen.y, width_px, height_px, STAND_BG_COLOR);
}

fn draw_stand_seats(sign: f32) {
    for (row_index, color) in STAND_ROW_COLOR.iter().enumerate() {
        let row_near = HALF_PITCH_HEIGHT + STAND_GAP + (row_index as f32) * ROW_DEPTH;

        for seat_row in 0..SEATS_PER_ROW_DEPTH {
            // Center of this seat-row's y-position within the band, in meters.
            let y_center = row_near + (seat_row as f32 + 0.5) * SEAT_PITCH;

            for col in 0..SEAT_COLUMNS {
                let x_center = -WINDOW_HALF_WIDTH_METERS + (col as f32 + 0.5) * SEAT_PITCH;
                draw_seat(x_center, y_center, sign, *color);
            }
        }
    }
}

fn draw_seat(x_center: f32, y_center: f32, sign: f32, color: Color) {
    let y = sign * y_center;

    let top_left_world = vec2(x_center - SEAT_SIZE / 2.0, y + SEAT_SIZE / 2.0);
    let screen = world_to_screen(top_left_world);

    let size_px = SEAT_SIZE * PIXELS_PER_METER;

    draw_rectangle(screen.x, screen.y, size_px, size_px, color);
}
