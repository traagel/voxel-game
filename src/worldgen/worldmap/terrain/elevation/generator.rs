//! worldgen::worldmap::generate
//! quick‑n‑dirty refactor => fewer side‑effects, narrower helpers, easier to read/bench/test.

use crate::worldgen::{
    worldmap::utils::noise::fractal_noise,
    worldmap::params::WorldGenParams,
};
use super::{
    constants as c,
    noise_sources::NoiseSources,
    craters::{random_craters, crater_effect},
    ridge::ridge_map,
};
use noise::{NoiseFn, Perlin};
use rand::{Rng, SeedableRng};

/// Stores the generated elevation and moisture maps.
pub struct Generated {
    pub elevation: Vec<Vec<f64>>,
    pub moisture:  Vec<Vec<f64>>,
}

/// Generates a world map with elevation and moisture values.
pub fn generate(
    params: &WorldGenParams,
    width: usize,
    height: usize,
    scale: f64,
    seed: u32,
    _continent_centers: &[(f64, f64)],
    _continent_radius: f64,
) -> Generated {
    println!("Starting world generation");

    let noise = create_noise_sources(seed);
    let craters = create_craters(seed, width, height);
    let ridge_weight = params.ridge_weight;
    let ridge_map = ridge_map(seed, width, height, scale);

    let mut elevation = vec![vec![0.0; height]; width];
    let mut moisture  = vec![vec![0.0; height]; width];

    for x in 0..width {
        for y in 0..height {
            let normalized = normalize_coords(x, y, width, height);
            let continent = continent_mask(params, &noise, normalized, scale);
            let detail = detail_noise(params, &noise, normalized, scale);
            let ridge = mountain_ridge(&noise, normalized, scale);
            let plateau = plateau_noise(&noise, normalized, scale);
            let lake = lake_noise(&noise, normalized, scale);
            let crater = crater_at(&craters, x as f64, y as f64);

            let elevation_value = combine_elevation(
                continent,
                detail,
                ridge,
                plateau,
                lake,
                crater,
            );
            elevation[x][y] = elevation_value * (1.0 - ridge_weight) + ridge_map[x][y] * ridge_weight;
            moisture[x][y]  = noise.moist.get([normalized.0 * scale, normalized.1 * scale]);
        }
    }

    print_elevation_stats(&elevation);

    Generated { elevation, moisture }
}

struct NoiseSet {
    continent: Perlin,
    detail: Perlin,
    moist: Perlin,
    ridge: Perlin,
    plateau: Perlin,
    lake: Perlin,
}

fn create_noise_sources(seed: u32) -> NoiseSet {
    NoiseSet {
        continent: Perlin::new(seed),
        detail: Perlin::new(seed.wrapping_add(1)),
        moist: Perlin::new(seed.wrapping_add(2)),
        ridge: Perlin::new(seed.wrapping_add(3)),
        plateau: Perlin::new(seed.wrapping_add(100)),
        lake: Perlin::new(seed.wrapping_add(300)),
    }
}

fn create_craters(seed: u32, width: usize, height: usize) -> Vec<(f64, f64, f64)> {
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed as u64 + 999);
    (0..5).map(|_| {
        (
            rng.gen_range(0.1..0.9) * width as f64,
            rng.gen_range(0.1..0.9) * height as f64,
            rng.gen_range(8.0..24.0),
        )
    }).collect()
}

#[inline]
fn normalize_coords(x: usize, y: usize, width: usize, height: usize) -> (f64, f64) {
    (x as f64 / width as f64 - 0.5, y as f64 / height as f64 - 0.5)
}

fn continent_mask(params: &WorldGenParams, noise: &NoiseSet, (nx, ny): (f64, f64), scale: f64) -> f64 {
    let value = fractal_noise(&noise.continent,
        nx * scale * params.continent_scale * 1.5,
        ny * scale * params.continent_scale * 1.5,
        params.octaves_continent,
        params.persistence,
    );
    (value - 0.2).clamp(0.0, 1.0)
}

fn detail_noise(params: &WorldGenParams, noise: &NoiseSet, (nx, ny): (f64, f64), scale: f64) -> f64 {
    fractal_noise(&noise.detail,
        nx * scale * params.detail_scale,
        ny * scale * params.detail_scale,
        params.octaves_detail,
        params.persistence,
    ) * 0.15
}

fn mountain_ridge(noise: &NoiseSet, (nx, ny): (f64, f64), scale: f64) -> f64 {
    let ridge = 1.0 - noise.ridge.get([nx * scale * 2.0, ny * scale * 2.0]).abs();
    (ridge.powi(3)) * 0.7
}

fn plateau_noise(noise: &NoiseSet, (nx, ny): (f64, f64), scale: f64) -> f64 {
    let plateau = (noise.plateau.get([nx * scale * 0.7, ny * scale * 0.7]) * 0.5 + 0.5).powf(2.0);
    plateau * 0.18
}

fn lake_noise(noise: &NoiseSet, (nx, ny): (f64, f64), scale: f64) -> f64 {
    let lake = (noise.lake.get([nx * scale * 0.8, ny * scale * 0.8]) * 0.5 + 0.5).powf(2.0);
    (1.0 - lake).powf(2.0)
}

fn crater_at(craters: &[(f64, f64, f64)], x: f64, y: f64) -> f64 {
    craters.iter().fold(0.0, |acc, &(cx, cy, r)| {
        let dist = ((x - cx).powi(2) + (y - cy).powi(2)).sqrt();
        if dist < r {
            let norm = 1.0 - dist / r;
            acc - norm.powf(1.5) * 0.25
        } else { acc }
    })
}

#[inline]
fn combine_elevation(
    continent: f64,
    detail: f64,
    ridge: f64,
    plateau: f64,
    lake: f64,
    crater: f64,
) -> f64 {
    let mut elevation = continent * 0.5
        + detail
        + ridge
        + plateau
        + crater
        - 0.15;

    elevation -= lake * 0.18;
    elevation = elevation * (1.0 - 0.25 * lake) + lake * 0.15;

    elevation.clamp(0.0, 1.0)
}

fn print_elevation_stats(elevation: &[Vec<f64>]) {
    let (mut min, mut max, mut sum, mut count) = (1.0, 0.0, 0.0, 0.0);
    for col in elevation {
        for &e in col {
            if e < min { min = e }
            if e > max { max = e }
            sum += e; count += 1.0;
        }
    }
    println!("Elevation stats: min={:.3}, max={:.3}, mean={:.3}",
             min, max, sum / count);
}
