use macroquad::math::{Circle, Vec2, vec2};

use crate::ball::Ball;
use crate::constants::{
    BALL_WALL_RESTITUTION, COLLISION_BOUNDARY_HEIGHT, COLLISION_BOUNDARY_WIDTH,
    PLAYER_BALL_RESTITUTION, PLAYER_WALL_RESTITUTION,
};
use crate::environment::Environment;
use crate::player::Player;

pub(crate) struct PhysicsSimulator {
    ball: Ball,
    player: Player,
    environment: Environment,
    locked_normal: Option<Vec2>,
}

impl PhysicsSimulator {
    pub(crate) fn new(ball: Ball, player: Player, environment: Environment) -> Self {
        Self {
            ball,
            player,
            environment,
            locked_normal: None,
        }
    }

    pub(crate) fn ball(&self) -> &Ball {
        &self.ball
    }

    pub(crate) fn player(&self) -> &Player {
        &self.player
    }

    pub(crate) fn step(&mut self, dt: f32) {
        let old_ball_position = self.ball.position;
        let old_player_position = self.player.position;

        self.ball.update(dt, &self.environment);
        self.player.update(dt);

        self.resolve_ball_player_sweep(old_ball_position, old_player_position, dt);
        self.collide_ball_player();
        self.collide_ball_wall();
        self.collide_player_wall();
    }

    fn is_ball_player_colliding(&self) -> bool {
        circles_colliding(
            Circle::new(self.ball.position.x, self.ball.position.y, self.ball.radius),
            Circle::new(
                self.player.position.x,
                self.player.position.y,
                self.player.radius,
            ),
        )
    }

    fn resolve_ball_player_velocity(&mut self, normal: Vec2) {
        let resolved = resolve_velocity(
            self.ball.velocity,
            self.ball.mass,
            self.player.velocity,
            self.player.attributes.mass,
            normal,
            PLAYER_BALL_RESTITUTION,
        );

        self.ball.velocity = resolved.0;
        self.player.velocity = resolved.1;
    }

    fn resolve_ball_player_position(&mut self, normal: Vec2) {
        let resolved = resolve_position(
            Body::new(self.ball.position, self.ball.radius, self.ball.mass),
            Body::new(
                self.player.position,
                self.player.radius,
                self.player.attributes.mass,
            ),
            normal,
        );

        self.ball.position = resolved.0;
        self.player.position = resolved.1;
    }

    // Returns the normal to use this frame: reuses the locked-in normal from
    // an ongoing contact episode if one exists, otherwise computes a fresh one.
    fn contact_normal(&mut self) -> Vec2 {
        if let Some(normal) = self.locked_normal {
            return normal;
        }

        let normal = safe_normal(
            self.ball.position,
            self.ball.velocity,
            self.player.position,
            self.player.velocity,
        );

        self.locked_normal = Some(normal);
        normal
    }

    fn collide_ball_player(&mut self) {
        if !self.is_ball_player_colliding() {
            self.locked_normal = None;
            return;
        }

        let normal = self.contact_normal();

        self.resolve_ball_player_velocity(normal);
        self.resolve_ball_player_position(normal);
    }

    fn collide_ball_wall(&mut self) {
        let x = bounce_component(
            self.ball.position.x,
            self.ball.velocity.x,
            COLLISION_BOUNDARY_WIDTH,
            self.ball.radius,
            BALL_WALL_RESTITUTION,
        );

        let y = bounce_component(
            self.ball.position.y,
            self.ball.velocity.y,
            COLLISION_BOUNDARY_HEIGHT,
            self.ball.radius,
            BALL_WALL_RESTITUTION,
        );

        self.ball.position = vec2(x.0, y.0);
        self.ball.velocity = vec2(x.1, y.1);
    }

