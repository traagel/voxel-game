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
    env.elev > env.sea && env.elev < env.coast - 0.01 && env.moisture > 0.6
}

/// The big biome matcher. Exactly same decision tree, just tidied.
pub fn biome(env: &TileEnv) -> BiomeId {
    if env.elev <= env.sea { return BiomeId::Ocean; }
    if env.river_here      { return BiomeId::River; }
    if is_lake(env)        { return BiomeId::Lake;  }
    if env.elev <= env.coast { return BiomeId::Sea; }
use noise::{NoiseFn, Perlin};

/// Generate a 2‑D precipitation field covering the full **0‥1** span.
/// Compared with the previous revision this version is biased *wetter* so
/// you get larger areas that satisfy the swamp thresholds.
///
/// Pipeline:
/// 1. 4‑octave Perlin FBM
/// 2. Histogram skew (`powf(0.6)`) – exponent < 1 ⇒ values are pulled **up**
/// 3. Latitude Hadley‑cell multiplier, stronger than before (0.7 + 1.2·belt)
/// 4. Global renormalisation to keep 0‥1 range
///
/// Keep the quadratic elevation dampening you added in `WorldMapBuilder` – it
/// will knock *mountain* precipitation back down yet still leave lowlands wet.
#[inline]
pub fn make(seed: u32, width: usize, height: usize, scale: f64) -> Vec<Vec<f64>> {
    let perlin = Perlin::new(seed.wrapping_add(10));
    let mut precip = vec![vec![0.0; height]; width];
    let base_scale = scale * 5.0;

    // ── 1. FBM ────────────────────────────────────────────────────────────────
    for x in 0..width {
        for y in 0..height {
            let nx = x as f64 / width as f64 - 0.5;
            let ny = y as f64 / height as f64 - 0.5;

            let mut value = 0.0;
            let mut amp = 1.0;
            let mut freq = 1.0;
            let mut norm = 0.0;
            for _ in 0..4 {
                value += perlin.get([nx * base_scale * freq, ny * base_scale * freq]) * amp;
                norm += amp;
                amp *= 0.5;
                freq *= 2.0;
            }
            let mut v = (value / norm + 1.0) * 0.5; // → 0‥1

            // ── 2. histogram skew ────────────────────────────────────────────
            v = v.powf(0.6); // <1 ⇒ biases *up* (wetter)
            precip[x][y] = v;
        }
    }

    // ── 3. Hadley‑cell latitude modifier ─────────────────────────────────────
    for x in 0..width {
        for y in 0..height {
            let lat = y as f64 / height as f64;                       // 0‥1 S→N
            let belt = ((lat - 0.5) * std::f64::consts::PI).sin().abs();
            precip[x][y] *= 0.7 + 1.2 * belt;                         // stronger spread
        }
    }

    // ── 4. global renormalisation ────────────────────────────────────────────
    let (mut min, mut max) = (f64::MAX, f64::MIN);
    for row in &precip {
        for &v in row { min = min.min(v); max = max.max(v); }
    }
    let span = (max - min).max(1e-9);
    for row in &mut precip {
        for v in row { *v = (*v - min) / span; }
    }
    println!("Precipitation min: 0.000, max: 1.000");

    precip
}

    if env.elev >= env.mountain && env.ridge > RIDGE_MOUNTAIN {
        return if env.temp < TEMP_COLD { BiomeId::Snow } else { BiomeId::Mountain };
    }

    if env.elev <= env.coast + 0.01 { return BiomeId::Beach; }

    match (env.temp, env.precip, env.veg, env.soil) {
        (t, p, _, _) if t < 0.18 && p < 0.3   => BiomeId::Tundra,
        (t, p, _, _) if t < 0.18              => BiomeId::Taiga,
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