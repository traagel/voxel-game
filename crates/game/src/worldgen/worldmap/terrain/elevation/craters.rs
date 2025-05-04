use rand::{Rng, SeedableRng};
use crate::worldgen::worldmap::terrain::elevation::constants::CRATER_DEPTH;

#[derive(Copy, Clone)]
pub struct Crater {
    pub x: f64,
    pub y: f64,
    pub rx: f64,
    pub ry: f64,
    pub angle: f64,
}

/// Generate craters with random position, elliptical radii, and orientation
pub fn random_craters(seed: u32, width: usize, height: usize, how_many: usize) -> Vec<Crater> {
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed as u64 + 999);
    (0..how_many)
        .map(|_| {
            let base_r = rng.gen_range(8.0..24.0);
            // Random stretch factor between 0.5x and 1.5x
            let stretch = rng.gen_range(0.5..1.5);
            let rx = base_r * stretch;
            let ry = base_r / stretch;
            let angle = rng.gen_range(0.0..std::f64::consts::TAU);
            Crater {
                x: rng.gen_range(0.1..0.9) * width as f64,
                y: rng.gen_range(0.1..0.9) * height as f64,
                rx,
                ry,
                angle,
            }
        })
        .collect()
}

/// Compute crater effect: applies elliptical attenuation based on rotated distance
pub fn crater_effect(cx: usize, cy: usize, craters: &[Crater]) -> f64 {
    let mut effect = 0.0;
    for c in craters {
        let dx = cx as f64 - c.x;
        let dy = cy as f64 - c.y;
        // Rotate point into crater's local coordinate frame
        let cos_a = c.angle.cos();
        let sin_a = c.angle.sin();
        let x_rot = dx * cos_a + dy * sin_a;
        let y_rot = -dx * sin_a + dy * cos_a;
        // Compute normalized distance squared in ellipse space
        let dist_norm_sq = (x_rot / c.rx).powi(2) + (y_rot / c.ry).powi(2);
        if dist_norm_sq < 1.0 {
            // distance from rim [0..1]
            let dist_norm = dist_norm_sq.sqrt();
            let depth_factor = 1.0 - dist_norm;
            effect -= depth_factor.powf(1.5) * CRATER_DEPTH;
        }
    }
    effect
}
