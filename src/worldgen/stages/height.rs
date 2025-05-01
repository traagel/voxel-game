use super::super::pipeline::{GenCtx, GenStage};
use crate::world::chunk::CHUNK_SIZE;
use crate::world::tile::TILE_SIZE;

use noise::{NoiseFn, Perlin};
pub struct HeightStage {
    perlin: Perlin,
    scale: f64,
}

impl HeightStage {
    pub fn new(seed: u32, scale: f64) -> Self {
        Self {
            perlin: Perlin::new(seed),
            scale,
        }
    }
}

impl GenStage for HeightStage {
    fn execute(&self, ctx: &mut GenCtx) {
        for tx in 0..CHUNK_SIZE {
            for ty in 0..CHUNK_SIZE {
                let wx = ctx.world_x0 + (tx * TILE_SIZE) as i32;
                let wy = ctx.world_y0 + (ty * TILE_SIZE) as i32;
                ctx.height[tx][ty] = ((self.perlin.get([wx as f64 * self.scale, wy as f64 * self.scale]) + 1.0) * 0.5) as f32;
            }
        }
    }
}
