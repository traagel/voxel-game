pub mod elevation;
pub mod mountains;
pub mod continents;

use noise::{Perlin, NoiseFn};

/// Generate a ridge map using Perlin noise, values in [0, 1]
pub fn ridge_map(seed: u32, width: usize, height: usize, scale: f64) -> Vec<Vec<f64>> {
    let perlin = Perlin::new(seed.wrapping_add(42));
    (0..width)
        .map(|x| {
            (0..height)
                .map(|y| {
                    let nx = x as f64 / width as f64 - 0.5;
                    let ny = y as f64 / height as f64 - 0.5;
                    // Ridge value in [0, 1]
                    (1.0 - perlin.get([nx * scale * 2.0, ny * scale * 2.0]).abs()).powi(3)
                })
                .collect()
        })
        .collect()
}
