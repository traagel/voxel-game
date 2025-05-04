use bevy_ecs::prelude::*;
use crate::ecs::components::particle::Particle;

pub fn tick_particles(mut query: Query<&mut Particle>) {
    for mut p in &mut query {
        p.x += p.dx;
        p.y += p.dy;
        p.dy += 0.05;
        p.life = p.life.saturating_sub(1);
    }
    // Despawning dead particles can be handled in a separate system
} 