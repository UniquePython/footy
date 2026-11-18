#[macro_use]
mod macros;

mod action;
mod aerodynamics;
mod attributes;
mod ball;
mod camera;
mod constants;
mod controller;
mod drawing;
mod environment;
mod facing_turn;
mod game;
mod input;
mod physics_simulator;
mod pitch;
mod player;
mod stands;

pub use constants::{DEFAULT_WINDOW_HEIGHT, DEFAULT_WINDOW_WIDTH};
pub use game::run;
