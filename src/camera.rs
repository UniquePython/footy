use macroquad::camera::{Camera2D, set_camera, set_default_camera};
use macroquad::math::{Vec2, vec2};
use macroquad::window::{screen_height, screen_width};

use crate::constants::{
    CAMERA_ALTITUDE_SPRING_DAMPING, CAMERA_ALTITUDE_SPRING_FREQUENCY, CAMERA_FRAMING_MARGIN,
    CAMERA_LOOKAHEAD_TIME, CAMERA_MAX_ALTITUDE, CAMERA_MAX_LOOKAHEAD, CAMERA_MAX_UPDATE_DT,
    CAMERA_MIN_ALTITUDE, CAMERA_PRIMARY_SAFETY_MARGIN, CAMERA_SPRING_DAMPING,
    CAMERA_SPRING_FREQUENCY, CAMERA_VERTICAL_FOV, WORLD_HALF_HEIGHT, WORLD_HALF_WIDTH,
};

// Something the camera tries to keep in view
#[derive(Clone, Copy)]
pub(crate) struct Subject {
    pub(crate) position: Vec2, // meters
    pub(crate) velocity: Vec2, // m/s
}

impl Subject {
    // Where the subject will be shortly, used to frame what is about to happen
    fn lead_point(&self) -> Vec2 {
        self.position
            + (self.velocity * CAMERA_LOOKAHEAD_TIME).clamp_length_max(CAMERA_MAX_LOOKAHEAD)
    }
}

pub(crate) struct Camera {
    center: Vec2,           // meters, world space: the point directly below the camera
    velocity: Vec2,         // m/s
    altitude: f32,          // meters above the pitch
    altitude_velocity: f32, // m/s
}

// Half the height of world visible per meter of altitude. The vertical FOV is fixed,
// so a wider window shows more to the sides, not a different zoom.
fn lens_slope() -> f32 {
    (CAMERA_VERTICAL_FOV.to_radians() / 2.0).tan()
}

fn aspect_ratio() -> f32 {
    screen_width() / screen_height()
}

// Half the world size visible through the lens at a given altitude, in meters
fn half_extents_at(altitude: f32) -> Vec2 {
    let half_height = altitude * lens_slope();
    vec2(half_height * aspect_ratio(), half_height)
}

impl Camera {
    pub(crate) fn new(center: Vec2, altitude: f32) -> Self {
        let mut camera = Self {
            center,
            velocity: Vec2::ZERO,
            altitude,
            altitude_velocity: 0.0,
        };

        // Start inside the allowed region instead of snapping there on the first frame
        camera.constrain_to_world();
        camera
    }

    // Frames both subjects when they fit, and the primary one always
    pub(crate) fn update(&mut self, dt: f32, primary: Subject, secondary: Subject) {
        let dt = dt.min(CAMERA_MAX_UPDATE_DT);
        let usable = 1.0 - CAMERA_FRAMING_MARGIN;

        // Bounding box of everything we want in view: both subjects and where they are heading
        let points = [
            primary.position,
            primary.lead_point(),
            secondary.position,
            secondary.lead_point(),
        ];
        let min = points.iter().copied().reduce(Vec2::min).unwrap();
        let max = points.iter().copied().reduce(Vec2::max).unwrap();
        let box_center = (min + max) / 2.0;
        let box_half = (max - min) / 2.0;

        // Altitude at which that box fits inside the usable part of the view
        let slope = lens_slope();
        let needed_altitude =
            (box_half.y / (usable * slope)).max(box_half.x / (usable * slope * aspect_ratio()));
        let target_altitude = needed_altitude.clamp(CAMERA_MIN_ALTITUDE, CAMERA_MAX_ALTITUDE);

        let omega = CAMERA_ALTITUDE_SPRING_FREQUENCY;
        let altitude_acceleration = omega * omega * (target_altitude - self.altitude)
            - 2.0 * CAMERA_ALTITUDE_SPRING_DAMPING * omega * self.altitude_velocity;
        self.altitude_velocity += altitude_acceleration * dt;
        self.altitude = (self.altitude + self.altitude_velocity * dt)
            .clamp(CAMERA_MIN_ALTITUDE, CAMERA_MAX_ALTITUDE);

        // Aim at the middle of the box, but never so far from the primary subject
        // that it would leave the usable part of the current view
        let inner_half = half_extents_at(self.altitude) * usable;
        let target = box_center.clamp(primary.position - inner_half, primary.position + inner_half);

        let omega = CAMERA_SPRING_FREQUENCY;
        let acceleration = omega * omega * (target - self.center)
            - 2.0 * CAMERA_SPRING_DAMPING * omega * self.velocity;
        self.velocity += acceleration * dt;
        self.center += self.velocity * dt;

        // Hard guarantee: the primary subject never leaves the view, however hard it is hit
        let safe_half = half_extents_at(self.altitude) * (1.0 - CAMERA_PRIMARY_SAFETY_MARGIN);
        self.constrain(primary.position - safe_half, primary.position + safe_half);

        self.constrain_to_world();
    }

    // Clamps the center, and stops pushing into a limit so velocity doesn't build up against it
    fn constrain(&mut self, min: Vec2, max: Vec2) {
        let clamped = self.center.clamp(min, max);

        if clamped.x != self.center.x {
            self.velocity.x = 0.0;
        }
        if clamped.y != self.center.y {
            self.velocity.y = 0.0;
        }

        self.center = clamped;
    }

    // Keeps the view inside the world. On an axis where the view is larger than
    // the world, the camera stays centered.
    fn constrain_to_world(&mut self) {
        let limit = (vec2(WORLD_HALF_WIDTH, WORLD_HALF_HEIGHT) - half_extents_at(self.altitude))
            .max(Vec2::ZERO);

        self.constrain(-limit, limit);
    }

    // Draw calls after this are in world space (meters, +Y up)
    pub(crate) fn apply(&self) {
        let half = half_extents_at(self.altitude);

        set_camera(&Camera2D {
            target: self.center,
            // macroquad's zoom is NDC units per world meter, i.e. 1 / (visible half-size in meters).
            // zoom.y is negated because macroquad flips Y when drawing to the screen,
            // so the negation is what makes +Y point up.
            zoom: vec2(1.0 / half.x, -1.0 / half.y),
            ..Default::default()
        });
    }

    // Back to screen space (pixels, origin top-left), for HUD drawing
    pub(crate) fn reset() {
        set_default_camera();
    }
}
