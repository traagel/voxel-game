use bevy_ecs::prelude::*;
use crate::ecs::{
    components::creature::Creature,
    resources::{
        world::WorldRes,
        game_view::GameViewRes,
        game_view::GameView,
        particle::ParticlesRes,
    },
};
use crate::world::localmap::world::World;
use crate::particle::Particle;

// Simple path finding for creatures
fn find_nearest_dig_target(creature: &mut Creature, world: &World) {
    // Basic target finding logic - just set a random nearby target
    let x = 5; // Random x coordinate
    let y = 5; // Random y coordinate
    creature.dig_target = Some((x, y));
}

// Simple movement logic
fn move_toward_target(creature: &mut Creature, world: &mut World) {
    // Just a stub for now - would implement actual pathfinding
    if let Some(_target) = creature.dig_target {
        // Move logic would go here
    }
}

// Simple digging logic
fn dig_if_close(creature: &mut Creature, world: &mut World, particles: &mut Vec<Particle>) {
    // Just a stub for now - would implement digging
    if let Some(_target) = creature.dig_target {
        // Digging logic would go here
    }
}

pub fn update_creatures(
    mut creatures: Query<&mut Creature>,
    mut world: ResMut<WorldRes>,
    mut particles: ResMut<ParticlesRes>,
    game_view: Res<GameViewRes>,
) {
    // Skip updates if not in local map view
    if !matches!(game_view.active_view, GameView::LocalMap) {
        return;
    }

    for mut creature in &mut creatures {
        if creature.dig_target.is_none() {
            find_nearest_dig_target(&mut creature, &world.0);
        }
        move_toward_target(&mut creature, &mut world.0);
        dig_if_close(&mut creature, &mut world.0, &mut particles.0);
    }
} 