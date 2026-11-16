use macroquad::input::{KeyCode, is_key_down};
use macroquad::math::Vec2;

pub(crate) trait Controller {
    fn desired_direction(&self) -> Vec2;
}

pub(crate) struct HumanController;

impl Controller for HumanController {
    fn desired_direction(&self) -> Vec2 {
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
}

pub(crate) struct AiController;

impl Controller for AiController {
    fn desired_direction(&self) -> Vec2 {
        Vec2::ZERO // placeholder
    }
}
