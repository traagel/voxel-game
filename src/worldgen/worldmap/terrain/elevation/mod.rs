use noise::Perlin;
use noise::NoiseFn;
use rand::Rng;
use rand::SeedableRng;
use crate::worldgen::worldmap::utils::noise::fractal_noise;
use crate::worldgen::worldmap::params::WorldGenParams;

pub fn generate(
    params: &WorldGenParams,
    width: usize,
    height: usize,
    scale: f64,
    seed: u32,
    continent_centers: &Vec<(f64, f64)>,
    continent_radius: f64,
) -> (Vec<Vec<f64>>, Vec<Vec<f64>>)
{
    let p = params;
    let mut elevation = vec![vec![0.0; height]; width];
    let mut moisture = vec![vec![0.0; height]; width];
    // New: Plateau and crater noise
    let perlin_continent = Perlin::new(seed);
    let perlin_detail = Perlin::new(seed.wrapping_add(1));
    let perlin_moist = Perlin::new(seed.wrapping_add(2));
    let perlin_ridge = Perlin::new(seed.wrapping_add(3));
    let perlin_plateau = Perlin::new(seed.wrapping_add(100));
    let perlin_crater = Perlin::new(seed.wrapping_add(200));
    let perlin_lake = Perlin::new(seed.wrapping_add(300)); // New: lake/plains noise
    // Generate random craters
    let mut craters = Vec::new();
    let num_craters = 5;
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed as u64 + 999);
    for _ in 0..num_craters {
        let cx = rng.gen_range(0.1..0.9) * width as f64;
        let cy = rng.gen_range(0.1..0.9) * height as f64;
        let r = rng.gen_range(8.0..24.0);
        craters.push((cx, cy, r));
    }
    for x in 0..width {
        for y in 0..height {
            let nx = x as f64 / width as f64 - 0.5;
            let ny = y as f64 / height as f64 - 0.5;
            let c = fractal_noise(
                &perlin_continent,
                nx * scale * p.continent_scale * 1.5,
                ny * scale * p.continent_scale * 1.5,
                p.octaves_continent,
                p.persistence,
            );
            let continental_mask = (c - 0.2).clamp(0.0, 1.0);
            let mut min_dist = 1.0;
            for &(cx, cy) in continent_centers {
                let dx = x as f64 - cx;
                let dy = y as f64 - cy;
                let dist = ((dx * dx + dy * dy).sqrt()) / continent_radius;
                if dist < min_dist {
                    min_dist = dist;
                }
            }
            let jagged = fractal_noise(&perlin_detail, nx * 2.0, ny * 2.0, 2, 0.5);
            // Modulate continent falloff with extra noise for more jagged edges
            let falloff_noise = perlin_detail.get([nx * 3.0, ny * 3.0]);
            let continent_falloff = ((1.0 - min_dist) + 0.3 * jagged + 0.15 * falloff_noise).clamp(0.0, 1.0);
            let d = fractal_noise(
                &perlin_detail,
                nx * scale * p.detail_scale,
                ny * scale * p.detail_scale,
                p.octaves_detail,
                p.persistence,
            );
            // Plateau noise for mesas and highlands
            let plateau = (perlin_plateau.get([nx * scale * 0.7, ny * scale * 0.7]) * 0.5 + 0.5).powf(2.0) * 0.18;
            // Ridge noise for mountains (stronger influence)
            let r = 1.0 - perlin_ridge.get([nx * scale * 2.0, ny * scale * 2.0]).abs();
            let ridge = (r.powi(3)) * 0.7;
            // Crater effect: subtract elevation in crater centers
            let mut crater_effect = 0.0;
            for &(cx, cy, rad) in &craters {
                let dx = x as f64 - cx;
                let dy = y as f64 - cy;
                let dist = (dx * dx + dy * dy).sqrt();
                if dist < rad {
                    let norm: f64 = 1.0 - (dist / rad);
                    crater_effect -= norm.powf(1.5) * 0.25;
                }
            }
            // Lake/plains noise: create flat, low-lying areas
            let lake_noise = (perlin_lake.get([nx * scale * 0.8, ny * scale * 0.8]) * 0.5 + 0.5).powf(2.0);
            let lake_mask = (1.0 - lake_noise).powf(2.0); // Stronger effect in low areas
            // Lower the baseline and reduce some weights
            let mut e = continental_mask * 0.5 + d * 0.15 + ridge + continent_falloff * 0.4 + plateau + crater_effect - 0.15;
            // Blend in lake/plains effect (subtract elevation, flatten)
            e -= lake_mask * 0.18; // Lower elevation in lake areas
            e = e * (1.0 - 0.25 * lake_mask) + lake_mask * 0.15; // Flatten in lake areas
            e = e.clamp(0.0, 1.0);
            elevation[x][y] = e;
            let m = perlin_moist.get([nx * scale, ny * scale]);
            moisture[x][y] = m;
        }
    }
    // Print elevation stats
    let mut min_elev = 1.0;
    let mut max_elev = 0.0;
    let mut sum_elev = 0.0;
    let mut count = 0.0;
    for x in 0..width {
        for y in 0..height {
            let e = elevation[x][y];
            if e < min_elev { min_elev = e; }
            if e > max_elev { max_elev = e; }
            sum_elev += e;
            count += 1.0;
        }
    }
    println!("Elevation stats: min={:.3}, max={:.3}, mean={:.3}", min_elev, max_elev, sum_elev / count);
    (elevation, moisture)
}
