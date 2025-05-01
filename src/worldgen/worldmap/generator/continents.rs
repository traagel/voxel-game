// Continent center generation logic
use rand::SeedableRng;
use rand::Rng;

pub fn generate_continent_centers(seed: u32, width: usize, height: usize, num: usize) -> Vec<(f64, f64)> {
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed as u64);
    let mut centers = Vec::with_capacity(num);
    let min_dist = (width.min(height) as f64) * 0.33; // tweak as needed
    let max_tries = 1000;
    for _ in 0..num {
        let mut tries = 0;
        loop {
            let x: f64 = rng.gen_range(0.15..0.85) * width as f64;
            let y: f64 = rng.gen_range(0.15..0.85) * height as f64;
            if centers.iter().all(|&(cx, cy): &(f64, f64)| ((x - cx).powi(2) + (y - cy).powi(2)).sqrt() > min_dist) {
                centers.push((x, y));
                break;
            }
            tries += 1;
            if tries > max_tries {
                centers.push((x, y)); // fallback
                break;
            }
        }
    }
    centers
} 