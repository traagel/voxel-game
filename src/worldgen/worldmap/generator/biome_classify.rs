// Biome classification logic
use crate::world::worldmap::biome::BiomeId;
use noise::Perlin;

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