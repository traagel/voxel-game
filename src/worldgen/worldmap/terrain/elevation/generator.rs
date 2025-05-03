use crate::worldgen::{
    worldmap::utils::noise::fractal_noise,
    worldmap::params::WorldGenParams,
};
use super::{constants as c, noise_sources::NoiseSources, craters::{random_craters, crater_effect}};
use rand::Rng;

pub struct Generated {
    pub elevation: Vec<Vec<f64>>,
    pub moisture:  Vec<Vec<f64>>,
}

pub fn generate(
    params: &WorldGenParams,
    width: usize,
    height: usize,
    scale: f64,
    seed: u32,
    continent_centers: &[(f64,f64)],
    continent_radius: f64,
) -> Generated {

    let noise = NoiseSources::new(seed);
    let craters = random_craters(seed, width, height, c::NUM_CRATERS);

    // --- Valley orientation: random direction per world ---
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed as u64 + 12345);
    let valley_angle = rng.gen_range(0.0..std::f64::consts::TAU); // 0..2π
    let (valley_dx, valley_dy) = (valley_angle.cos(), valley_angle.sin());
    let valley_scale = 2.0; // Controls valley width/frequency

    let mut elevation = vec![vec![0.0; height]; width];
    let mut moisture  = vec![vec![0.0; height]; width];

    for x in 0..width {
        for y in 0..height {
            let (nx, ny) = (
                x as f64 / width  as f64 - 0.5,
                y as f64 / height as f64 - 0.5
            );

            // --- CONTINENTAL MASK + FALLOFF ----------------------------------
            let cont_noise = fractal_noise(
                &noise.continent,
                nx * scale * params.continent_scale * c::CONTINENT_SCALE_FACTOR,
                ny * scale * params.continent_scale * c::CONTINENT_SCALE_FACTOR,
                params.octaves_continent,
                params.persistence,
            );
            let continental_mask = (cont_noise - 0.2).clamp(0.0, 1.0);

            let min_dist = continent_centers.iter()
                .map(|(cx,cy)| {
                    let dx = x as f64 - cx;
                    let dy = y as f64 - cy;
                    ((dx*dx + dy*dy).sqrt()) / continent_radius
                })
                .fold(1.0, f64::min);

            let jagged = fractal_noise(&noise.detail,
                nx * c::DETAIL_JAGGED_FREQ, ny * c::DETAIL_JAGGED_FREQ, 2, 0.5);

            let falloff_noise = noise.detail.get([nx * c::DETAIL_FALLOFF_FREQ, ny * c::DETAIL_FALLOFF_FREQ]);
            let continent_falloff = ((1.0 - min_dist) + 0.3 * jagged + 0.15 * falloff_noise).clamp(0.0, 1.0);

            // --- MID / SMALL‑SCALE DETAIL ------------------------------------
            let detail = fractal_noise(
                &noise.detail,
                nx * scale * params.detail_scale,
                ny * scale * params.detail_scale,
                params.octaves_detail,
                params.persistence,
            );

            // --- RIDGES, PLATEAUS, CRATERS (NO LAKES HERE) -------------------
            let ridge = (1.0 - noise.ridge.get([nx * scale * c::RIDGE_FREQ, ny * scale * c::RIDGE_FREQ]).abs()).powi(3) * 0.7;

            let plateau = (noise.plateau.get([nx * scale * c::PLATEAU_FREQ, ny * scale * c::PLATEAU_FREQ]) * 0.5 + 0.5).powf(2.0) * 0.18;

            let crater = crater_effect(x, y, &craters);

            // --- VALLEY MASK (random direction) ------------------------------
            let valley_nx = nx * valley_dx * valley_scale + ny * valley_dy * valley_scale;
            let valley_ny = -nx * valley_dy * 0.5 + ny * valley_dx * 0.5; // Perpendicular for variation
            let valley_noise = noise.ridge.get([valley_nx, valley_ny]);
            let valley_mask = (1.0 - valley_noise.abs()).powi(2) * 0.25; // Tune power/weight as needed

            // --- COMBINE ------------------------------------------------------
            let mut e =
                  continental_mask * c::WEIGHT_CONTINENTAL_MASK
                + detail           * c::WEIGHT_DETAIL
                + ridge            * c::WEIGHT_RIDGE
                + continent_falloff* c::WEIGHT_FALLOFF
                + plateau          * c::WEIGHT_PLATEAU
                + crater
                + c::BASELINE_SHIFT;

            // Subtract valley mask to carve valleys
            e -= valley_mask;

            // [Lakes are now handled by hydrology::lakes::apply_lakes]

            e  = e.clamp(0.0, 1.0);

            elevation[x][y] = e;
            moisture[x][y]  = noise.moist.get([nx * scale, ny * scale]);
        }
    }

    // Optional: dump quick stats
    if cfg!(debug_assertions) {
        let flat: Vec<f64> = elevation.iter().flatten().copied().collect();
        let (min,max,mean) = (
            flat.iter().cloned().fold(f64::INFINITY, f64::min),
            flat.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
            flat.iter().sum::<f64>() / flat.len() as f64
        );
        println!("Elevation stats: min={min:.3}, max={max:.3}, mean={mean:.3}");
    }

    Generated { elevation, moisture }
}
