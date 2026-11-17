use super::ball::{BALL_PHYSICS_RADIUS, BALL_RENDER_RADIUS};

pub(crate) const PLAYER_PHYSICS_RADIUS: f32 = 0.3; // meters

const PLAYER_TO_BALL_PHYSICS_RADIUS_RATIO: f32 = PLAYER_PHYSICS_RADIUS / BALL_PHYSICS_RADIUS;

pub(crate) const PLAYER_RENDER_RADIUS: f32 =
    BALL_RENDER_RADIUS * PLAYER_TO_BALL_PHYSICS_RADIUS_RATIO; // pixels

pub(crate) const MIN_SPEED: f32 = 6.0; // m/s
pub(crate) const MAX_SPEED: f32 = 9.5; // m/s

pub(crate) const MIN_ACCELERATION: f32 = 2.5; // m/s^2
pub(crate) const MAX_ACCELERATION: f32 = 6.0; // m/s^2
