use crate::creatures::Creature;
use crate::particle::Particle;
use crate::world::localmap::world::World;
use crate::world::localmap::terrain_material::TerrainMaterial;
use macroquad::rand::gen_range;
use macroquad::prelude::RED;

pub fn find_spawn_point(world: &World) -> Option<(f32, f32)> {
    let tries = 100;
    for _ in 0..tries {
        let x = gen_range(0, 256) as i32;
        let y = gen_range(0, 256) as i32;
        let material = world.get_material_at(x, y);
        if material == TerrainMaterial::Dirt {
            return Some((x as f32, y as f32));
        }
    }
    None
}

pub fn spawn_creatures(world: &World, count: usize) -> Vec<Creature> {
    let mut creatures = Vec::new();
    
    for _ in 0..count {
        if let Some((spawn_x, spawn_y)) = find_spawn_point(world) {
            creatures.push(Creature::new(spawn_x, spawn_y, 2.0, RED));
        }
    }
    
    creatures
}

pub fn update_creatures(creatures: &mut [Creature], world: &mut World, particles: &mut Vec<Particle>) {
    for creature in creatures {
        if creature.target.is_none() {
            creature.find_nearest_dig_target(world);
        }
        creature.move_toward_target(world);
        creature.dig_if_close(world, particles);
    }
} 