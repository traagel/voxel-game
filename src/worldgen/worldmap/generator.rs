// world_map_generator.rs – self‑contained example
// external dependency: noise = "0.9"

use noise::{NoiseFn, Perlin};
use rand::seq::SliceRandom;
use rand::SeedableRng;
use rand::Rng;
use crate::world::worldmap::biome::BiomeId;
use crate::world::worldmap::world_map::WorldMap;

/// All tweakable knobs live here so you do not hunt hard‑coded numbers.
#[derive(Clone, Copy)]
pub struct WorldGenParams {
    pub ocean_percent: f64,        // portion of tiles that should be water
    pub coast_percent: f64,        // thin band above sea level
    pub mountain_percent: f64,     // top X % counted as mountains
    pub erosion_iterations: usize, // cheap thermal erosion passes
    pub river_threshold: f64,      // min flow to be called a river
    pub continent_scale: f64,      // low‑freq scale (big shapes)
    pub detail_scale: f64,         // hi‑freq scale (little bumps)
    pub octaves_continent: usize,
    pub octaves_detail: usize,
    pub persistence: f64, // FBM persistence for both noises
    pub num_continents: usize, // number of major continents
}

impl Default for WorldGenParams {
    fn default() -> Self {
        Self {
            ocean_percent: 0.35,
            coast_percent: 0.10,
            mountain_percent: 0.10,
            erosion_iterations: 30,
            river_threshold: 120.0,
            continent_scale: 0.25,
            detail_scale: 4.0,
            octaves_continent: 3,
            octaves_detail: 5,
            persistence: 0.5,
            num_continents: 3,
        }
    }
}

pub struct WorldMapGenerator {
    seed: u32,
    width: usize,
    height: usize,
    scale: f64,
    params: WorldGenParams,
}

impl WorldMapGenerator {
    pub fn new(
        seed: u32,
        width: usize,
        height: usize,
        scale: f64,
        params: Option<WorldGenParams>,
    ) -> Self {
        Self {
            seed,
            width,
            height,
            scale,
            params: params.unwrap_or_default(),
        }
    }

