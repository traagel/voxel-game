use noise::{NoiseFn, Perlin};

/// Fractional Brownian motion 2‑D.
pub fn fractal_noise(
    perlin: &Perlin,
    x: f64,
    y: f64,
    octaves: usize,
    persistence: f64,
) -> f64 {
    let (mut total, mut freq, mut amp, mut max) = (0.0, 1.0, 1.0, 0.0);
    for _ in 0..octaves {
        total += perlin.get([x * freq, y * freq]) * amp;
        max += amp;
        amp *= persistence;
        freq *= 2.0;
    }
    total / max
}

/// Return the value below which `p` fraction of the slice lies.
pub fn percentile(values: &mut [f64], p: f64) -> f64 {
    values.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let idx = ((values.len() as f64) * p).floor() as usize;
    values[idx]
}
