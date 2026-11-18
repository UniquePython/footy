use macroquad::color::RED;
use macroquad::math::Vec2;
use macroquad::shapes::{draw_circle, draw_triangle};

use crate::ball::Ball;
use crate::constants::{
    BALL_COLOR, FACING_INDICATOR_COLOR, FACING_INDICATOR_LENGTH, FACING_INDICATOR_WIDTH,
};
use crate::pitch::draw_pitch;
use crate::player::Player;
use crate::stands::draw_stands;

pub(crate) fn draw(ball: &Ball, player: &Player) {
    draw_pitch();
    draw_stands();
    draw_ball(ball);
    draw_player(player);
    draw_facing_indicator(player);
}

fn draw_ball(ball: &Ball) {
    draw_circle(ball.position.x, ball.position.y, ball.radius, BALL_COLOR);
}

fn draw_player(player: &Player) {
    draw_circle(player.position.x, player.position.y, player.radius, RED);
}

fn draw_facing_indicator(player: &Player) {
    let facing = player.facing();
    let perpendicular = Vec2::new(-facing.y, facing.x); // 90° rotation of facing

    let tip = player.position + facing * FACING_INDICATOR_LENGTH;
    let base_half = perpendicular * (FACING_INDICATOR_WIDTH / 2.0);
    let base_left = player.position + base_half;
    let base_right = player.position - base_half;

    draw_triangle(tip, base_left, base_right, FACING_INDICATOR_COLOR);
}
