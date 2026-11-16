use macroquad::math::Vec2;
use macroquad::time::get_frame_time;
use macroquad::window::{clear_background, next_frame};

use crate::ball::Ball;
use crate::constants::BG_COLOR;
use crate::drawing::draw;

pub async fn run() {
    // let mut ball = Ball {
    //     velocity: vec2(5.0, 0.0),
    //     ..Ball::new(Vec2::ZERO)
    // }; // For visualizing rolling

    let mut ball = Ball::new(Vec2::ZERO);

    loop {
        let dt = get_frame_time();

        ball.update(dt);

        clear_background(BG_COLOR);
        draw(&ball);

        next_frame().await;
    }
}
