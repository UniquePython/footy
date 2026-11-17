use macroquad::camera::{Camera2D, set_camera, set_default_camera};
use macroquad::math::{Vec2, vec2};
use macroquad::window::{screen_height, screen_width};

pub(crate) struct Camera {
    center: Vec2, // meters, world space: the point shown at the middle of the window
    pixels_per_meter: f32,
}

impl Camera {
    pub(crate) fn new(center: Vec2, pixels_per_meter: f32) -> Self {
        Self {
            center,
            pixels_per_meter,
        }
    }

    // Draw calls after this are in world space (meters, +Y up)
    pub(crate) fn apply(&self) {
        set_camera(&Camera2D {
            target: self.center,
            // macroquad's zoom is NDC units per world meter, i.e. 2 / (visible size in meters).
            // zoom.y is negated because macroquad flips Y when drawing to the screen,
            // so the negation is what makes +Y point up.
            zoom: vec2(
                2.0 * self.pixels_per_meter / screen_width(),
                -2.0 * self.pixels_per_meter / screen_height(),
            ),
            ..Default::default()
        });
    }

    // Back to screen space (pixels, origin top-left), for HUD drawing
    pub(crate) fn reset() {
        set_default_camera();
    }
}
