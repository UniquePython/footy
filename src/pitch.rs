use macroquad::color::Color;
use macroquad::math::{Vec2, vec2};
use macroquad::shapes::{draw_arc, draw_circle, draw_line, draw_poly_lines, draw_rectangle};

use crate::constants::{
    CENTER_CIRCLE_RADIUS, CORNER_ARC_RADIUS, GOAL_AREA_HEIGHT, GOAL_AREA_WIDTH, HALF_PITCH_HEIGHT,
    HALF_PITCH_WIDTH, MARKING_COLOR, MARKING_THICKNESS, PENALTY_ARC_HALF_ANGLE, PENALTY_ARC_RADIUS,
    PENALTY_ARC_SWEEP, PENALTY_AREA_HEIGHT, PENALTY_AREA_WIDTH, PENALTY_SPOT_DISTANCE, PITCH_COLOR,
    PITCH_HEIGHT, PITCH_WIDTH, SPOT_RADIUS,
};

const CURVE_SIDES: u8 = 255; // segments per full circle; macroquad's default of 30 is visibly polygonal when zoomed in

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
    Arc {
        x: f32,
        y: f32,
        r: f32,
        rotation: f32, // degrees, starting angle
        sweep: f32,    // degrees, how much of the circle to draw
    },
}

const PITCH: Element = Element::Rectangle {
    x: -HALF_PITCH_WIDTH,
    y: HALF_PITCH_HEIGHT,
    w: PITCH_WIDTH,
    h: PITCH_HEIGHT,
    color: PITCH_COLOR,
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
    r: SPOT_RADIUS,
};

const RIGHT_PENALTY_AREA: Element = Element::RectangleOutline {
    x: HALF_PITCH_WIDTH - PENALTY_AREA_WIDTH,
    y: PENALTY_AREA_HEIGHT / 2.0,
    w: PENALTY_AREA_WIDTH,
    h: PENALTY_AREA_HEIGHT,
};

const LEFT_PENALTY_AREA: Element = Element::RectangleOutline {
    x: -HALF_PITCH_WIDTH,
    y: PENALTY_AREA_HEIGHT / 2.0,
    w: PENALTY_AREA_WIDTH,
    h: PENALTY_AREA_HEIGHT,
};

const RIGHT_GOAL_AREA: Element = Element::RectangleOutline {
    x: HALF_PITCH_WIDTH - GOAL_AREA_WIDTH,
    y: GOAL_AREA_HEIGHT / 2.0,
    w: GOAL_AREA_WIDTH,
    h: GOAL_AREA_HEIGHT,
};

const LEFT_GOAL_AREA: Element = Element::RectangleOutline {
    x: -HALF_PITCH_WIDTH,
    y: GOAL_AREA_HEIGHT / 2.0,
    w: GOAL_AREA_WIDTH,
    h: GOAL_AREA_HEIGHT,
};

const RIGHT_PENALTY_SPOT: Element = Element::Circle {
    x: HALF_PITCH_WIDTH - PENALTY_SPOT_DISTANCE,
    y: 0.0,
    r: SPOT_RADIUS,
};

const LEFT_PENALTY_SPOT: Element = Element::Circle {
    x: -HALF_PITCH_WIDTH + PENALTY_SPOT_DISTANCE,
    y: 0.0,
    r: SPOT_RADIUS,
};

const TOP_LEFT_CORNER_ARC: Element = Element::Arc {
    x: -HALF_PITCH_WIDTH,
    y: HALF_PITCH_HEIGHT,
    r: CORNER_ARC_RADIUS,
    rotation: 270.0,
    sweep: 90.0,
};

const TOP_RIGHT_CORNER_ARC: Element = Element::Arc {
    x: HALF_PITCH_WIDTH,
    y: HALF_PITCH_HEIGHT,
    r: CORNER_ARC_RADIUS,
    rotation: 180.0,
    sweep: 90.0,
};

const BOTTOM_RIGHT_CORNER_ARC: Element = Element::Arc {
    x: HALF_PITCH_WIDTH,
    y: -HALF_PITCH_HEIGHT,
    r: CORNER_ARC_RADIUS,
    rotation: 90.0,
    sweep: 90.0,
};

const BOTTOM_LEFT_CORNER_ARC: Element = Element::Arc {
    x: -HALF_PITCH_WIDTH,
    y: -HALF_PITCH_HEIGHT,
    r: CORNER_ARC_RADIUS,
    rotation: 0.0,
    sweep: 90.0,
};

const RIGHT_PENALTY_ARC: Element = Element::Arc {
    x: HALF_PITCH_WIDTH - PENALTY_SPOT_DISTANCE,
    y: 0.0,
    r: PENALTY_ARC_RADIUS,
    rotation: 180.0 - PENALTY_ARC_HALF_ANGLE,
    sweep: PENALTY_ARC_SWEEP,
};

const LEFT_PENALTY_ARC: Element = Element::Arc {
    x: -HALF_PITCH_WIDTH + PENALTY_SPOT_DISTANCE,
    y: 0.0,
    r: PENALTY_ARC_RADIUS,
    rotation: 360.0 - PENALTY_ARC_HALF_ANGLE,
    sweep: PENALTY_ARC_SWEEP,
};

const PITCH_ELEMENTS: &[Element] = &[
    PITCH,
    BOUNDARY,
    HALFWAY_LINE,
    CENTER_CIRCLE,
    CENTER_SPOT,
    LEFT_PENALTY_AREA,
    RIGHT_PENALTY_AREA,
    LEFT_GOAL_AREA,
    RIGHT_GOAL_AREA,
    LEFT_PENALTY_SPOT,
    RIGHT_PENALTY_SPOT,
    TOP_LEFT_CORNER_ARC,
    TOP_RIGHT_CORNER_ARC,
    BOTTOM_RIGHT_CORNER_ARC,
    BOTTOM_LEFT_CORNER_ARC,
    RIGHT_PENALTY_ARC,
    LEFT_PENALTY_ARC,
];

fn draw_marking(from: Vec2, to: Vec2) {
    draw_line(from.x, from.y, to.x, to.y, MARKING_THICKNESS, MARKING_COLOR);
}

pub fn draw_pitch() {
    for element in PITCH_ELEMENTS {
        match *element {
            Element::Rectangle { x, y, w, h, color } => {
                // `y` is the top edge, but draw_rectangle wants the bottom-left corner in a Y-up world
                draw_rectangle(x, y - h, w, h, color);
            }
            Element::RectangleOutline { x, y, w, h } => {
                let top_left = vec2(x, y);
                let top_right = vec2(x + w, y);
                let bottom_right = vec2(x + w, y - h);
                let bottom_left = vec2(x, y - h);

                draw_marking(top_left, top_right);
                draw_marking(top_right, bottom_right);
                draw_marking(bottom_right, bottom_left);
                draw_marking(bottom_left, top_left);
            }
            Element::Line { x1, y1, x2, y2 } => {
                draw_marking(vec2(x1, y1), vec2(x2, y2));
            }
            Element::Circle { x, y, r } => {
                draw_circle(x, y, r, MARKING_COLOR);
            }
            Element::CircleOutline { x, y, r } => {
                draw_poly_lines(x, y, CURVE_SIDES, r, 0.0, MARKING_THICKNESS, MARKING_COLOR);
            }
            Element::Arc {
                x,
                y,
                r,
                rotation,
                sweep,
            } => {
                draw_arc(
                    x,
                    y,
                    CURVE_SIDES,
                    r,
                    rotation,
                    MARKING_THICKNESS,
                    sweep,
                    MARKING_COLOR,
                );
            }
        }
    }
}
