pub(crate) const PLAYER_PHYSICS_RADIUS: f32 = 0.3; // meters

pub(crate) const MIN_SPEED: f32 = 6.0; // m/s
pub(crate) const MAX_SPEED: f32 = 9.5; // m/s

pub(crate) const MIN_ACCELERATION: f32 = 2.5; // m/s^2
pub(crate) const MAX_ACCELERATION: f32 = 6.0; // m/s^2

// Facing is an angular offset from the player's movement direction, steered by
// holding left/right. Turning in is slower and deliberate; returning to center
// on release is faster, so aim resets decisively.
pub(crate) const FACING_TURN_RATE: f32 = 3.0; // rad/s, while turning left/right
pub(crate) const FACING_RETURN_RATE: f32 = 2.0 * FACING_TURN_RATE; // rad/s, while returning to center
pub(crate) const MAX_FACING_OFFSET: f32 = std::f32::consts::FRAC_PI_2; // radians, ±90°

pub(crate) const FACING_INDICATOR_LENGTH: f32 = 0.5; // meters, tip distance from player center
pub(crate) const FACING_INDICATOR_WIDTH: f32 = 0.25; // meters, base width
pub(crate) const FACING_INDICATOR_COLOR: macroquad::color::Color = macroquad::color::YELLOW;
