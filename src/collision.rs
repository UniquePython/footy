use macroquad::math::{Circle, Vec2, vec2};

use crate::ball::Ball;
use crate::player::Player;

fn circles_colliding(c1: Circle, c2: Circle) -> bool {
    let p1 = vec2(c1.x, c1.y);
    let p2 = vec2(c2.x, c2.y);
    let distance = (p1 - p2).length();
    distance < c1.r + c2.r
}

fn is_ball_player_colliding(ball: &Ball, player: &Player) -> bool {
    circles_colliding(
        Circle::new(ball.position.x, ball.position.y, ball.radius),
        Circle::new(player.position.x, player.position.y, player.radius),
    )
}

#[derive(Debug, Clone, Copy)]
struct Component {
    normal: f32,
    tangential: Vec2,
}

fn decompose(vec: Vec2, normal: Vec2) -> Component {
    let normal_component = vec.dot(normal);
    let tangential_component = vec - normal_component * normal;
    Component {
        normal: normal_component,
        tangential: tangential_component,
    }
}

fn resolve_velocity(v1: Vec2, m1: f32, v2: Vec2, m2: f32, normal: Vec2) -> (Vec2, Vec2) {
    let c1 = decompose(v1, normal);
    let c2 = decompose(v2, normal);

    let new_normal1 = ((m1 - m2) * c1.normal + 2.0 * m2 * c2.normal) / (m1 + m2);
    let new_normal2 = ((m2 - m1) * c2.normal + 2.0 * m1 * c1.normal) / (m1 + m2);

    (
        new_normal1 * normal + c1.tangential,
        new_normal2 * normal + c2.tangential,
    )
}

fn resolve_ball_player_velocity(ball: &mut Ball, player: &mut Player, normal: Vec2) {
    let resolved = resolve_velocity(
        ball.velocity,
        ball.mass,
        player.velocity,
        player.attributes.mass,
        normal,
    );

    ball.velocity = resolved.0;
    player.velocity = resolved.1;
}

#[derive(Debug, Clone, Copy)]
struct Body {
    shape: Circle,
    mass: f32,
}

impl Body {
    fn new(position: Vec2, radius: f32, mass: f32) -> Self {
        Self {
            shape: Circle::new(position.x, position.y, radius),
            mass,
        }
    }
}

fn resolve_position(body1: Body, body2: Body, normal: Vec2) -> (Vec2, Vec2) {
    let p1 = vec2(body1.shape.x, body1.shape.y);
    let p2 = vec2(body2.shape.x, body2.shape.y);

    let overlap = (body1.shape.r + body2.shape.r) - (p1 - p2).length();

    if overlap.is_sign_negative() {
        return (p1, p2);
    }

    let move1 = overlap * (body2.mass / (body1.mass + body2.mass));
    let move2 = overlap * (body1.mass / (body1.mass + body2.mass));

    (p1 - normal * move1, p2 + normal * move2)
}

fn resolve_ball_player_position(ball: &mut Ball, player: &mut Player, normal: Vec2) {
    let resolved = resolve_position(
        Body::new(ball.position, ball.radius, ball.mass),
        Body::new(player.position, player.radius, player.attributes.mass),
        normal,
    );

    ball.position = resolved.0;
    player.position = resolved.1;
}

pub(crate) fn collide_ball_player(ball: &mut Ball, player: &mut Player) {
    if !is_ball_player_colliding(ball, player) {
        return;
    }

    let normal = (ball.position - player.position).normalize_or_zero();

    resolve_ball_player_velocity(ball, player, normal);
    resolve_ball_player_position(ball, player, normal);
}
