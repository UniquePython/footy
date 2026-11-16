use macroquad::input::{KeyCode, is_key_down};
use macroquad::math::Vec2;
use macroquad::time::get_frame_time;
use macroquad::window::{clear_background, next_frame};

use crate::attributes::Attributes;
use crate::ball::Ball;
use crate::constants::BG_COLOR;
use crate::drawing::draw;
use crate::player::Player;

// Temporary
fn read_movement_input() -> Vec2 {
    let mut direction = Vec2::ZERO;

    if is_key_down(KeyCode::Up) {
        direction.y += 1.0;
    }
    if is_key_down(KeyCode::Down) {
        direction.y -= 1.0;
    }
    if is_key_down(KeyCode::Right) {
        direction.x += 1.0;
    }
    if is_key_down(KeyCode::Left) {
        direction.x -= 1.0;
    }

    direction.normalize_or_zero()
}

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
    ); // stub data

    loop {
        let dt = get_frame_time();

        ball.update(dt);
        player.update(read_movement_input(), dt);

        clear_background(BG_COLOR);
        draw(&ball, &player);

        next_frame().await;
    }
}
