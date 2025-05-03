use rand::Rng;

pub fn add_ranges(
    seed: u32,
    width: usize,
    height: usize,
    elevation: &mut Vec<Vec<f64>>,
) {
    use rand::{Rng, SeedableRng};
    use rand::rngs::StdRng;
    let mut rng = StdRng::seed_from_u64(seed as u64 + 42);
    let num_ranges = 5;
    let range_width = (width.max(height) as f64 * 0.03).max(2.0) as isize; // width in cells
    let range_height = 0.25; // how much to boost elevation at the center

    // Calculate elevation thresholds for hills and mountains
    let mut flat: Vec<f64> = elevation.iter().flatten().copied().collect();
    flat.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mountain_level = flat[(flat.len() as f64 * 0.90) as usize]; // top 10% as mountains
    let hill_level = flat[(flat.len() as f64 * 0.80) as usize];     // next 10% as hills

    // Collect candidate points for hills and mountains
    let mut mountain_points = Vec::new();
    let mut hill_points = Vec::new();
    for x in 0..width as isize {
        for y in 0..height as isize {
            let elev = elevation[x as usize][y as usize];
            if elev >= mountain_level {
                mountain_points.push((x, y));
            } else if elev >= hill_level {
                hill_points.push((x, y));
            }
        }
    }

    for _ in 0..num_ranges {
        // Pick start and end points: prefer mountain-to-mountain, else hill-to-mountain, else hill-to-hill
        let (start, end) = if mountain_points.len() >= 2 && rng.gen_bool(0.7) {
            // 70% chance: mountain-to-mountain
            let a = mountain_points[rng.gen_range(0..mountain_points.len())];
            let mut b = mountain_points[rng.gen_range(0..mountain_points.len())];
            // Ensure start != end
            for _ in 0..10 {
                if a != b { break; }
                b = mountain_points[rng.gen_range(0..mountain_points.len())];
            }
            (a, b)
        } else if !hill_points.is_empty() && !mountain_points.is_empty() {
            // hill-to-mountain
            let a = hill_points[rng.gen_range(0..hill_points.len())];
            let b = mountain_points[rng.gen_range(0..mountain_points.len())];
            (a, b)
        } else if hill_points.len() >= 2 {
            // hill-to-hill
            let a = hill_points[rng.gen_range(0..hill_points.len())];
            let mut b = hill_points[rng.gen_range(0..hill_points.len())];
            for _ in 0..10 {
                if a != b { break; }
                b = hill_points[rng.gen_range(0..hill_points.len())];
            }
            (a, b)
        } else {
            // fallback: random points as before
            let x0 = rng.gen_range(0..width as isize);
            let y0 = rng.gen_range(0..height as isize);
            let x1 = rng.gen_range(0..width as isize);
            let y1 = rng.gen_range(0..height as isize);
            ((x0, y0), (x1, y1))
        };

        let path = generate_noisy_line(width, height, start.0, start.1, end.0, end.1, &mut rng);
        for &(px, py) in &path {
            for dx in -range_width..=range_width {
                for dy in -range_width..=range_width {
                    let nx = px + dx;
                    let ny = py + dy;
                    if nx >= 0 && nx < width as isize && ny >= 0 && ny < height as isize {
                        let dist = ((dx * dx + dy * dy) as f64).sqrt();
                        let falloff = (1.0 - dist / range_width as f64).max(0.0);
                        elevation[nx as usize][ny as usize] += falloff * range_height;
                    }
                }
            }
        }
    }
    // Clamp elevation to [0, 1]
    for x in 0..width {
        for y in 0..height {
            elevation[x][y] = elevation[x][y].clamp(0.0, 1.0);
        }
    }
}

pub fn generate_noisy_line(
    width: usize,
    height: usize,
    x0: isize,
    y0: isize,
    x1: isize,
    y1: isize,
    rng: &mut impl rand::Rng,
) -> Vec<(isize, isize)> {
    let mut points = Vec::new();
    let steps = ((x1 - x0).abs().max((y1 - y0).abs())).max(1) as usize;
    for i in 0..=steps {
        let t = i as f64 / steps as f64;
        let mut x = x0 as f64 * (1.0 - t) + x1 as f64 * t;
        let mut y = y0 as f64 * (1.0 - t) + y1 as f64 * t;
        // Add jitter
        x += rng.gen_range(-1.0..1.0);
        y += rng.gen_range(-1.0..1.0);
        let xi = x.round() as isize;
        let yi = y.round() as isize;
        if xi >= 0 && xi < width as isize && yi >= 0 && yi < height as isize {
            points.push((xi, yi));
        }
    }
    points
}
