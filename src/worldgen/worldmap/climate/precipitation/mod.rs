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
