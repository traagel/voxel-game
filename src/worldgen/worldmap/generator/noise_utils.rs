// Noise and terrain utility functions for world generation
use noise::Perlin;
use noise::NoiseFn;

/// Classic fractional Brownian motion helper.
pub fn fractal_noise(
    perlin: &Perlin,
    x: f64,
    y: f64,
    octaves: usize,
    persistence: f64,
) -> f64 {
    let mut total = 0.0;
    let mut frequency = 1.0;
    let mut amplitude = 1.0;
    let mut max_value = 0.0;
    for _ in 0..octaves {
        total += perlin.get([x * frequency, y * frequency]) * amplitude;
        max_value += amplitude;
        amplitude *= persistence;
        frequency *= 2.0;
    }
    total / max_value
}

/// Returns the value below which `percent` of the sorted slice lies.
pub fn percentile(values: &mut [f64], p: f64) -> f64 {
    values.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let idx = ((values.len() as f64) * p).floor() as usize;
    values[idx]
}

/// One step of extremely cheap thermal erosion.
pub fn erosion_pass(elev: &mut [Vec<f64>]) {
    let w = elev.len();
    let h = elev[0].len();
    let mut coords: Vec<(usize, usize)> =
        (0..w).flat_map(|x| (0..h).map(move |y| (x, y))).collect();
    use rand::seq::SliceRandom;
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

pub fn lowest_neighbor(elev: &[Vec<f64>], x: usize, y: usize) -> (usize, usize) {
    let w = elev.len();
    let h = elev[0].len();
    let mut min = elev[x][y];
    let mut best = (x, y);
    for dx in [-1i32, 0, 1] {
        for dy in [-1i32, 0, 1] {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = x.wrapping_add(dx as usize);
            let ny = y.wrapping_add(dy as usize);
            if nx < w && ny < h {
                if elev[nx][ny] < min {
                    min = elev[nx][ny];
                    best = (nx, ny);
                }
            }
        }
    }
    best
}

/// Walk downhill accumulating flow; simple river carving.
pub fn accumulate_flow(
    elev: &[Vec<f64>],
    flow: &mut [Vec<f64>],
    x: usize,
    y: usize,
    sea_level: f64,
) {
    let mut cx = x;
    let mut cy = y;
    loop {
        flow[cx][cy] += 1.0;
        if elev[cx][cy] <= sea_level {
            break;
        }
        let (nx, ny) = lowest_neighbor(elev, cx, cy);
        if (nx, ny) == (cx, cy) {
            break;
        }
        cx = nx;
        cy = ny;
    }
} 