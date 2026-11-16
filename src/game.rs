use macroquad::color::BLACK;
use macroquad::math::Vec2;
use macroquad::window::{clear_background, next_frame};

use crate::ball::Ball;
use crate::drawing::draw;

pub async fn run() {
    let ball = Ball::new(Vec2::ZERO);
    loop {
        clear_background(BLACK);
        draw(&ball);
        next_frame().await;
    }
}
