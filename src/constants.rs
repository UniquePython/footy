use macroquad::color::{Color, WHITE};
use macroquad::prelude::color_u8;

pub(crate) const PADDING_SIDELINE: f32 = 136.5; // pixels
pub(crate) const PADDING_GOAL_END: f32 = 60.0; // pixels
pub(crate) const PADDING_ALONG_X_AXIS: f32 = 2.0 * PADDING_GOAL_END; // pixels
pub(crate) const PADDING_ALONG_Y_AXIS: f32 = 2.0 * PADDING_SIDELINE; // pixels

pub(crate) const PITCH_WIDTH: f32 = 105.0; // meters
pub(crate) const PITCH_HEIGHT: f32 = 70.0; // meters

pub(crate) const HALF_PITCH_WIDTH: f32 = PITCH_WIDTH / 2.0; // meters
pub(crate) const HALF_PITCH_HEIGHT: f32 = PITCH_HEIGHT / 2.0; // meters

pub(crate) const PIXELS_PER_METER: f32 = 10.5;

pub const WINDOW_WIDTH: f32 = (PITCH_WIDTH * PIXELS_PER_METER) + PADDING_ALONG_X_AXIS; // pixels
pub const WINDOW_HEIGHT: f32 = (PITCH_HEIGHT * PIXELS_PER_METER) + PADDING_ALONG_Y_AXIS; // pixels

pub(crate) const PITCH_DRAWING_WIDTH: f32 = WINDOW_WIDTH - PADDING_ALONG_X_AXIS; // pixels
pub(crate) const PITCH_DRAWING_HEIGHT: f32 = WINDOW_HEIGHT - PADDING_ALONG_Y_AXIS; // pixels

const CHECK_HORIZONTAL_PIXELS_PER_METER: f32 = PITCH_DRAWING_WIDTH / PITCH_WIDTH;
const CHECK_VERTICAL_PIXELS_PER_METER: f32 = PITCH_DRAWING_HEIGHT / PITCH_HEIGHT;
const _: () = assert!(CHECK_HORIZONTAL_PIXELS_PER_METER == PIXELS_PER_METER);
const _: () = assert!(CHECK_VERTICAL_PIXELS_PER_METER == PIXELS_PER_METER);

pub(crate) const WINDOW_CENTER_X: f32 = WINDOW_WIDTH / 2.0; // pixels
pub(crate) const WINDOW_CENTER_Y: f32 = WINDOW_HEIGHT / 2.0; // pixels

pub(crate) const WINDOW_HALF_WIDTH_METERS: f32 = WINDOW_CENTER_X / PIXELS_PER_METER; // meters
pub(crate) const WINDOW_HALF_HEIGHT_METERS: f32 = WINDOW_CENTER_Y / PIXELS_PER_METER; // meters

pub(crate) const STAND_GAP: f32 = 5.0; // meters
pub(crate) const STAND_DEPTH: f32 = (PADDING_SIDELINE / PIXELS_PER_METER) - STAND_GAP; // meters

pub(crate) const SEAT_PITCH: f32 = 0.4; // meters, center-to-center spacing
pub(crate) const SEAT_SIZE: f32 = 0.3; // meters, drawn size of each seat
pub(crate) const STAND_BG_COLOR: Color = color_u8!(75, 60, 60, 255);
pub(crate) const STAND_ROW_COUNT: usize = 4;
pub(crate) const ROW_DEPTH: f32 = STAND_DEPTH / STAND_ROW_COUNT as f32;
pub(crate) const SEATS_PER_ROW_DEPTH: usize = (ROW_DEPTH / SEAT_PITCH).floor() as usize;
pub(crate) const SEAT_COLUMNS: usize =
    ((2.0 * WINDOW_HALF_WIDTH_METERS) / SEAT_PITCH).floor() as usize;
pub(crate) const STAND_ROW_COLOR: &[Color; STAND_ROW_COUNT] = &[
    color_u8!(225, 15, 15, 255),
    color_u8!(195, 30, 30, 255),
    color_u8!(165, 45, 45, 255),
    color_u8!(135, 60, 60, 255),
];

pub(crate) const COLLISION_BOUNDARY_WIDTH: f32 = WINDOW_HALF_WIDTH_METERS; // meters
pub(crate) const COLLISION_BOUNDARY_HEIGHT: f32 = HALF_PITCH_HEIGHT + STAND_GAP; // meters

pub(crate) const MARKING_COLOR: Color = WHITE;
pub(crate) const MARKING_THICKNESS: f32 = 0.12 * PIXELS_PER_METER; // pixels

pub(crate) const PENALTY_AREA_HEIGHT: f32 = 40.32;
pub(crate) const PENALTY_AREA_WIDTH: f32 = 16.5;
pub(crate) const GOAL_AREA_HEIGHT: f32 = 18.32;
pub(crate) const GOAL_AREA_WIDTH: f32 = 5.5;
pub(crate) const PENALTY_SPOT_DISTANCE: f32 = 11.0;

pub(crate) const CENTER_CIRCLE_RADIUS: f32 = 9.15; // meters
pub(crate) const SPOT_RADIUS: f32 = 0.15; // meters

pub(crate) const CORNER_ARC_RADIUS: f32 = 1.0; // meters
pub(crate) const PENALTY_ARC_RADIUS: f32 = CENTER_CIRCLE_RADIUS; // meters
pub(crate) const PENALTY_ARC_HALF_ANGLE: f32 = 53.0518; // degrees, derived from box geometry
pub(crate) const PENALTY_ARC_SWEEP: f32 = 2.0 * PENALTY_ARC_HALF_ANGLE; // degrees

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

pub(crate) const PLAYER_WALL_RESTITUTION: f32 = 0.3;
pub(crate) const BALL_WALL_RESTITUTION: f32 = 0.85; // FIFA rigid-surface bounce spec (0.82–0.88 range)
pub(crate) const PLAYER_BALL_RESTITUTION: f32 = 0.75; // approximates passive ball-to-body impact research
