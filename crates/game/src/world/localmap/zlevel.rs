use crate::world::localmap::chunk::Chunk;
use std::collections::HashMap;

pub struct ZLevel {
    pub z: i32,
    pub chunks: HashMap<(i32, i32), Chunk>,
}

impl ZLevel {
    pub fn new(z: i32) -> Self {
        Self {
            z,
            chunks: HashMap::new(),
        }
    }
}
