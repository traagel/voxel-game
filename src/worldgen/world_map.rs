use crate::worldgen::biome::BiomeId;
use noise::{NoiseFn, Perlin};

pub struct WorldMap {
    pub width: usize,
    pub height: usize,
    pub biomes: Vec<Vec<BiomeId>>,
    // You can add more fields later: elevation, rainfall, etc.
}

pub struct WorldMapGenerator {
    pub seed: u32,
    pub width: usize,
    pub height: usize,
    pub scale: f64,
}

impl WorldMapGenerator {
    pub fn new(seed: u32, width: usize, height: usize, scale: f64) -> Self {
        Self { seed, width, height, scale }
    }

    pub fn generate(&self) -> WorldMap {
        let perlin = Perlin::new(self.seed);
        let mut biomes = vec![vec![BiomeId::Plains; self.height]; self.width];
        for x in 0..self.width {
            for y in 0..self.height {
                let n = perlin.get([
                    x as f64 * self.scale,
                    y as f64 * self.scale,
                ]);
                biomes[x][y] = if n > 0.3 {
                    BiomeId::Mountain
                } else {
                    BiomeId::Plains
                };
            }
        }
        WorldMap {
            width: self.width,
            height: self.height,
            biomes,
        }
    }
} 