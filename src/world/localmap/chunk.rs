use crate::world::localmap::tile::Tile;

pub const CHUNK_SIZE: usize = 32;

pub struct Chunk {
    pub tiles: [[Tile; CHUNK_SIZE]; CHUNK_SIZE],
    pub dirty: bool,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            tiles: [[Tile::default(); CHUNK_SIZE]; CHUNK_SIZE],
            dirty: false,
        }
    }
}