    /// Classic fractional Brownian motion helper.
    fn fractal_noise(
        &self,
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
    fn percentile(values: &mut [f64], p: f64) -> f64 {
        values.sort_by(|a, b| a.partial_cmp(b).unwrap()); // total order via NaN-free cmp
        let idx = ((values.len() as f64) * p).floor() as usize;
        values[idx]
    }

    /// One step of extremely cheap thermal erosion.
    fn erosion_pass(elev: &mut [Vec<f64>]) {
        let w = elev.len();
        let h = elev[0].len();
        let mut coords: Vec<(usize, usize)> =
            (0..w).flat_map(|x| (0..h).map(move |y| (x, y))).collect();
        coords.shuffle(&mut rand::thread_rng());
        for (x, y) in coords {
            let (nx, ny) = Self::lowest_neighbor(elev, x, y);
            if (nx, ny) != (x, y) {
                let diff = (elev[x][y] - elev[nx][ny]) * 0.05;
                elev[x][y] -= diff;
                elev[nx][ny] += diff;
            }
        }
    }

    fn lowest_neighbor(elev: &[Vec<f64>], x: usize, y: usize) -> (usize, usize) {
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
    fn accumulate_flow(
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
            let (nx, ny) = Self::lowest_neighbor(elev, cx, cy);
            if (nx, ny) == (cx, cy) {
                break;
            }
            cx = nx;
            cy = ny;
        }
    }

    /// Generate random continent centers
    fn generate_continent_centers(&self, num: usize) -> Vec<(f64, f64)> {
        let mut rng = rand::rngs::StdRng::seed_from_u64(self.seed as u64);
        let mut centers = Vec::with_capacity(num);
        for _ in 0..num {
            let x = rng.gen_range(0.15..0.85) * self.width as f64;
            let y = rng.gen_range(0.15..0.85) * self.height as f64;
            centers.push((x, y));
        }
        centers
    }

    pub fn generate(&self) -> WorldMap {
        let p = &self.params;
        let perlin_continent = Perlin::new(self.seed);
        let perlin_detail = Perlin::new(self.seed.wrapping_add(1));
        let perlin_moist = Perlin::new(self.seed.wrapping_add(2));
        let perlin_ridge = Perlin::new(self.seed.wrapping_add(3));

        // Generate continent centers
        let continent_centers = self.generate_continent_centers(p.num_continents.max(1));
        let continent_radius = (self.width.min(self.height) as f64) * 0.33;

        let mut elevation = vec![vec![0.0; self.height]; self.width];
        let mut moisture = vec![vec![0.0; self.height]; self.width];

        // 1. Generate base elevation & moisture
        for x in 0..self.width {
            for y in 0..self.height {
                let nx = x as f64 / self.width as f64 - 0.5;
                let ny = y as f64 / self.height as f64 - 0.5;

                // Large‑scale continents (higher frequency)
                let c = self.fractal_noise(
                    &perlin_continent,
                    nx * self.scale * p.continent_scale * 1.5,
                    ny * self.scale * p.continent_scale * 1.5,
                    p.octaves_continent,
                    p.persistence,
                );
                let continental_mask = (c - 0.2).clamp(0.0, 1.0);

                // Radial mask from nearest continent center
                let mut min_dist = 1.0;
                for &(cx, cy) in &continent_centers {
                    let dx = x as f64 - cx;
                    let dy = y as f64 - cy;
                    let dist = ((dx * dx + dy * dy).sqrt()) / continent_radius;
                    if dist < min_dist {
                        min_dist = dist;
                    }
                }
                let continent_falloff = (1.0 - min_dist).clamp(0.0, 1.0);

                // Local detail
                let d = self.fractal_noise(
                    &perlin_detail,
                    nx * self.scale * p.detail_scale,
                    ny * self.scale * p.detail_scale,
                    p.octaves_detail,
                    p.persistence,
                );

                // Ridged noise for mountain belts
                let r = 1.0 - perlin_ridge.get([nx * self.scale, ny * self.scale]).abs();
                let ridge = (r.powi(3)) * 0.3;

                // Combine: blend continent mask, falloff, and detail
                let mut e = continental_mask * 0.6 + d * 0.2 + ridge + continent_falloff * 0.5;
                e = e.clamp(0.0, 1.0);

                elevation[x][y] = e;

                // Moisture is simpler noise field
                let m = perlin_moist.get([nx * self.scale, ny * self.scale]);
                moisture[x][y] = m;
            }
        }

        // 2. Erosion
        for _ in 0..p.erosion_iterations {
            Self::erosion_pass(&mut elevation);
        }

        // 3. Global percentiles for thresholds
        let mut flat: Vec<f64> = elevation.iter().flatten().copied().collect();
        let sea_level = Self::percentile(&mut flat, p.ocean_percent);
        let coast_level = Self::percentile(&mut flat, p.ocean_percent + p.coast_percent);
        let mountain_level = Self::percentile(&mut flat, 1.0 - p.mountain_percent);

        // 4. Flow accumulation for rivers
        let mut flow = vec![vec![0.0; self.height]; self.width];
        for x in 0..self.width {
            for y in 0..self.height {
                if elevation[x][y] > coast_level {
                    Self::accumulate_flow(&elevation, &mut flow, x, y, sea_level);
                }
            }
        }

        // 5. Classify biomes
        let mut biomes = vec![vec![BiomeId::Plains; self.height]; self.width];
        let lat_factor = 1.0 / (self.height as f64 / 2.0);
        for x in 0..self.width {
            for y in 0..self.height {
                let e = elevation[x][y];
                let m = moisture[x][y];
                let latitude = ((y as f64 - self.height as f64 / 2.0).abs()) * lat_factor; // 0 = equator
                let temp = 1.0 - latitude - (e * 0.1).max(0.0); // crude lapse‑rate

                let river_here = flow[x][y] > p.river_threshold;

                biomes[x][y] = if e <= sea_level {
                    BiomeId::Ocean
                } else if river_here {
                    BiomeId::River
                } else if e <= coast_level {
                    BiomeId::Sea
                } else if e >= mountain_level {
                    if temp < 0.3 {
                        BiomeId::Snow
                    } else {
                        BiomeId::Mountain
                    }
                } else {
                    // Inland biomes – very rough κ‑means
                    match (temp, m) {
                        (t, _) if t < 0.25 => BiomeId::Tundra,
                        (t, _) if t < 0.45 => BiomeId::Taiga,
                        (t, moisture) if t > 0.75 && moisture < -0.1 => BiomeId::Desert,
                        (t, moisture) if t > 0.75 && moisture > 0.4 => BiomeId::Jungle,
                        (_, moisture) if moisture > 0.6 => BiomeId::Swamp,
                        (_, moisture) if moisture < -0.2 => BiomeId::Savanna,
                        (_, moisture) if moisture > 0.2 => BiomeId::Forest,
                        _ => BiomeId::Plains,
                    }
                };
            }
        }

        // 6. Build river mask
        let mut rivers = vec![vec![false; self.height]; self.width];
        for x in 0..self.width {
            for y in 0..self.height {
                if flow[x][y] > p.river_threshold {
                    rivers[x][y] = true;
                }
            }
        }

        WorldMap {
            width: self.width,
            height: self.height,
            biomes,
            elevation,
            moisture,
            rivers,
        }
    }
}
