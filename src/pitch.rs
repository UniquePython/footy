use macroquad::color::Color;
use macroquad::math::vec2;
use macroquad::{
    color_u8,
    shapes::{draw_circle, draw_circle_lines, draw_line, draw_rectangle, draw_rectangle_lines},
};

use crate::constants::{
    CENTER_CIRCLE_RADIUS, CENTER_SPOT_RADIUS, HALF_PITCH_HEIGHT, HALF_PITCH_WIDTH, MARKING_COLOR,
    MARKING_THICKNESS, PITCH_HEIGHT, PITCH_WIDTH, PIXELS_PER_METER,
};

use crate::coords::world_to_screen;

#[derive(Debug, Clone, Copy)]
enum Element {
    Rectangle {
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        color: Color,
    },
    RectangleOutline {
        x: f32,
        y: f32,
        w: f32,
        h: f32,
    },
    Line {
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
    },
    Circle {
        x: f32,
        y: f32,
        r: f32,
    },
    CircleOutline {
        x: f32,
        y: f32,
        r: f32,
    },
}

const PITCH: Element = Element::Rectangle {
    x: -HALF_PITCH_WIDTH,
    y: HALF_PITCH_HEIGHT,
    w: PITCH_WIDTH,
    h: PITCH_HEIGHT,
    color: color_u8!(126, 175, 52, 255),
};

const BOUNDARY: Element = Element::RectangleOutline {
    x: -HALF_PITCH_WIDTH,
    y: HALF_PITCH_HEIGHT,
    w: PITCH_WIDTH,
    h: PITCH_HEIGHT,
};

const HALFWAY_LINE: Element = Element::Line {
    x1: 0.0,
    y1: HALF_PITCH_HEIGHT,
    x2: 0.0,
    y2: -HALF_PITCH_HEIGHT,
};

const CENTER_CIRCLE: Element = Element::CircleOutline {
    x: 0.0,
    y: 0.0,
    r: CENTER_CIRCLE_RADIUS,
};

const CENTER_SPOT: Element = Element::Circle {
    x: 0.0,
    y: 0.0,
    r: CENTER_SPOT_RADIUS,
};

const PITCH_ELEMENTS: &[Element] = &[PITCH, BOUNDARY, HALFWAY_LINE, CENTER_CIRCLE, CENTER_SPOT];

pub fn draw_pitch() {
    for element in PITCH_ELEMENTS {
        match *element {
            Element::Rectangle { x, y, w, h, color } => {
                let screen = world_to_screen(vec2(x, y));
                draw_rectangle(
                    screen.x,
                    screen.y,
                    w * PIXELS_PER_METER,
                    h * PIXELS_PER_METER,
                    color,
                )
            }
            Element::RectangleOutline { x, y, w, h } => {
                let screen = world_to_screen(vec2(x, y));
                draw_rectangle_lines(
                    screen.x,
                    screen.y,
                    w * PIXELS_PER_METER,
                    h * PIXELS_PER_METER,
                    MARKING_THICKNESS,
                    MARKING_COLOR,
                )
            }
            Element::Line { x1, y1, x2, y2 } => {
                let screen1 = world_to_screen(vec2(x1, y1));
                let screen2 = world_to_screen(vec2(x2, y2));
                draw_line(
                    screen1.x,
                    screen1.y,
                    screen2.x,
                    screen2.y,
                    MARKING_THICKNESS,
                    MARKING_COLOR,
                )
            }
            Element::Circle { x, y, r } => {
                let screen = world_to_screen(vec2(x, y));
                draw_circle(screen.x, screen.y, r * PIXELS_PER_METER, MARKING_COLOR)
            }
            Element::CircleOutline { x, y, r } => {
                let screen = world_to_screen(vec2(x, y));
                draw_circle_lines(
                    screen.x,
                    screen.y,
                    r * PIXELS_PER_METER,
                    MARKING_THICKNESS,
                    MARKING_COLOR,
                )
            }
        }
    }
}
