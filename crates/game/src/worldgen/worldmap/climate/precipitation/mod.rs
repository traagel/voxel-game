use noise::{NoiseFn, Perlin};

/// Generate a 2‑D precipitation field in **0‥1** with geography‑aware detail.
/// 
/// Adds two realism boosts over the old version:
/// * **Zonal noise** breaks the pure latitude stripes.
/// * **Orographic modifier** (wind‑ward wet, lee‑ward dry) uses the elevation
///   map, so mountains cast rain‑shadows and coastal lowlands get wetter.
///
/// Steps:
/// 1. 4‑octave Perlin FBM base field.
/// 2. Histogram skew (`powf(0.6)`) ⇒ overall wetter world.
/// 3. Hadley‑cell latitude multiplier *times* low‑frequency E‑W Perlin.
/// 4. Orographic amplification/dampening taken from the height map.
/// 5. Global renormalisation back to 0‥1.
#[inline]
pub fn make(
    seed: u32,
    width: usize,
    height: usize,
    scale: f64,
    elevation: &Vec<Vec<f64>>, // new parameter!
) -> Vec<Vec<f64>> {
    // Base noise fields
    let perlin = Perlin::new(seed.wrapping_add(10));
    let zonal_perlin = Perlin::new(seed.wrapping_add(42));

    let mut precip = vec![vec![0.0; height]; width];
    let base_scale = scale * 5.0;

    // ── 1–3. FBM + skew + latitude/zonal ────────────────────────────────────
    for x in 0..width {
        for y in 0..height {
            let nx = x as f64 / width as f64 - 0.5;
            let ny = y as f64 / height as f64 - 0.5;

            // FBM -------------------------------------------------------------
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

            // Histogram skew (wetter) ----------------------------------------
            v = v.powf(0.6);

            // Hadley belt × zonal noise --------------------------------------
            let lat  = y as f64 / height as f64;          // 0‥1
let hadley = ((lat - 0.5) * std::f64::consts::PI).cos().max(0.0); // 1 at 0°, 0 at 0.25 & 0.75
let zonal  = 0.5 + 0.5 * zonal_perlin.get([nx * 0.3, 123.4]);
v *= (0.8 + 1.0 * hadley) * zonal;   

            precip[x][y] = v;
        }
    }

    // ── 4. Orographic rain‑shadow ───────────────────────────────────────────
    for x in 0..width {
        for y in 0..height {
            let lat = y as f64 / height as f64;
            // crude prevailing wind: trade winds (E→W) in tropics, westerlies (W→E) in temperate
            let wind_dx: isize = if lat < 0.33 || lat > 0.66 { 1 } else { -1 };
            let x_upwind = ((x as isize) - wind_dx).rem_euclid(width as isize) as usize;
            let slope = elevation[x][y] - elevation[x_upwind][y];

            let oro = if slope > 0.0 {
                1.0 + 2.0 * slope      // wind‑ward boost
            } else {
                1.0 + 0.5 * slope      // lee‑ward drying (slope negative)
            };
            precip[x][y] *= oro.max(0.0);
        }
    }

    // ── 5. Global renormalisation ───────────────────────────────────────────
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
