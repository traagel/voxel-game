use bevy_ecs::prelude::*;

#[derive(Component, Copy, Clone, Debug, Default)]
pub struct Velocity {
    pub dx: f32,
    pub dy: f32,
} 