    fn collide_player_wall(&mut self) {
        let x = bounce_component(
            self.player.position.x,
            self.player.velocity.x,
            COLLISION_BOUNDARY_WIDTH,
            self.player.radius,
            PLAYER_WALL_RESTITUTION,
        );

        let y = bounce_component(
            self.player.position.y,
            self.player.velocity.y,
            COLLISION_BOUNDARY_HEIGHT,
            self.player.radius,
            PLAYER_WALL_RESTITUTION,
        );

        self.player.position = vec2(x.0, y.0);
        self.player.velocity = vec2(x.1, y.1);
    }

    fn resolve_ball_player_sweep(
        &mut self,
        old_ball_position: Vec2,
        old_player_position: Vec2,
        dt: f32,
    ) {
        let combined_radius = self.ball.radius + self.player.radius;

        let maybe_t = ball_player_sweep_t(
            old_ball_position,
            self.ball.velocity,
            old_player_position,
            self.player.velocity,
            combined_radius,
            dt,
        );

        if let Some(t) = maybe_t {
            self.ball.position = old_ball_position + self.ball.velocity * t * dt;
            self.player.position = old_player_position + self.player.velocity * t * dt;

            let normal = self.contact_normal();

            self.resolve_ball_player_velocity(normal);
            self.resolve_ball_player_position(normal);
        }
    }
}

fn circles_colliding(c1: Circle, c2: Circle) -> bool {
    let p1 = vec2(c1.x, c1.y);
    let p2 = vec2(c2.x, c2.y);
    let distance = (p1 - p2).length();
    distance < c1.r + c2.r
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

fn resolve_velocity(v1: Vec2, m1: f32, v2: Vec2, m2: f32, normal: Vec2, e: f32) -> (Vec2, Vec2) {
    let c1 = decompose(v1, normal);
    let c2 = decompose(v2, normal);

    let new_normal1 = c1.normal - ((1.0 + e) * m2 * (c1.normal - c2.normal)) / (m1 + m2);
    let new_normal2 = c2.normal + ((1.0 + e) * m1 * (c1.normal - c2.normal)) / (m1 + m2);

    (
        new_normal1 * normal + c1.tangential,
        new_normal2 * normal + c2.tangential,
    )
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

fn safe_normal(p1: Vec2, v1: Vec2, p2: Vec2, v2: Vec2) -> Vec2 {
    const MIN_SEPARATION: f32 = 0.01; // meters; below this, position alone can't reliably determine a direction

    let separation = p1 - p2;

    if separation.length() > MIN_SEPARATION {
        return separation.normalize_or_zero();
    }

    let velocity_normal = (v1 - v2).normalize_or_zero();

    if velocity_normal != Vec2::ZERO {
        return velocity_normal;
    }

    Vec2::X // arbitrary fallback
}

fn bounce_component(position: f32, velocity: f32, limit: f32, radius: f32, e: f32) -> (f32, f32) {
    let mut position = position;
    let mut velocity = velocity;

    if position + radius > limit {
        if velocity > 0.0 {
            velocity = -e * velocity;
        }
        position = limit - radius;
    }

    if position - radius < -limit {
        if velocity < 0.0 {
            velocity = -e * velocity;
        }
        position = -limit + radius;
    }

    (position, velocity)
}

fn sweep_t(p: Vec2, v: Vec2, r: f32) -> Option<f32> {
    let a = v.dot(v);

    if a == 0.0 {
        return None;
    }

    let b = 2.0 * p.dot(v);
    let c = p.dot(p) - r * r;

    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        return None;
    }

    let sqrt_d = discriminant.sqrt();
    let t_enter = (-b - sqrt_d) / (2.0 * a);

    if (0.0..=1.0).contains(&t_enter) {
        Some(t_enter)
    } else {
        None
    }
}

fn ball_player_sweep_t(
    old_ball_position: Vec2,
    ball_velocity: Vec2,
    old_player_position: Vec2,
    player_velocity: Vec2,
    combined_radius: f32,
    dt: f32,
) -> Option<f32> {
    let p = old_player_position - old_ball_position;
    let v = (player_velocity - ball_velocity) * dt;

    sweep_t(p, v, combined_radius)
}
