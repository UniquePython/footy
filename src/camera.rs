use macroquad::camera::{Camera2D, set_camera, set_default_camera};
use macroquad::math::{Vec2, vec2};
use macroquad::window::{screen_height, screen_width};

use crate::constants::CAMERA_VERTICAL_FOV;

pub(crate) struct Camera {
    center: Vec2,  // meters, world space: the point directly below the camera
    altitude: f32, // meters above the pitch
}

impl Camera {
    pub(crate) fn new(center: Vec2, altitude: f32) -> Self {
        Self { center, altitude }
    }

    // Height of world visible through the lens, in meters
    fn visible_height(&self) -> f32 {
        2.0 * self.altitude * (CAMERA_VERTICAL_FOV.to_radians() / 2.0).tan()
    }

    // Draw calls after this are in world space (meters, +Y up)
    pub(crate) fn apply(&self) {
        // The vertical FOV is fixed, so visible height depends only on altitude:
        // a wider window shows more to the sides, not a different zoom.
        let visible_height = self.visible_height();
        let aspect = screen_width() / screen_height();

        set_camera(&Camera2D {
            target: self.center,
            // macroquad's zoom is NDC units per world meter, i.e. 2 / (visible size in meters).
            // zoom.y is negated because macroquad flips Y when drawing to the screen,
            // so the negation is what makes +Y point up.
            zoom: vec2(2.0 / (visible_height * aspect), -2.0 / visible_height),
            ..Default::default()
        });
    }

    // Back to screen space (pixels, origin top-left), for HUD drawing
    pub(crate) fn reset() {
        set_default_camera();
    }
}
