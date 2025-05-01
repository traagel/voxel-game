use crate::world::worldmap::biome::BiomeId;

#[derive(Debug, Clone)]
pub struct WorldMap {
    pub width: usize,
    pub height: usize,
    pub biomes: Vec<Vec<BiomeId>>,
    pub elevation: Vec<Vec<f64>>,
    pub moisture: Vec<Vec<f64>>,
    pub rivers: Vec<Vec<bool>>,
    // You can add more fields later: elevation, rainfall, etc.
} 