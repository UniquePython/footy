use crate::constants::{
    BMI_SPAN, HEIGHT_SPAN, K_HEIGHT_ACCEL, K_HEIGHT_SPEED, K_MASS_ACCEL, K_MASS_SPEED,
    MAX_ACCELERATION, MAX_SPEED, MIN_ACCELERATION, MIN_SPEED, REF_BMI, REF_HEIGHT,
};

#[derive(Debug, Clone, Copy)]
pub(crate) struct Attributes {
    pub(crate) pace: u8,         // 0-99
    pub(crate) acceleration: u8, // 0-99
    pub(crate) mass: f32,        // kg
    pub(crate) height: f32,      // meters
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
    fn bmi(&self) -> f32 {
        debug_assert!(self.height > 0.0, "height must be positive");
        self.mass / (self.height * self.height)
    }

    // -1.0 (short) .. 0.0 (reference) .. 1.0 (tall)
    fn height_t(&self) -> f32 {
        ((self.height - REF_HEIGHT) / HEIGHT_SPAN).clamp(-1.0, 1.0)
    }

    // 0.0 (at or below reference BMI) .. 1.0 (heavy for height). One-sided: no bonus for lean.
    fn mass_t(&self) -> f32 {
        ((self.bmi() - REF_BMI) / BMI_SPAN).clamp(0.0, 1.0)
    }

    fn speed_multiplier(&self) -> f32 {
        1.0 + K_HEIGHT_SPEED * self.height_t() - K_MASS_SPEED * self.mass_t()
    }

    fn acceleration_multiplier(&self) -> f32 {
        1.0 - K_HEIGHT_ACCEL * self.height_t() - K_MASS_ACCEL * self.mass_t()
    }

    pub(crate) fn pace_value(&self) -> f32 {
        let m = self.speed_multiplier();
        lerp(MIN_SPEED * m, MAX_SPEED * m, self.normalize(self.pace))
    }

    pub(crate) fn acceleration_value(&self) -> f32 {
        let m = self.acceleration_multiplier();
        lerp(
            MIN_ACCELERATION * m,
            MAX_ACCELERATION * m,
            self.normalize(self.acceleration),
        )
    }
}
