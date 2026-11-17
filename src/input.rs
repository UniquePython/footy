use std::collections::HashMap;

use macroquad::input::{KeyCode, is_key_down};

use crate::action::Action;

pub(crate) struct Bindings {
    keys: HashMap<Action, Vec<KeyCode>>,
}

impl Bindings {
    // WASD and arrow keys both work
    pub(crate) fn standard() -> Self {
        let keys = HashMap::from([
            (Action::MoveUp, vec![KeyCode::W, KeyCode::Up]),
            (Action::MoveDown, vec![KeyCode::S, KeyCode::Down]),
            (Action::MoveLeft, vec![KeyCode::A, KeyCode::Left]),
            (Action::MoveRight, vec![KeyCode::D, KeyCode::Right]),
        ]);

        Self { keys }
    }

    // True if any key bound to this action is currently held
    pub(crate) fn is_active(&self, action: Action) -> bool {
        self.keys
            .get(&action)
            .is_some_and(|keys| keys.iter().any(|&key| is_key_down(key)))
    }
}
