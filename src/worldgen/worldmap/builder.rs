use crate::world::worldmap::world_map::WorldMap;
use crate::world::worldmap::biome::{BiomeId, TemperatureType, VegetationType, PrecipitationType, ElevationType};
use crate::worldgen::worldmap::biome::{classify_temperature, classify_vegetation, classify_precipitation, classify_elevation};
use rand::SeedableRng;

use super::params::WorldGenParams;
use super::utils::{erosion::erosion_pass, noise::percentile};
use super::terrain::{continents, elevation, mountains};
use super::climate::{temperature, precipitation, wind, soil, vegetation};
use super::hydrology::{flow, rivers};
use super::civ;
use super::biome;

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
        Self {
            seed,
            width,
            height,
            scale,
            params: params.unwrap_or_default(),
        }
    }

    /// Full pipeline – returns a populated `WorldMap`.
    pub fn generate(&self) -> WorldMap {
        let p = &self.params;

        // === Terrain ===
        let perlin_seed = self.seed;
        let centers = continents::generate_continent_centers(
            perlin_seed,
            self.width,
            self.height,
            p.num_continents.max(1),
        );
        let continent_radius = (self.width.min(self.height) as f64) * 0.33;

        let (mut elevation, mut moisture) = elevation::generate(
            &self.params,
            self.width,
            self.height,
            self.scale,
            self.seed,
            &centers,
            continent_radius,
        );
        mountains::add_ranges(self.seed, self.width, self.height, &mut elevation);
        for _ in 0..p.erosion_iterations {
            erosion_pass(&mut elevation);
        }

        // === Climate ===
        let temperature = temperature::make(&elevation);
        let precipitation = precipitation::make(self.seed, self.width, self.height, self.scale);
        let wind = wind::make(self.seed, self.width, self.height, self.scale);
        let soil = soil::make(&elevation, &precipitation, &vec![vec![false; self.height]; self.width]);
        let vegetation = vegetation::make(&temperature, &precipitation, &soil);

        // === Thresholds ===
        let (sea, coast, mountain) = {
            let mut flat: Vec<f64> = elevation.iter().flatten().copied().collect();
            let sea = percentile(&mut flat, p.ocean_percent);
            let coast = percentile(&mut flat, p.ocean_percent + p.coast_percent);
            let mount = percentile(&mut flat, 1.0 - p.mountain_percent);
            (sea, coast, mount)
        };

        // === Hydrology ===
        let flow = {
            let mut flow = vec![vec![0.0; self.width]; self.height];
            for x in 0..self.width {
                for y in 0..self.height {
                    flow::accumulate_flow(&elevation, &mut flow, x, y, sea);
                }
            }
            flow
        };
        let river_mask = rivers::mask(self, &flow);

        // === Biomes ===
        let ridge = crate::worldgen::worldmap::terrain::ridge_map(self.seed, self.width, self.height, self.scale);
        let biomes = biome::all(
            self,
            &elevation,
            &moisture,
            &river_mask,
            &temperature,
            &precipitation,
            &soil,
            &vegetation,
            &ridge,
            sea,
            coast,
            mountain,
        );

        // === Civilisations & trade ===
        let (civ_map, cities, relations, trade) =
            civ::generate_all(self, &elevation, sea, &biomes, &river_mask);

        // === Category maps (for renderer) ===
        let temperature_map: Vec<Vec<TemperatureType>> = (0..self.width)
            .map(|x| (0..self.height)
                .map(|y| classify_temperature(temperature[x][y]))
                .collect())
            .collect();

        let vegetation_map: Vec<Vec<VegetationType>> = (0..self.width)
            .map(|x| (0..self.height)
                .map(|y| classify_vegetation(vegetation[x][y], temperature[x][y], precipitation[x][y]))
                .collect())
            .collect();

        let precipitation_map: Vec<Vec<PrecipitationType>> = (0..self.width)
            .map(|x| (0..self.height)
                .map(|y| classify_precipitation(precipitation[x][y]))
                .collect())
            .collect();

        let elevation_map: Vec<Vec<ElevationType>> = (0..self.width)
            .map(|x| (0..self.height)
                .map(|y| classify_elevation(elevation[x][y], sea, coast, mountain))
                .collect())
            .collect();

        // === Assemble ===
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
            resources: vec![vec![None; self.height]; self.width], // stub; move to its own module later
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

// Preserve old type name so call‑sites don't break.
pub use WorldMapBuilder as WorldMapGenerator;