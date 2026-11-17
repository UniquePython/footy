use macroquad::math::Vec2;
use macroquad::time::get_frame_time;
use macroquad::window::{clear_background, next_frame};

use crate::attributes::Attributes;
use crate::ball::Ball;
use crate::camera::Camera;
use crate::constants::{BG_COLOR, PIXELS_PER_METER};
use crate::controller::HumanController;
use crate::drawing::draw;
use crate::environment::Environment;
use crate::input::Bindings;
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
        Box::new(HumanController::new(Bindings::standard())),
    ); // stub data

    let environment = Environment::standard();
    let mut simulation = PhysicsSimulator::new(ball, player, environment);

    let camera = Camera::new(Vec2::ZERO, PIXELS_PER_METER);

    loop {
        let dt = get_frame_time();

        simulation.step(dt);

        clear_background(BG_COLOR);

        camera.apply();
        draw(simulation.ball(), simulation.player());
        Camera::reset();

        next_frame().await;
    }
}
