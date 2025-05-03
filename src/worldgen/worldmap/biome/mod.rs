// Biome classification logic
use crate::world::worldmap::biome::{BiomeId, TemperatureType, VegetationType, PrecipitationType, ElevationType};
use noise::Perlin;

// Category classification functions
pub fn classify_temperature(temp: f64) -> TemperatureType {
    if temp < 0.15 {
        TemperatureType::Freezing
    } else if temp < 0.3 {
        TemperatureType::Cold
    } else if temp < 0.6 {
        TemperatureType::Temperate
    } else if temp < 0.85 {
        TemperatureType::Warm
    } else {
        TemperatureType::Hot
    }
}

pub fn classify_vegetation(veg: f64, temp: f64, precip: f64) -> VegetationType {
    // Placeholder logic, can be improved
    if veg < 0.1 {
        VegetationType::None
    } else if veg < 0.25 {
        if temp > 0.6 && precip < 0.3 {
            VegetationType::Cacti
        } else {
            VegetationType::Sparse
        }
    } else if veg < 0.4 {
        VegetationType::Grass
    } else if veg < 0.6 {
        VegetationType::Shrubs
    } else if veg < 0.8 {
        VegetationType::Forest
    } else {
        if temp > 0.7 && precip > 0.7 {
            VegetationType::Jungle
        } else {
            VegetationType::Forest
        }
    }
}

pub fn classify_precipitation(precip: f64) -> PrecipitationType {
    if precip < 0.15 {
        PrecipitationType::Arid
    } else if precip < 0.3 {
        PrecipitationType::SemiArid
    } else if precip < 0.6 {
        PrecipitationType::Moderate
    } else if precip < 0.85 {
        PrecipitationType::Wet
    } else {
        PrecipitationType::Rainforest
    }
}

pub fn classify_elevation(elev: f64, sea: f64, coast: f64, mountain: f64) -> ElevationType {
    if elev <= sea {
        ElevationType::Ocean
    } else if elev <= coast {
        ElevationType::Coast
    } else if elev < mountain {
        ElevationType::Lowland
    } else if elev < mountain + 0.03 {
        ElevationType::Hill
    } else if elev < mountain + 0.07 {
        ElevationType::Mountain
    } else {
        ElevationType::Peak
    }
}

/// Classifies whether a tile is a lake based on elevation and moisture.
pub fn classify_lake(elev: f64, sea: f64, coast: f64, moisture: f64) -> bool {
    // Lake: just above sea level, not coast, and high moisture
    elev > sea && elev < coast - 0.01 && moisture > 0.6
}

pub fn classify_biome(
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    elevation: f64,
    moisture: f64,
    ridge: f64,
    temp: f64,
    river_here: bool,
    sea_level: f64,
    coast_level: f64,
    mountain_level: f64,
    precipitation: f64,
    soil_fertility: f64,
    vegetation: f64,
) -> BiomeId {
    if elevation <= sea_level {
        BiomeId::Ocean
    } else if river_here {
        BiomeId::River
    } else if elevation > sea_level && elevation < coast_level - 0.01 && moisture > 0.6 {
        BiomeId::Lake
    } else if elevation <= coast_level {
        BiomeId::Sea
    } else if elevation >= mountain_level && ridge > 0.3 {
        if temp < 0.3 {
            BiomeId::Snow
        } else {
            BiomeId::Mountain
        }
    } else if elevation <= coast_level + 0.02 {
        BiomeId::Beach
    } else if temp < 0.18 && precipitation < 0.3 {
        BiomeId::Tundra
    } else if temp < 0.18 && precipitation >= 0.3 {
        BiomeId::Taiga
    } else if precipitation > 0.8 && temp > 0.7 && vegetation > 0.7 {
        BiomeId::Rainforest
    } else if precipitation > 0.6 && temp > 0.4 && temp < 0.7 && vegetation > 0.5 {
        BiomeId::TemperateForest
    } else if temp < 0.4 && precipitation > 0.4 && vegetation > 0.3 {
        BiomeId::BorealForest
    } else if precipitation > 0.4 && vegetation > 0.5 {
        BiomeId::Forest
    } else if precipitation < 0.2 && temp > 0.6 && vegetation < 0.3 {
        BiomeId::Desert
    } else if precipitation < 0.3 && temp > 0.5 && vegetation > 0.3 {
        BiomeId::Savanna
    } else if precipitation > 0.6 && soil_fertility > 0.5 && vegetation > 0.5 {
        BiomeId::Swamp
    } else if soil_fertility > 0.6 && vegetation > 0.4 {
        BiomeId::Plains
    } else if elevation > coast_level && elevation < mountain_level && ridge > 0.2 {
        BiomeId::Hills
    } else {
        BiomeId::Plains
    }
}

// Classify all biomes for the world map
pub fn all(
    _builder: &crate::worldgen::worldmap::builder::WorldMapBuilder,
    elevation: &Vec<Vec<f64>>,
    moisture: &Vec<Vec<f64>>,
    river_mask: &Vec<Vec<bool>>,
    temperature: &Vec<Vec<f64>>,
    precipitation: &Vec<Vec<f64>>,
    soil_fertility: &Vec<Vec<f64>>,
    vegetation: &Vec<Vec<f64>>,
    ridge: &Vec<Vec<f64>>,
    sea_level: f64,
    coast_level: f64,
    mountain_level: f64,
) -> Vec<Vec<BiomeId>> {
    let width = elevation.len();
    let height = if width > 0 { elevation[0].len() } else { 0 };
    let mut biomes = vec![vec![BiomeId::Plains; height]; width];
    for x in 0..width {
        for y in 0..height {
            let river_here = river_mask[x][y];
            biomes[x][y] = classify_biome(
                x, y, width, height,
                elevation[x][y],
                moisture[x][y],
                ridge[x][y],
                temperature[x][y],
                river_here,
                sea_level,
                coast_level,
                mountain_level,
                precipitation[x][y],
                soil_fertility[x][y],
                vegetation[x][y],
            );
        }
    }
    biomes
} 