use noise::NoiseFn;
use noise::Perlin;

/// Generate a ridge map using Perlin noise, values in [0, 1].
/// This version uses fractal ridged noise, coordinate warping, and sharpening to make ridges less blob-like.
pub fn ridge_map(seed: u32, width: usize, height: usize, scale: f64) -> Vec<Vec<f64>> {
    let perlin = Perlin::new(seed.wrapping_add(42));
    let warp = Perlin::new(seed.wrapping_add(99));
    let detail = Perlin::new(seed.wrapping_add(123));
    let ridge_freq = scale * 6.0; // Even higher frequency
    let warp_freq = scale * 2.5;
    let detail_freq = scale * 16.0;
    let octaves = 3;
    let persistence = 0.5;
    let detail_weight = 0.18;
    (0..width)
        .map(|x| {
            (0..height)
                .map(|y| {
                    let nx = x as f64 / width as f64 - 0.5;
                    let ny = y as f64 / height as f64 - 0.5;
                    // Coordinate warping
                    let wx = nx + 0.15 * warp.get([nx * warp_freq, ny * warp_freq]);
                    let wy = ny + 0.15 * warp.get([ny * warp_freq, nx * warp_freq]);
                    // Fractal ridged noise
                    let mut amplitude = 1.0;
                    let mut frequency = 1.0;
                    let mut value = 0.0;
                    let mut max = 0.0;
                    for _ in 0..octaves {
                        let n = 1.0 - perlin.get([wx * ridge_freq * frequency, wy * ridge_freq * frequency]).abs();
                        value += n.powi(3) * amplitude;
                        max += amplitude;
                        amplitude *= persistence;
                        frequency *= 2.0;
                    }
                    let ridge = (value / max).clamp(0.0, 1.0);
                    // Add high-frequency detail
                    let d = (1.0 - detail.get([nx * detail_freq, ny * detail_freq]).abs()).powi(2);
                    // Sharpen the ridge mask for thinner, more chain-like ridges
                    let sharpened = ridge.powf(2.5);
                    (sharpened * (1.0 - detail_weight) + d * detail_weight).clamp(0.0, 1.0)
                })
                .collect()
        })
        .collect()
} 