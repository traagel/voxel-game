use noise::Perlin;
use noise::NoiseFn;

pub fn make(seed: u32, width: usize, height: usize, scale: f64) -> Vec<Vec<f64>> {
    let perlin = Perlin::new(seed.wrapping_add(10));
    let mut precipitation = vec![vec![0.0; height]; width];
    for x in 0..width {
        for y in 0..height {
            let nx = x as f64 / width as f64 - 0.5;
            let ny = y as f64 / height as f64 - 0.5;
            let p = perlin.get([nx * scale, ny * scale]);
            precipitation[x][y] = ((p + 1.0) / 2.0).clamp(0.0, 1.0); // Normalize to [0,1]
        }
    }
    precipitation
}
