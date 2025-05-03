pub mod constants;
pub mod craters;
pub mod noise_sources;
pub mod generator;
pub mod ridge;

pub use ridge::ridge_map;

use crate::worldgen::worldmap::params::WorldGenParams;

pub fn generate(
    params: &WorldGenParams,
    width: usize,
    height: usize,
    scale: f64,
    seed: u32,
    continent_centers: &Vec<(f64, f64)>,
    continent_radius: f64,
) -> (Vec<Vec<f64>>, Vec<Vec<f64>>)
{
    let result = generator::generate(
        params,
        width,
        height,
        scale,
        seed,
        continent_centers,
        continent_radius,
    );
    (result.elevation, result.moisture)
}
