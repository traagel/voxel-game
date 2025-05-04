use bevy_ecs::prelude::*;
use crate::ecs::components::{position::Position, velocity::Velocity};

pub fn movement(mut query: Query<(&mut Position, &Velocity)>) {
    // TODO: Replace with actual delta time from a resource
    let dt = 1.0;
    for (mut pos, vel) in &mut query {
        pos.x += vel.dx * dt;
        pos.y += vel.dy * dt;
    }
} 