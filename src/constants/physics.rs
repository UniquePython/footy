use super::pitch::HALF_PITCH_HEIGHT;
use super::stands::STAND_GAP;
use super::world::WORLD_HALF_WIDTH;

pub(crate) const GRAVITY: f32 = 9.81; // m/s^2

pub(crate) const PLAYER_WALL_RESTITUTION: f32 = 0.3;
pub(crate) const BALL_WALL_RESTITUTION: f32 = 0.85; // FIFA rigid-surface bounce spec (0.82–0.88 range)
pub(crate) const PLAYER_BALL_RESTITUTION: f32 = 0.75; // approximates passive ball-to-body impact research

pub(crate) const COLLISION_BOUNDARY_WIDTH: f32 = WORLD_HALF_WIDTH; // meters
pub(crate) const COLLISION_BOUNDARY_HEIGHT: f32 = HALF_PITCH_HEIGHT + STAND_GAP; // meters
