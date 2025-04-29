use crate::world::chunk::Chunk;
use crate::world::terrain_material::TerrainMaterial;
use crate::world::world::World;
use noise::{NoiseFn, Perlin};

pub struct WorldGenerator {
    pub seed: u32,
}

impl WorldGenerator {
    pub fn new(seed: u32) -> Self {
        Self { seed }
    }

    pub fn generate_chunk(&self, chunk_x: i32, chunk_y: i32) -> Chunk {
        let mut chunk = Chunk::new();

        let perlin = Perlin::new(self.seed);
        let scale = 0.005; // adjust for larger/smaller land features

        for tile_x in 0..chunk.tiles.len() {
            for tile_y in 0..chunk.tiles[tile_x].len() {
                for sub_x in 0..8 {
                    for sub_y in 0..8 {
                        let world_x = chunk_x * 32 * 8 + tile_x as i32 * 8 + sub_x as i32;
                        let world_y = chunk_y * 32 * 8 + tile_y as i32 * 8 + sub_y as i32;

                        let noise_val =
                            perlin.get([world_x as f64 * scale, world_y as f64 * scale]);

                        let material = if noise_val > 0.6 {
                            TerrainMaterial::Rock // mountain
                        } else if noise_val > 0.3 {
                            TerrainMaterial::Dirt // hills
                        } else {
                            TerrainMaterial::Dirt // flatland
                        };

                        chunk.tiles[tile_x][tile_y].subgrid[sub_x][sub_y] =
                            crate::world::subpixel::Subpixel {
                                material,
                                dig_target: false,
                            };
                    }
                }
            }
        }

        chunk
    }

    pub fn generate_into_world(&self, world: &mut World, chunk_coords: &[(i32, i32)]) {
        for &(cx, cy) in chunk_coords {
            let chunk = self.generate_chunk(cx, cy);
            world.z_levels[0].chunks.insert((cx, cy), chunk);
        }
    }
}
