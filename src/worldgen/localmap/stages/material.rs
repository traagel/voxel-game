use crate::worldgen::localmap::pipeline::{GenCtx, GenStage};
use crate::world::localmap::terrain_material::TerrainMaterial;
use crate::world::localmap::chunk::CHUNK_SIZE;
use crate::world::localmap::tile::TILE_SIZE;
use noise::{NoiseFn, Perlin};

pub struct MaterialStage {
    perlin: Perlin,
    scale: f64,
    pub dirt_height: f32,
}

impl MaterialStage {
    pub fn new(seed: u32, scale: f64, dirt_height: f32) -> Self {
        Self {
            perlin: Perlin::new(seed),
            scale,
            dirt_height,
        }
    }
}

impl GenStage for MaterialStage {
    fn execute(&self, ctx: &mut GenCtx) {
        // Debug: print min/max subpixel noise for this chunk
        let mut min_h = f32::MAX;
        let mut max_h = f32::MIN;
        for tx in 0..CHUNK_SIZE {
            for ty in 0..CHUNK_SIZE {
                for sx in 0..TILE_SIZE {
                    for sy in 0..TILE_SIZE {
                        let wx = ctx.world_x0 + (tx * TILE_SIZE + sx) as i32;
                        let wy = ctx.world_y0 + (ty * TILE_SIZE + sy) as i32;
                        let sub_height = ((self.perlin.get([
                            wx as f64 * self.scale,
                            wy as f64 * self.scale,
                        ]) + 1.0) * 0.5) as f32;
                        if sub_height < min_h { min_h = sub_height; }
                        if sub_height > max_h { max_h = sub_height; }
                        let sub = &mut ctx.chunk.tiles[tx][ty].subgrid[sx][sy];
                        if sub_height < self.dirt_height {
                            sub.material = TerrainMaterial::Dirt;
                        } else {
                            sub.material = TerrainMaterial::Air;
                        }
                    }
                }
            }
        }
        println!(
            "MaterialStage: chunk ({}, {}) subpixel noise min: {:.3}, max: {:.3}",
            ctx.world_x0, ctx.world_y0, min_h, max_h
        );
    }
} 