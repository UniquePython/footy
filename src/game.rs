use macroquad::math::Vec2;
use macroquad::time::get_frame_time;
use macroquad::window::{clear_background, next_frame};

use crate::attributes::Attributes;
use crate::ball::Ball;
use crate::constants::BG_COLOR;
use crate::controller::HumanController;
use crate::drawing::draw;
use crate::player::Player;

pub async fn run() {
    // let mut ball = Ball {
    //     velocity: vec2(5.0, 0.0),
    //     ..Ball::new(Vec2::ZERO)
    // }; // For visualizing rolling

    let mut ball = Ball::new(Vec2::ZERO);
    let mut player = Player::new(
        Vec2::ONE,
        Attributes {
            pace: 80,
            acceleration: 80,
        },
        Box::new(HumanController),
    ); // stub data

    loop {
        let dt = get_frame_time();

        ball.update(dt);
        player.update(dt);

        clear_background(BG_COLOR);
        draw(&ball, &player);

        next_frame().await;
    }
}
