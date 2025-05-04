use bevy_ecs::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct Particle {
    pub x: f32,
    pub y: f32,
    pub dx: f32,
    pub dy: f32,
    pub life: u32,
} 