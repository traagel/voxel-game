use noise::{Perlin, NoiseFn};

/// Handy wrapper so we’re not sprinkling `Perlin::new(...)` everywhere.
pub struct NoiseSources {
    pub continent: Perlin,
    pub detail:    Perlin,
    pub moist:     Perlin,
    pub ridge:     Perlin,
    pub plateau:   Perlin,
    pub crater:    Perlin,
    pub lake:      Perlin,
}

impl NoiseSources {
    pub fn new(seed: u32) -> Self {
        Self {
            continent: Perlin::new(seed),
            detail:    Perlin::new(seed.wrapping_add(1)),
            moist:     Perlin::new(seed.wrapping_add(2)),
            ridge:     Perlin::new(seed.wrapping_add(3)),
            plateau:   Perlin::new(seed.wrapping_add(100)),
            crater:    Perlin::new(seed.wrapping_add(200)),
            lake:      Perlin::new(seed.wrapping_add(300)),
        }
    }
}
