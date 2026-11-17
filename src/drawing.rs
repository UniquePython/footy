use macroquad::color::RED;
use macroquad::shapes::draw_circle;

use crate::ball::Ball;
use crate::constants::BALL_COLOR;
use crate::pitch::draw_pitch;
use crate::player::Player;
use crate::stands::draw_stands;

pub(crate) fn draw(ball: &Ball, player: &Player) {
    draw_pitch();
    draw_stands();
    draw_ball(ball);
    draw_player(player);
}

fn draw_ball(ball: &Ball) {
    draw_circle(ball.position.x, ball.position.y, ball.radius, BALL_COLOR);
}

fn draw_player(player: &Player) {
    draw_circle(player.position.x, player.position.y, player.radius, RED);
}
