use super::super::utils::erosion::lowest_neighbor;

/// Accumulate downslope flow counts – basis for rivers.
pub fn accumulate_flow(
    elev: &[Vec<f64>],
    flow: &mut [Vec<f64>],
    x: usize,
    y: usize,
    sea_level: f64,
) {
    let (mut cx, mut cy) = (x, y);
    loop {
        flow[cx][cy] += 1.0;
        if elev[cx][cy] <= sea_level { break; }
        let (nx, ny) = lowest_neighbor(elev, cx, cy);
        if (nx, ny) == (cx, cy) { break; }
        cx = nx; cy = ny;
    }
}
