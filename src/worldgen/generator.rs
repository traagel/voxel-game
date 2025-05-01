use crate::world::chunk::Chunk;
use crate::world::chunk::CHUNK_SIZE;
use crate::world::tile::TILE_SIZE;
use crate::world::zlevel::ZLevel;
use crate::worldgen::biome::BiomeId;
use crate::worldgen::pipeline::{GenCtx, GenStage};
use crate::worldgen::stages::height::HeightStage;
use std::collections::HashMap;

pub struct WorldGenerator {
    stages: Vec<Box<dyn GenStage>>,
}

impl WorldGenerator {
    pub fn new(seed: u32) -> Self {
        Self {
            stages: vec![Box::new(HeightStage::new(seed, 0.005))],
        }
    }

    pub fn generate_chunk(&self, chunk_x: i32, chunk_y: i32) -> Chunk {
        let mut chunk = Chunk::new();
        let mut ctx = GenCtx {
            chunk: &mut chunk,
            world_x0: chunk_x * CHUNK_SIZE as i32 * TILE_SIZE as i32,
            world_y0: chunk_y * CHUNK_SIZE as i32 * TILE_SIZE as i32,
            height: [[0.0; CHUNK_SIZE]; CHUNK_SIZE],
            biome: [[BiomeId::Plains; CHUNK_SIZE]; CHUNK_SIZE],
        };
        for stage in &self.stages {
            stage.execute(&mut ctx);
        }
        chunk
    }

    pub fn from_stages(stages: Vec<Box<dyn GenStage>>) -> Self {
        Self { stages }
    }
}

impl WorldGenerator {
    /// Fills the given world with chunks for every (cx, cy) in `area`.
    pub fn generate_into_world(&self, world: &mut crate::world::world::World, area: &[(i32, i32)]) {
        // make sure z-level 0 has a chunk map
        if world.z_levels.is_empty() {
            world.z_levels.push(ZLevel {
                z: 0,
                chunks: HashMap::new(),
            });
        }
        let z0 = &mut world.z_levels[0];

        for &(cx, cy) in area {
            // skip if chunk already exists
            z0.chunks
                .entry((cx, cy))
                .or_insert_with(|| self.generate_chunk(cx, cy));
        }
    }
}
