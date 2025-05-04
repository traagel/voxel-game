use bevy_ecs::prelude::*;
use crate::ecs::components::{creature::Creature, position::Position};
use crate::ecs::resources::world::WorldRes;

pub fn run_ai(
    mut creatures: Query<(&mut Creature, &mut Position)>,
    world: Res<WorldRes>,
) {
    for (mut creature, mut pos) in &mut creatures {
        // TODO: Implement AI logic (find dig target, move, dig, etc.)
        // Example:
        // if creature.dig_target.is_none() {
        //     creature.find_nearest_dig_target(&world.0);
        // }
        // creature.move_toward_target(&world.0, &mut pos);
    }
} 