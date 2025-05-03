use super::{classifiers::biome, models::TileEnv};
use crate::world::worldmap::biome::BiomeId;

pub fn classify_world(
    elevation: &[Vec<f64>],
    moisture: &[Vec<f64>],
    river:     &[Vec<bool>],
    temp:      &[Vec<f64>],
    precip:    &[Vec<f64>],
    soil:      &[Vec<f64>],
    veg:       &[Vec<f64>],
    ridge:     &[Vec<f64>],
    sea: f64, coast: f64, mountain: f64,
) -> Vec<Vec<BiomeId>> {

    let (w, h) = (elevation.len(), elevation[0].len());
    let mut out = vec![vec![BiomeId::Plains; h]; w];

    for x in 0..w {
        for y in 0..h {
            let env = TileEnv {
                elev: elevation[x][y],
                ridge: ridge[x][y],
                moisture: moisture[x][y],
                temp: temp[x][y],
                precip: precip[x][y],
                soil: soil[x][y],
                veg: veg[x][y],
                sea, coast, mountain,
                river_here: river[x][y],
            };
            out[x][y] = biome(&env);
        }
    }
    out
} 