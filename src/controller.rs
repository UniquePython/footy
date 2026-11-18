use macroquad::math::Vec2;

use crate::action::Action;
use crate::facing_turn::FacingTurn;
use crate::input::Bindings;

pub(crate) trait Controller {
    fn desired_direction(&self) -> Vec2;
    fn facing_turn(&self) -> FacingTurn;
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

    fn facing_turn(&self) -> FacingTurn {
        let left = self.bindings.is_active(Action::FaceLeft);
        let right = self.bindings.is_active(Action::FaceRight);

        // Both held cancels out, same as opposing movement keys would
        if left == right {
            FacingTurn::None
        } else if left {
            FacingTurn::Left
        } else {
            FacingTurn::Right
        }
    }
}

pub(crate) struct AiController;

impl Controller for AiController {
    fn desired_direction(&self) -> Vec2 {
        Vec2::ZERO // placeholder
    }

    fn facing_turn(&self) -> FacingTurn {
        FacingTurn::None // placeholder
    }
}
