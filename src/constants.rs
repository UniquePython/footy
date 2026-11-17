use macroquad::color::{Color, WHITE};
use macroquad::prelude::color_u8;

pub const WINDOW_WIDTH: f32 = 1380.0; // pixels
pub const WINDOW_HEIGHT: f32 = 960.0; // pixels

pub(crate) const PADDING_EACH_SIDE: f32 = 60.0; // pixels
pub(crate) const PADDING_ALONG_AXIS: f32 = 2.0 * PADDING_EACH_SIDE; // pixels

pub(crate) const PITCH_DRAWING_WIDTH: f32 = WINDOW_WIDTH - PADDING_ALONG_AXIS; // pixels
pub(crate) const PITCH_DRAWING_HEIGHT: f32 = WINDOW_HEIGHT - PADDING_ALONG_AXIS; // pixels

pub(crate) const WINDOW_CENTER_X: f32 = WINDOW_WIDTH / 2.0; // pixels
pub(crate) const WINDOW_CENTER_Y: f32 = WINDOW_HEIGHT / 2.0; // pixels

pub(crate) const PITCH_WIDTH: f32 = 105.0; // meters
pub(crate) const PITCH_HEIGHT: f32 = 70.0; // meters

pub(crate) const HALF_PITCH_WIDTH: f32 = PITCH_WIDTH / 2.0; // meters
pub(crate) const HALF_PITCH_HEIGHT: f32 = PITCH_HEIGHT / 2.0; // meters

const HORIZONTAL_PIXELS_PER_METER: f32 = PITCH_DRAWING_WIDTH / PITCH_WIDTH; // pixels / meter
const VERTICAL_PIXELS_PER_METER: f32 = PITCH_DRAWING_HEIGHT / PITCH_HEIGHT; // pixels / meter
const _: () = assert!(HORIZONTAL_PIXELS_PER_METER == VERTICAL_PIXELS_PER_METER);
// pixels / meter (horizontal/vertical doesn't matter; assert gurantees both are same)
pub(crate) const PIXELS_PER_METER: f32 = HORIZONTAL_PIXELS_PER_METER;

pub(crate) const MARKING_COLOR: Color = WHITE;
pub(crate) const MARKING_THICKNESS: f32 = 0.12 * PIXELS_PER_METER; // pixels

pub(crate) const CENTER_CIRCLE_RADIUS: f32 = 9.15; // meters
pub(crate) const CENTER_SPOT_RADIUS: f32 = 0.15; // meters

pub(crate) const BALL_PHYSICS_RADIUS: f32 = 0.11; // meters
pub(crate) const BALL_RENDER_RADIUS: f32 = 3.5; // pixels
pub(crate) const BALL_MASS: f32 = 0.43; // kg

pub(crate) const BG_COLOR: Color = color_u8!(96, 145, 22, 255);
pub(crate) const PITCH_COLOR: Color = color_u8!(126, 175, 52, 255);
pub(crate) const BALL_COLOR: Color = color_u8!(240, 255, 0, 255);

pub(crate) const BALL_FRICTION_COEFFICIENT: f32 = 0.035;
pub(crate) const GRAVITY: f32 = 9.81; // m/s^2
pub(crate) const BALL_ROLLING_DECELERATION: f32 = BALL_FRICTION_COEFFICIENT * GRAVITY; // m/s^2

pub(crate) const MIN_SPEED: f32 = 6.0; // m/s
pub(crate) const MAX_SPEED: f32 = 9.5; // m/s

pub(crate) const MIN_ACCELERATION: f32 = 2.5; // m/s^2
pub(crate) const MAX_ACCELERATION: f32 = 6.0; // m/s^2

pub(crate) const PLAYER_PHYSICS_RADIUS: f32 = 0.3; // meters

const PLAYER_TO_BALL_PHYSICS_RADIUS_RATIO: f32 = PLAYER_PHYSICS_RADIUS / BALL_PHYSICS_RADIUS;

pub(crate) const PLAYER_RENDER_RADIUS: f32 =
    BALL_RENDER_RADIUS * PLAYER_TO_BALL_PHYSICS_RADIUS_RATIO; // pixels
