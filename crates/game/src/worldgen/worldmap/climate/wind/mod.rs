use noise::Perlin;
use noise::NoiseFn;

pub fn make(seed: u32, width: usize, height: usize, scale: f64) -> Vec<Vec<(f64, f64)>> {
    let perlin = Perlin::new(seed.wrapping_add(20));
    let mut wind = vec![vec![(1.0, 0.0); height]; width];
    for x in 0..width {
        for y in 0..height {
            let nx = x as f64 / width as f64 - 0.5;
            let ny = y as f64 / height as f64 - 0.5;
            let angle = perlin.get([nx * scale, ny * scale]) * std::f64::consts::PI;
            wind[x][y] = (angle.cos(), angle.sin());
        }
    }
    wind
}
