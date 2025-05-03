// Continent center generation logic
use rand::SeedableRng;
use rand::Rng;

/// Smart continent center placement with adaptive falloff.
pub fn generate_continent_centers(seed: u32, width: usize, height: usize, num: usize) -> Vec<(f64, f64)> {
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed as u64);
    let mut centers = Vec::with_capacity(num);
    let map_diag = ((width * width + height * height) as f64).sqrt();
    let falloff_radius = map_diag / (num as f64).sqrt() * 0.7; // adaptive: fewer continents = more spread
    let max_tries = 1000;
    for _ in 0..num {
        let mut best_candidate = None;
        let mut best_score = -1.0;
        for _ in 0..max_tries {
            let x: f64 = rng.gen_range(0.0..1.0) * width as f64;
            let y: f64 = rng.gen_range(0.0..1.0) * height as f64;
            let min_dist = centers.iter()
                .map(|&(cx, cy): &(f64, f64)| ((x - cx).powi(2) + (y - cy).powi(2)).sqrt())
                .fold(f64::INFINITY, f64::min);
            // Falloff: score is higher the farther from existing centers
            let score = if centers.is_empty() {
                1.0
            } else {
                (1.0 - (-min_dist / falloff_radius).exp()).clamp(0.0f64, 1.0f64)
            };
            if score > best_score {
                best_score = score;
                best_candidate = Some((x, y));
            }
            // Accept with probability = score
            if rng.gen_bool(score as f64) {
                centers.push((x, y));
                break;
            }
        }
        // If not accepted after max_tries, use the best candidate
        if centers.len() < centers.capacity() {
            if let Some((x, y)) = best_candidate {
                centers.push((x, y));
            }
        }
    }
    centers
} 