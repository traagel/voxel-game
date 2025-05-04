use bevy_ecs::prelude::*;
use crate::ecs::components::{spatial::position::Position, spatial::velocity::Velocity, render::renderable::Renderable};

#[derive(Component, Debug)]
pub struct Creature {
    pub dig_target: Option<(i32, i32)>,
    // Add other fields as needed (color, etc.)
}

impl Creature {
    pub fn at(x: f32, y: f32) -> (Position, Velocity, Creature, Renderable) {
        (
            Position { x, y },
            Velocity::default(),
            Creature { dig_target: None },
            Renderable::creature(),
        )
    }
} 