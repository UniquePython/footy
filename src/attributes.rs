use crate::constants::{MAX_ACCELERATION, MAX_SPEED, MIN_ACCELERATION, MIN_SPEED};

#[derive(Debug, Clone, Copy)]
pub(crate) struct Attributes {
    pub(crate) pace: u8,         // 0-99
    pub(crate) acceleration: u8, // 0-99
    pub(crate) mass: f32,        // kg
}

impl Attributes {
    const MAX: f32 = 99.0;

    fn normalize(&self, value: u8) -> f32 {
        value as f32 / Self::MAX
    }
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

impl Attributes {
    pub(crate) fn pace_value(&self) -> f32 {
        lerp(MIN_SPEED, MAX_SPEED, self.normalize(self.pace))
    }

    pub(crate) fn acceleration_value(&self) -> f32 {
        lerp(
            MIN_ACCELERATION,
            MAX_ACCELERATION,
            self.normalize(self.acceleration),
        )
    }
}
