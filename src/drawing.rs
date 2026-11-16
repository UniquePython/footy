use macroquad::shapes::draw_circle;

use crate::pitch::draw_pitch;

use crate::ball::Ball;
use crate::constants::{BALL_COLOR, BALL_RENDER_RADIUS};
use crate::coords::world_to_screen;

pub(crate) fn draw(ball: &Ball) {
    draw_pitch();
    draw_ball(ball);
}

fn draw_ball(ball: &Ball) {
    let screen = world_to_screen(ball.position);
    draw_circle(screen.x, screen.y, BALL_RENDER_RADIUS, BALL_COLOR);
}
