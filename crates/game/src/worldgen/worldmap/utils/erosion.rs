use rand::seq::SliceRandom;

/// One very cheap thermal‑erosion step.
pub fn erosion_pass(elev: &mut [Vec<f64>]) {
    let (w, h) = (elev.len(), elev[0].len());
    let mut coords: Vec<(usize, usize)> =
        (0..w).flat_map(|x| (0..h).map(move |y| (x, y))).collect();
    coords.shuffle(&mut rand::thread_rng());

    for (x, y) in coords {
        let (nx, ny) = lowest_neighbor(elev, x, y);
        if (nx, ny) != (x, y) {
            let diff = (elev[x][y] - elev[nx][ny]) * 0.05;
            elev[x][y] -= diff;
            elev[nx][ny] += diff;
        }
    }
}

/// Find the lowest neighbour of (x,y) including itself.
pub fn lowest_neighbor(elev: &[Vec<f64>], x: usize, y: usize) -> (usize, usize) {
    let (w, h) = (elev.len(), elev[0].len());
    let mut min = elev[x][y];
    let mut best = (x, y);
    for dx in [-1_i32, 0, 1] {
        for dy in [-1_i32, 0, 1] {
            if dx == 0 && dy == 0 { continue; }
            let nx = x.wrapping_add(dx as usize);
            let ny = y.wrapping_add(dy as usize);
            if nx < w && ny < h && elev[nx][ny] < min {
                min = elev[nx][ny];
                best = (nx, ny);
            }
        }
    }
    best
}
