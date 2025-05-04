use bevy_ecs::prelude::*;
use crate::ecs::resources::{
    particle::ParticlesRes,
    game_view::GameViewRes,
    game_view::GameView,
};

pub fn update_particles(
    mut particles: ResMut<ParticlesRes>,
    game_view: Res<GameViewRes>,
) {
    // Skip updates if not in local map view
    if !matches!(game_view.active_view, GameView::LocalMap) {
        return;
    }

    for p in &mut particles.0 {
        p.x += p.dx;
        p.y += p.dy;
        p.dy += 0.05;
        p.life = p.life.saturating_sub(1);
    }
    particles.0.retain(|p| p.life > 0);
} 