use super::thresholds::*;
use super::models::TileEnv;
use crate::world::worldmap::biome::{TemperatureType, VegetationType,
                                    PrecipitationType, ElevationType, BiomeId};

pub fn temperature(t: f64) -> TemperatureType {
    match t {
        x if x < TEMP_FREEZING => TemperatureType::Freezing,
        x if x < TEMP_COLD     => TemperatureType::Cold,
        x if x < TEMP_TEMPERATE=> TemperatureType::Temperate,
        x if x < TEMP_WARM     => TemperatureType::Warm,
        _                      => TemperatureType::Hot,
    }
}

pub fn vegetation(v: f64, temp: f64, precip: f64) -> VegetationType {
    match v {
        x if x < VEG_NONE  => VegetationType::None,
        x if x < VEG_SPARSE=> {
            if temp > TEMP_TEMPERATE && precip < PRECIP_SEMI {
                VegetationType::Cacti
            } else {
                VegetationType::Sparse
            }
        }
        x if x < VEG_GRASS => VegetationType::Grass,
        x if x < VEG_SHRUBS=> VegetationType::Shrubs,
        x if x < VEG_FOREST=> VegetationType::Forest,
        _ => {
            if temp > TEMP_WARM && precip > PRECIP_WET {
                VegetationType::Jungle
            } else {
                VegetationType::Forest
            }
        }
    }
}

pub fn precipitation(p: f64) -> PrecipitationType {
    match p {
        x if x < PRECIP_ARID => PrecipitationType::Arid,
        x if x < PRECIP_SEMI => PrecipitationType::SemiArid,
        x if x < PRECIP_MOD  => PrecipitationType::Moderate,
        x if x < PRECIP_WET  => PrecipitationType::Wet,
        _                    => PrecipitationType::Rainforest,
    }
}

pub fn elevation(e: f64, env: &TileEnv) -> ElevationType {
    if e <= env.sea                      { ElevationType::Ocean    }
    else if e <= env.coast               { ElevationType::Coast    }
    else if e <  env.mountain            { ElevationType::Lowland  }
    else if e <  env.mountain + HILL_OFFSET { ElevationType::Hill   }
    else if e <  env.mountain + PEAK_OFFSET { ElevationType::Mountain}
    else                                 { ElevationType::Peak     }
}

/// Lake decision kept separate for clarity.
pub fn is_lake(env: &TileEnv) -> bool {
    env.lake_here
}

/// The big biome matcher. Exactly same decision tree, just tidied.
pub fn biome(env: &TileEnv) -> BiomeId {
    if env.elev <= env.sea { return BiomeId::Ocean; }
    if env.river_here      { return BiomeId::River; }
    if is_lake(env)        { return BiomeId::Lake;  }
    if env.elev <= env.coast { return BiomeId::Sea; }

    if env.elev >= env.mountain && env.ridge > RIDGE_MOUNTAIN {
        return if env.temp < TEMP_COLD { BiomeId::Snow } else { BiomeId::Mountain };
    }

    if env.elev <= env.coast + 0.02 {
        // Custom beach logic for cold climates
        if env.temp < 0.18 {
            return BiomeId::Tundra;
        } else if env.temp < 0.3 {
            return BiomeId::Taiga;
        } else {
            return BiomeId::Beach;
        }
    }

    match (env.temp, env.precip, env.veg, env.soil) {
        (t, p, _, _) if t < 0.18 && p < 0.3   => BiomeId::Tundra,
        (t, p, _, _) if t < 0.3               => BiomeId::Taiga,
        (t, p, v, _) if p > 0.8 && t > 0.7 && v > 0.7       => BiomeId::Rainforest,
        (t, p, v, _) if p > 0.6 && (0.4..0.7).contains(&t) && v > 0.5
                                                   => BiomeId::TemperateForest,
        (t, p, v, _) if t < 0.4 && p > 0.4 && v > 0.3        => BiomeId::BorealForest,
        (_, p, v, _) if p > 0.4 && v > 0.5                   => BiomeId::Forest,
        (t, p, v, _) if p < 0.2 && t > 0.6 && v < 0.3        => BiomeId::Desert,
        (t, p, v, _) if p < 0.3 && t > 0.5 && v > 0.3        => BiomeId::Savanna,
        (_, p, v, s) if p > 0.6 && s > 0.5 && v > 0.5        => BiomeId::Swamp,
        (_, _, v, s) if s > 0.6 && v > 0.4                   => BiomeId::Plains,
        (_, _, _, _) if env.elev < env.mountain && env.ridge > RIDGE_HILLS
                                                   => BiomeId::Hills,
        _                                                  => BiomeId::Plains,
    }
} 