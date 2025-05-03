use crate::world::worldmap::biome::{
    BiomeId, ElevationType, PrecipitationType, TemperatureType, VegetationType,
};
use crate::world::worldmap::world_map::WorldMap;
use crate::worldgen::worldmap::biome as biome_classifiers;
use rand::SeedableRng;

use super::biome;
use super::civ;
use super::climate::{precipitation, soil, temperature, vegetation, wind};
use super::hydrology::{flow, rivers};
use super::params::WorldGenParams;
use super::terrain::{continents, elevation, mountains};
use super::utils::{erosion::erosion_pass, noise::percentile};
use crate::worldgen::worldmap::hydrology::lakes;
use crate::worldgen::worldmap::terrain::elevation::{
    craters::random_craters, noise_sources::NoiseSources,
};

pub struct WorldMapBuilder {
    pub seed: u32,
    pub width: usize,
    pub height: usize,
    pub scale: f64,
    pub params: WorldGenParams,
}

impl WorldMapBuilder {
    pub fn new(
        seed: u32,
        width: usize,
        height: usize,
        scale: f64,
        params: Option<WorldGenParams>,
    ) -> Self {
        Self { seed, width, height, scale, params: params.unwrap_or_default() }
    }

    /// Full pipeline – returns a populated `WorldMap`.
    pub fn generate(&self) -> WorldMap {
        // Settings
        println!("Generating world with settings:");
        println!("  seed: {} width: {} height: {} scale: {}", self.seed, self.width, self.height, self.scale);
        println!("  params: {:#?}", self.params);
        let p = &self.params;

        // === Terrain ===
        let centers = continents::generate_continent_centers(self.seed, self.width, self.height, p.num_continents.max(1));
        let continent_radius = (self.width.min(self.height) as f64) * 0.33;
        let (mut elevation, mut moisture) = elevation::generate(p, self.width, self.height, self.scale, self.seed, &centers, continent_radius);
        mountains::add_ranges(self.seed, self.width, self.height, &mut elevation);
        for _ in 0..p.erosion_iterations { erosion_pass(&mut elevation); }

        // === Initial thresholds for flow accumulation ===
        let mut flat_init: Vec<f64> = elevation.iter().flatten().copied().collect();
        let sea = percentile(&mut flat_init, p.ocean_percent);
        let _coast = percentile(&mut flat_init, p.ocean_percent + p.coast_percent);
        let _mountain = percentile(&mut flat_init, 1.0 - p.mountain_percent);

        // === Hydrology: Craters & Noise ===
        let craters = random_craters(self.seed, self.width, self.height, p.num_craters);
        let noise = NoiseSources::new(self.seed);

        // === Flow accumulation using computed sea level ===
        let mut flow = vec![vec![0.0; self.height]; self.width];
        for x in 0..self.width {
            for y in 0..self.height {
                flow::accumulate_flow(&elevation, &mut flow, x, y, sea);
            }
        }

        // === Lakes carve elevation ===
        let _lake_mask = lakes::apply_lakes(&mut elevation, &flow, &craters, &noise, p.river_threshold);

        // === Elevation contrast stretch ===
        let mut min_e = f64::INFINITY;
        let mut max_e = f64::NEG_INFINITY;
        for col in &elevation {
            for &v in col {
                if v < min_e { min_e = v; }
                if v > max_e { max_e = v; }
            }
        }
        let range = max_e - min_e;
        if range > 0.0 {
            for x in 0..self.width {
                for y in 0..self.height {
                    elevation[x][y] = (elevation[x][y] - min_e) / range;
                }
            }
        }

        // === Final thresholds for biomes and map generation ===
        let (sea, coast, mountain) = {
            let mut flat: Vec<f64> = elevation.iter().flatten().copied().collect();
            let sea = percentile(&mut flat, p.ocean_percent);
            let coast = percentile(&mut flat, p.ocean_percent + p.coast_percent);
            let mount = percentile(&mut flat, 1.0 - p.mountain_percent);
            (sea, coast, mount)
        };

        // === Climate ===
        let temperature = temperature::make(&elevation);
        let precipitation = precipitation::make(self.seed, self.width, self.height, self.scale, &elevation);
        let wind = wind::make(self.seed, self.width, self.height, self.scale);
        let soil = soil::make(&elevation, &precipitation, &vec![vec![false; self.height]; self.width]);
        let vegetation = vegetation::make(&temperature, &precipitation, &soil);

        // === Hydrology: River mask ===
        let river_mask = rivers::mask(self, &flow);

        // === Biomes ===
        let ridge = crate::worldgen::worldmap::terrain::elevation::ridge_map(self.seed, self.width, self.height, self.scale);
        let biomes = biome::classify_world(&elevation, &moisture, &river_mask, &temperature, &precipitation, &soil, &vegetation, &ridge, sea, coast, mountain);

        // === Biome counts print ===
        println!("Biome counts:");
        {
            use std::collections::HashMap;
            let mut counts = HashMap::new();
            for row in &biomes { for &b in row { *counts.entry(b).or_insert(0) += 1; }}
            for (b, c) in counts { println!("  {:?}: {}", b, c); }
        }

        // === Civilisations & trade ===
        let (civ_map, cities, relations, trade) = civ::generate_all(self, &elevation, sea, &biomes, &river_mask);

        // === Category maps ===
        let temperature_map = (0..self.width)
            .map(|x| (0..self.height).map(|y| biome_classifiers::temperature(temperature[x][y])).collect())
            .collect();
        let vegetation_map = (0..self.width)
            .map(|x| (0..self.height).map(|y| biome_classifiers::vegetation(vegetation[x][y], temperature[x][y], precipitation[x][y])).collect())
            .collect();
        let precipitation_map = (0..self.width)
            .map(|x| (0..self.height).map(|y| biome_classifiers::precipitation(precipitation[x][y])).collect())
            .collect();
        let elevation_map = (0..self.width)
            .map(|x| (0..self.height).map(|y| biome_classifiers::elevation(
                elevation[x][y],
                &biome_classifiers::models::TileEnv{ elev:elevation[x][y], sea, coast, mountain, ridge:0.0, moisture:0.0, temp:0.0, precip:0.0, soil:0.0, veg:0.0, river_here:false }
            )).collect())
            .collect();

        WorldMap {
            width: self.width,
            height: self.height,
            biomes,
            elevation,
            moisture,
            rivers: river_mask,
            temperature,
            precipitation,
            soil_fertility: soil,
            vegetation,
            wind_direction: wind,
            resources: vec![vec![None; self.height]; self.width],
            temperature_map,
            vegetation_map,
            precipitation_map,
            elevation_map,
            civilization_map: civ_map,
            cities,
            civ_relations: relations,
            trade_routes: trade,
            sea_level: sea,
        }
    }
}

// Preserve old type name
pub use WorldMapBuilder as WorldMapGenerator;
