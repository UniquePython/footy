use macroquad::math::Vec2;

use crate::action::Action;
use crate::input::Bindings;

pub(crate) trait Controller {
    fn desired_direction(&self) -> Vec2;
}

pub(crate) struct HumanController {
    bindings: Bindings,
}

impl HumanController {
    pub(crate) fn new(bindings: Bindings) -> Self {
        Self { bindings }
    }
}

impl Controller for HumanController {
    fn desired_direction(&self) -> Vec2 {
        let mut direction = Vec2::ZERO;

        if self.bindings.is_active(Action::MoveUp) {
            direction.y += 1.0;
        }
        if self.bindings.is_active(Action::MoveDown) {
            direction.y -= 1.0;
        }
        if self.bindings.is_active(Action::MoveRight) {
            direction.x += 1.0;
        }
        if self.bindings.is_active(Action::MoveLeft) {
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
