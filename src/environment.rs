use macroquad::math::Vec2;

use crate::constants::{
    AIR_GAS_CONSTANT, BAROMETRIC_EXPONENT, DEFAULT_SEA_LEVEL_PRESSURE,
    DEFAULT_SEA_LEVEL_TEMPERATURE, SUTHERLAND_CONSTANT, SUTHERLAND_REFERENCE_TEMPERATURE,
    SUTHERLAND_REFERENCE_VISCOSITY, TEMPERATURE_LAPSE_RATE,
};

#[derive(Debug, Clone, Copy)]
pub(crate) struct Environment {
    pub(crate) wind: Vec2,         // m/s, world space
    pub(crate) air_density: f32,   // kg / m^3
    pub(crate) air_viscosity: f32, // Pa * s
}

impl Environment {
    pub(crate) fn new(
        altitude: f32,              // meters above sea level
        sea_level_temperature: f32, // kelvin
        sea_level_pressure: f32,    // pascals
        wind: Vec2,
    ) -> Self {
        let temperature = sea_level_temperature - TEMPERATURE_LAPSE_RATE * altitude;

        let pressure =
            sea_level_pressure * (temperature / sea_level_temperature).powf(BAROMETRIC_EXPONENT);

        Self {
            wind,
            air_density: pressure / (AIR_GAS_CONSTANT * temperature),
            air_viscosity: sutherland_viscosity(temperature),
        }
    }

    // Sea level, 15 C, no wind
    pub(crate) fn standard() -> Self {
        Self::new(
            0.0,
            DEFAULT_SEA_LEVEL_TEMPERATURE,
            DEFAULT_SEA_LEVEL_PRESSURE,
            Vec2::ZERO,
        )
    }
}

fn sutherland_viscosity(temperature: f32) -> f32 {
    SUTHERLAND_REFERENCE_VISCOSITY
        * (temperature / SUTHERLAND_REFERENCE_TEMPERATURE).powf(1.5)
        * (SUTHERLAND_REFERENCE_TEMPERATURE + SUTHERLAND_CONSTANT)
        / (temperature + SUTHERLAND_CONSTANT)
}
