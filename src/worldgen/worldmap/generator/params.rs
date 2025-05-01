// WorldGenParams struct and Default implementation

#[derive(Clone, Copy)]
pub struct WorldGenParams {
    pub ocean_percent: f64,        // portion of tiles that should be water
    pub coast_percent: f64,        // thin band above sea level
    pub mountain_percent: f64,     // top X % counted as mountains
    pub erosion_iterations: usize, // cheap thermal erosion passes
    pub river_threshold: f64,      // min flow to be called a river
    pub continent_scale: f64,      // low‑freq scale (big shapes)
    pub detail_scale: f64,         // hi‑freq scale (little bumps)
    pub octaves_continent: usize,
    pub octaves_detail: usize,
    pub persistence: f64, // FBM persistence for both noises
    pub num_continents: usize, // number of major continents
}

impl Default for WorldGenParams {
    fn default() -> Self {
        Self {
            ocean_percent: 0.35,
            coast_percent: 0.10,
            mountain_percent: 0.10,
            erosion_iterations: 30,
            river_threshold: 120.0,
            continent_scale: 0.25,
            detail_scale: 16.0,
            octaves_continent: 6,
            octaves_detail: 8,
            persistence: 1.2,
            num_continents: 3,
        }
    }
} 