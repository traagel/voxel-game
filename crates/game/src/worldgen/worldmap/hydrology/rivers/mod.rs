use crate::worldgen::worldmap::builder::WorldMapBuilder;

/// Returns a river mask (true where river exists) based on flow threshold.
pub fn mask(
    builder: &WorldMapBuilder,
    flow: &Vec<Vec<f64>>,
) -> Vec<Vec<bool>> {
    let threshold = builder.params.river_threshold;
    let width = builder.width;
    let height = builder.height;
    let mut rivers = vec![vec![false; height]; width];
    for x in 0..width {
        for y in 0..height {
            if flow[x][y] > threshold {
                rivers[x][y] = true;
            }
        }
    }
    rivers
}
