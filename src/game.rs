use macroquad::math::Vec2;
use macroquad::time::get_frame_time;
use macroquad::window::{clear_background, next_frame};

use crate::attributes::Attributes;
use crate::ball::Ball;
use crate::constants::BG_COLOR;
use crate::controller::HumanController;
use crate::drawing::draw;
use crate::physics_simulator::PhysicsSimulator;
use crate::player::Player;

pub async fn run() {
    let ball = Ball::new(Vec2::ZERO);
    let player = Player::new(
        Vec2::ONE,
        Attributes {
            pace: 80,
            acceleration: 80,
            mass: 75.0,
        },
        Box::new(HumanController),
    ); // stub data

    let mut simulation = PhysicsSimulator::new(ball, player);

    loop {
        let dt = get_frame_time();

        simulation.step(dt);

        clear_background(BG_COLOR);
        draw(simulation.ball(), simulation.player());

        next_frame().await;
    }
}
