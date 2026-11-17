use macroquad::math::Vec2;

use crate::constants::{
    BALL_CROSS_SECTION_AREA, BALL_DIAMETER, BALL_MASS, CD_SUBCRITICAL, CD_SUPERCRITICAL,
    RE_CRISIS_END, RE_CRISIS_START,
};
use crate::environment::Environment;

fn smoothstep(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

// Drag coefficient as a function of Reynolds number: high before the drag crisis,
// low after it, with a smooth transition in between.
fn drag_coefficient(reynolds: f32) -> f32 {
    let t = smoothstep((reynolds - RE_CRISIS_START) / (RE_CRISIS_END - RE_CRISIS_START));
    CD_SUBCRITICAL + (CD_SUPERCRITICAL - CD_SUBCRITICAL) * t
}

// Deceleration (m/s^2) on the ball from air drag.
// `relative_velocity` is the ball's velocity relative to the air (ball velocity minus wind).
pub(crate) fn drag_acceleration(relative_velocity: Vec2, env: &Environment) -> Vec2 {
    let speed = relative_velocity.length();

    let reynolds_per_speed = env.air_density * BALL_DIAMETER / env.air_viscosity;
    let cd = drag_coefficient(reynolds_per_speed * speed);

    let k = 0.5 * env.air_density * cd * BALL_CROSS_SECTION_AREA / BALL_MASS;

    // a = -k * |v| * v: magnitude k*v^2, direction opposite to the relative velocity
    -k * speed * relative_velocity
}
