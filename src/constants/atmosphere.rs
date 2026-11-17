use super::physics::GRAVITY;

// International Standard Atmosphere, valid in the troposphere (< 11 km)
pub(crate) const DEFAULT_SEA_LEVEL_TEMPERATURE: f32 = 288.15; // kelvin (15 C)
pub(crate) const DEFAULT_SEA_LEVEL_PRESSURE: f32 = 101_325.0; // pascals
pub(crate) const TEMPERATURE_LAPSE_RATE: f32 = 0.0065; // K per meter of altitude
pub(crate) const AIR_GAS_CONSTANT: f32 = 287.053; // J / (kg * K), specific gas constant of dry air
pub(crate) const BAROMETRIC_EXPONENT: f32 = GRAVITY / (AIR_GAS_CONSTANT * TEMPERATURE_LAPSE_RATE); // dimensionless

// Sutherland's law for dynamic viscosity of air
pub(crate) const SUTHERLAND_REFERENCE_VISCOSITY: f32 = 1.716e-5; // Pa * s, at the reference temperature
pub(crate) const SUTHERLAND_REFERENCE_TEMPERATURE: f32 = 273.15; // kelvin
pub(crate) const SUTHERLAND_CONSTANT: f32 = 110.4; // kelvin
