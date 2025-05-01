use crate::world::localmap::chunk::Chunk;
use crate::world::localmap::chunk::CHUNK_SIZE;
use crate::world::worldmap::biome::BiomeId;

pub struct GenCtx<'a> {
    pub chunk: &'a mut Chunk,
    pub world_x0: i32,
    pub world_y0: i32,
    pub height: [[f32; CHUNK_SIZE]; CHUNK_SIZE],
    pub biome: [[BiomeId; CHUNK_SIZE]; CHUNK_SIZE],
}

pub trait GenStage: Send + Sync {
    fn execute(&self, ctx: &mut GenCtx);
}
