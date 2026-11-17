use macroquad::color::RED;
use macroquad::shapes::draw_circle;

use crate::pitch::draw_pitch;

use crate::ball::Ball;
use crate::constants::{BALL_COLOR, BALL_RENDER_RADIUS, PLAYER_RENDER_RADIUS};
use crate::coords::world_to_screen;
use crate::player::Player;
use crate::stands::draw_stands;

pub(crate) fn draw(ball: &Ball, player: &Player) {
    draw_pitch();
    draw_stands();
    draw_ball(ball);
    draw_player(player);
}

fn draw_ball(ball: &Ball) {
    let screen = world_to_screen(ball.position);
    draw_circle(screen.x, screen.y, BALL_RENDER_RADIUS, BALL_COLOR);
}

fn draw_player(player: &Player) {
    let screen = world_to_screen(player.position);
    draw_circle(screen.x, screen.y, PLAYER_RENDER_RADIUS, RED);
}
