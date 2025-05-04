use bevy_ecs::prelude::*;

#[derive(Component, Copy, Clone, Debug, Default)]
pub struct Position {
    pub x: f32,
    pub y: f32,
} 