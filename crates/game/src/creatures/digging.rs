use crate::particle::Particle;
use crate::world::localmap::terrain_material::TerrainMaterial;
use crate::world::localmap::world::World;
use macroquad::prelude::*;
use macroquad::rand::gen_range;
use super::Creature;

impl Creature {
    pub fn dig_if_close(&mut self, world: &mut World, particles: &mut Vec<Particle>) {
        if let Some((target_x, target_y)) = self.target {
            let dx = self.x - target_x as f32;
            let dy = self.y - target_y as f32;
            let dist_sq = dx * dx + dy * dy;
            if dist_sq < 4.0 {
                if self.dig_cooldown > 0 {
                    self.dig_cooldown -= 1;
                    return;
                }
                self.dig_cooldown = 10;
                let radius = 2;
                for dx in -radius..=radius {
                    for dy in -radius..=radius {
                        let px = target_x + dx;
                        let py = target_y + dy;
                        if let Some(subpixel) = world.get_subpixel_mut(px, py) {
                            if subpixel.material != TerrainMaterial::Air {
                                subpixel.material = TerrainMaterial::Dirt;
                            }
                            subpixel.dig_target = false;
                            // spawn a particle
                            if gen_range(0, 100) < 10 {
                                // 10% chance per subpixel
                                particles.push(Particle {
                                    x: px as f32 + gen_range(0.0, 1.0),
                                    y: py as f32 + gen_range(0.0, 1.0),
                                    dx: gen_range(-0.5, 0.5),
                                    dy: gen_range(-1.0, -0.2),
                                    life: 20,
                                    color: GREEN,
                                });
                            }
                        }
                    }
                }
                self.target = None;
            }
        }
    }
} 