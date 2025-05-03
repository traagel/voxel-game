#[derive(Clone, Copy)]
pub struct WorldGenParams {
    /// Fraction of tiles that should be ocean (0–1).
    pub ocean_percent: f64,
    /// Extra percentile band that becomes coast/beach.
    pub coast_percent: f64,
    /// Top percentile that becomes mountains.
    pub mountain_percent: f64,
    /// Cheap thermal‑erosion passes.
    pub erosion_iterations: usize,
    /// Minimum accumulated flow to mark a river.
    pub river_threshold: f64,
    /// Low‑frequency scale for continents.
    pub continent_scale: f64,
    /// High‑frequency scale for detail.
    pub detail_scale: f64,
    pub octaves_continent: usize,
    pub octaves_detail: usize,
    /// fBm persistence for both noise layers.
    pub persistence: f64,
    /// How many major continents to generate.
    pub num_continents: usize,
    /// How many craters to generate for worldgen.
    pub num_craters: usize,
    /// How much the ridge map influences elevation (0.0–1.0).
    pub ridge_weight: f64,
}

impl Default for WorldGenParams {
    fn default() -> Self {
        Self {
            ocean_percent: 0.35,
            coast_percent: 0.10,
            mountain_percent: 0.05,
            erosion_iterations: 30,
            river_threshold: 45.0,
            continent_scale: 0.25,
            detail_scale: 16.0,
            octaves_continent: 6,
            octaves_detail: 8,
            persistence: 1.2,
            num_continents: 3,
            num_craters: 5,
            ridge_weight: 0.18,
        }
    }
}