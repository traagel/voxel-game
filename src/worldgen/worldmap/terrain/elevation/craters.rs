use rand::{Rng, SeedableRng};

#[derive(Copy, Clone)]
pub struct Crater {
    pub x: f64,
    pub y: f64,
    pub r: f64,
}

pub fn random_craters(seed: u32, width: usize, height: usize, how_many: usize) -> Vec<Crater> {
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed as u64 + 999);
    (0..how_many)
        .map(|_| Crater {
            x: rng.gen_range(0.1..0.9) * width as f64,
            y: rng.gen_range(0.1..0.9) * height as f64,
            r: rng.gen_range(8.0..24.0),
        })
        .collect()
}

pub fn crater_effect(cx: usize, cy: usize, craters: &[Crater]) -> f64 {
    use crate::worldgen::worldmap::terrain::elevation::constants::CRATER_DEPTH;
    let mut effect = 0.0;
    for c in craters {
        let dx = cx as f64 - c.x;
        let dy = cy as f64 - c.y;
        let dist = (dx * dx + dy * dy).sqrt();
        if dist < c.r {
            let norm = 1.0 - (dist / c.r);
            effect -= norm.powf(1.5) * CRATER_DEPTH;
        }
    }
    effect
}
