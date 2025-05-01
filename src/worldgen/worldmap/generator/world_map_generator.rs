use noise::{NoiseFn, Perlin};
use rand::SeedableRng;
use rand::Rng;
use crate::world::worldmap::biome::BiomeId;
use crate::world::worldmap::world_map::WorldMap;
use super::params::WorldGenParams;
use super::noise_utils::*;
use super::continents::generate_continent_centers;
use super::biome_classify::classify_biome;
use rand::seq::SliceRandom;

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

    pub fn generate(&self) -> WorldMap {
        let p = &self.params;
        let perlin_continent = Perlin::new(self.seed);
        let perlin_detail = Perlin::new(self.seed.wrapping_add(1));
        let perlin_moist = Perlin::new(self.seed.wrapping_add(2));
        let perlin_ridge = Perlin::new(self.seed.wrapping_add(3));

        // Generate continent centers
        let continent_centers = generate_continent_centers(self.seed, self.width, self.height, p.num_continents.max(1));
        let continent_radius = (self.width.min(self.height) as f64) * 0.33;

        // 1. Generate base elevation & moisture
        let (mut elevation, mut moisture) = self.generate_elevation_and_moisture(
            &perlin_continent,
            &perlin_detail,
            &perlin_moist,
            &perlin_ridge,
            &continent_centers,
            continent_radius,
        );

        // === Add connected mountain ranges ===
        self.add_mountain_ranges(&mut elevation);

        // Generate temperature map
        let temperature = self.generate_temperature_map(&elevation);

        // Generate precipitation map
        let precipitation = self.generate_precipitation_map();

        // Generate wind direction map
        let wind_direction = self.generate_wind_direction_map();

        // Generate soil fertility map (based on elevation, precipitation, and proximity to rivers)
        let soil_fertility = self.generate_soil_fertility_map(&elevation, &precipitation, &self.build_river_mask(&self.accumulate_river_flow(&elevation, 0.5, 0.0)));

        // Generate vegetation map (based on temperature, precipitation, and soil fertility)
        let vegetation = self.generate_vegetation_map(&temperature, &precipitation, &soil_fertility);

        // Generate resources map (randomly assign some resources)
        let resources = self.generate_resources_map();

        // 2. Erosion
        self.apply_erosion(&mut elevation);

        // 3. Global percentiles for thresholds
        let (sea_level, coast_level, mountain_level) = self.calculate_percentiles(&elevation);

        // 5. Classify biomes
        let biomes = self.classify_biomes(
            &elevation,
            &moisture,
            &self.accumulate_river_flow(&elevation, coast_level, sea_level),
            &perlin_ridge,
            sea_level,
            coast_level,
            mountain_level,
        );

        // 6. Build river mask
        let rivers = self.build_river_mask(&self.accumulate_river_flow(&elevation, coast_level, sea_level));

        // === New: Generate category maps ===
        use crate::world::worldmap::biome::{classify_temperature, classify_vegetation, classify_precipitation, classify_elevation, TemperatureType, VegetationType, PrecipitationType, ElevationType};
        let mut temperature_map = vec![vec![TemperatureType::Temperate; self.height]; self.width];
        let mut vegetation_map = vec![vec![VegetationType::Grass; self.height]; self.width];
        let mut precipitation_map = vec![vec![PrecipitationType::Moderate; self.height]; self.width];
        let mut elevation_map = vec![vec![ElevationType::Lowland; self.height]; self.width];
        for x in 0..self.width {
            for y in 0..self.height {
                let temp = temperature[x][y];
                let veg = vegetation[x][y];
                let precip = precipitation[x][y];
                let elev = elevation[x][y];
                temperature_map[x][y] = classify_temperature(temp);
                vegetation_map[x][y] = classify_vegetation(veg, temp, precip);
                precipitation_map[x][y] = classify_precipitation(precip);
                elevation_map[x][y] = classify_elevation(elev, sea_level, coast_level, mountain_level);
            }
        }

        // === LOGGING: World properties and biome/elevation counts ===
        println!("--- World Generation Log ---");
        println!("Seed: {} | Size: {}x{} | Scale: {}", self.seed, self.width, self.height, self.scale);
        println!("Params: ocean_percent={:.2}, coast_percent={:.2}, mountain_percent={:.2}, erosion_iterations={}, river_threshold={:.2}, num_continents={} ",
            p.ocean_percent, p.coast_percent, p.mountain_percent, p.erosion_iterations, p.river_threshold, p.num_continents);
        println!("Thresholds: sea_level={:.3}, coast_level={:.3}, mountain_level={:.3}", sea_level, coast_level, mountain_level);
        let mut elevation_counts: HashMap<ElevationType, usize> = HashMap::new();
        for x in 0..self.width {
            for y in 0..self.height {
                let et = elevation_map[x][y];
                *elevation_counts.entry(et).or_insert(0) += 1;
            }
        }
        println!("ElevationType counts:");
        for (et, count) in &elevation_counts {
            println!("  {:?}: {} ({:.2}%)", et, count, (*count as f64) * 100.0 / (self.width * self.height) as f64);
        }
        let mut biome_counts: HashMap<BiomeId, usize> = HashMap::new();
        for x in 0..self.width {
            for y in 0..self.height {
                let b = biomes[x][y];
                *biome_counts.entry(b).or_insert(0) += 1;
            }
        }
        println!("BiomeId counts:");
        for (b, count) in &biome_counts {
            println!("  {:?}: {} ({:.2}%)", b, count, (*count as f64) * 100.0 / (self.width * self.height) as f64);
        }
        println!("---------------------------");

        // 4. Flow accumulation for rivers
        let flow = self.accumulate_river_flow(&elevation, coast_level, sea_level);

        // 7. Build river mask
        let rivers = self.build_river_mask(&flow);

        // === Civilization and City Generation ===
        use crate::world::worldmap::{Civilization, CivilizationInstance, Culture, Alignment, SocietalTrait, City};
        use rand::{SeedableRng, Rng};
        use rand::rngs::StdRng;
        let mut rng = StdRng::seed_from_u64(self.seed as u64 + 1000);
        let civ_types = [
            Civilization::Human,
            Civilization::Elf,
            Civilization::Dwarf,
            Civilization::GnomeHalfling,
            Civilization::OrcGoblin,
            Civilization::Merfolk,
            Civilization::Lizardfolk,
            Civilization::FairyFae,
            Civilization::Kobold,
        ];
        let alignments = [
            Alignment::LawfulGood, Alignment::NeutralGood, Alignment::ChaoticGood,
            Alignment::LawfulNeutral, Alignment::TrueNeutral, Alignment::ChaoticNeutral,
            Alignment::LawfulEvil, Alignment::NeutralEvil, Alignment::ChaoticEvil,
        ];
        let traits = [
            SocietalTrait::Isolationist, SocietalTrait::Expansionist, SocietalTrait::Nomadic,
            SocietalTrait::Mercantile, SocietalTrait::Militaristic, SocietalTrait::Scholarly, SocietalTrait::Spiritual,
        ];
        let traditions = [
            "Ancestor Worship", "Arcane Scholarship", "Nature Reverence", "Warrior Code", "Trade Guilds", "Seafaring", "Stonecraft", "Dreamwalking", "Shadow Pact"
        ];
        let religions = [
            "Sun God", "Nature Spirits", "Ancestor Ghosts", "The Great Machine", "The Deep", "The Dragon", "The Moon Court", "The Old Ones", "The Flame"
        ];
        // Place civilization seeds
        let num_civs = civ_types.len();
        let mut civ_seeds = Vec::new();
        for &civ in &civ_types {
            // Try to find a land tile for the civ seed
            for _ in 0..100 {
                let x = rng.gen_range(0..self.width);
                let y = rng.gen_range(0..self.height);
                if elevation[x][y] > sea_level && !civ_seeds.iter().any(|(cx,cy,_)| (*cx as isize - x as isize).abs() < 10 && (*cy as isize - y as isize).abs() < 10) {
                    let culture = Culture {
                        alignment: *alignments.choose(&mut rng).unwrap(),
                        tradition: traditions.choose(&mut rng).unwrap().to_string(),
                        religion: religions.choose(&mut rng).unwrap().to_string(),
                        trait_: *traits.choose(&mut rng).unwrap(),
                    };
                    civ_seeds.push((x, y, CivilizationInstance { civ_type: civ, culture }));
                    break;
                }
            }
        }
        // Assign each tile to the nearest civ seed (Voronoi)
        let mut civilization_map = vec![vec![None; self.height]; self.width];
        for x in 0..self.width {
            for y in 0..self.height {
                if elevation[x][y] > sea_level {
                    let mut min_dist = f64::MAX;
                    let mut nearest = None;
                    for (cx, cy, civ) in &civ_seeds {
                        let dist = ((*cx as isize - x as isize).pow(2) + (*cy as isize - y as isize).pow(2)) as f64;
                        if dist < min_dist {
                            min_dist = dist;
                            nearest = Some(civ.clone());
                        }
                    }
                    civilization_map[x][y] = nearest;
                }
            }
        }
        // Place cities for each civilization
        let mut cities = Vec::new();
        let city_names = [
            "Aldoria", "Brighthaven", "Stonehelm", "Rivermouth", "Frostford", "Sunspire", "Shadowfen", "Goldport", "Ironhold", "Starfall", "Mistwood", "Deepmere", "Windrest", "Moonwatch", "Thundertop"
        ];
        for (seed_x, seed_y, civ_inst) in &civ_seeds {
            let mut placed = 0;
            let max_cities = 3;
            for _ in 0..200 {
                let x = rng.gen_range(0..self.width);
                let y = rng.gen_range(0..self.height);
                if civilization_map[x][y].as_ref().map(|c| c.civ_type) == Some(civ_inst.civ_type)
                    && elevation[x][y] > sea_level
                    && (x != *seed_x || y != *seed_y)
                    && !cities.iter().any(|c: &City| (c.x as isize - x as isize).abs() < 5 && (c.y as isize - y as isize).abs() < 5)
                {
                    let name = city_names.choose(&mut rng).unwrap_or(&"Unnamed").to_string();
                    let population = rng.gen_range(1000..100_000);
                    cities.push(City {
                        name,
                        civ: civ_inst.civ_type,
                        x,
                        y,
                        population,
                    });
                    placed += 1;
                    if placed >= max_cities { break; }
                }
            }
            // Always place a capital at the seed
            let name = format!("{} Capital", match civ_inst.civ_type {
                Civilization::Human => "Human",
                Civilization::Elf => "Elf",
                Civilization::Dwarf => "Dwarf",
                Civilization::GnomeHalfling => "Gnome/Halfling",
                Civilization::OrcGoblin => "Orc/Goblin",
                Civilization::Merfolk => "Merfolk",
                Civilization::Lizardfolk => "Lizardfolk",
                Civilization::FairyFae => "Fairy/Fae",
                Civilization::Kobold => "Kobold",
            });
            let population = rng.gen_range(50_000..500_000);
            cities.push(City {
                name,
                civ: civ_inst.civ_type,
                x: *seed_x,
                y: *seed_y,
                population,
            });
        }

        // === Civilization Relations and Trade Routes ===
        use crate::world::worldmap::{Relation, CivilizationRelations, TradeRoute};
        use std::collections::HashMap;
        let mut relations = HashMap::new();
        // Assign random relations between all pairs (symmetric, no self-relations)
        for i in 0..civ_types.len() {
            for j in (i+1)..civ_types.len() {
                let rel = match rng.gen_range(0..10) {
                    0 => Relation::War,
                    1..=3 => Relation::Peace,
                    _ => Relation::Neutral,
                };
                relations.insert((civ_types[i], civ_types[j]), rel);
                relations.insert((civ_types[j], civ_types[i]), rel);
            }
        }
        let civ_relations = CivilizationRelations { relations };
        // Find capitals for each civ
        let mut capitals = HashMap::new();
        for (seed_x, seed_y, civ_inst) in &civ_seeds {
            capitals.insert(civ_inst.civ_type, (*seed_x, *seed_y));
        }
        // Generate trade routes between capitals of civs at Peace
        let trade_routes = self.generate_trade_routes(
            &capitals,
            &civ_types,
            &civ_relations,
            &cities,
            &elevation,
            &rivers,
            sea_level,
        );

        WorldMap {
            width: self.width,
            height: self.height,
            biomes,
            elevation,
            moisture,
            rivers,
            temperature,
            precipitation,
            soil_fertility,
            vegetation,
            wind_direction,
            resources,
            temperature_map,
            vegetation_map,
            precipitation_map,
            elevation_map,
            civilization_map,
            cities,
            civ_relations,
            trade_routes,
            sea_level,
        }
    }

    fn generate_elevation_and_moisture(
        &self,
        perlin_continent: &Perlin,
        perlin_detail: &Perlin,
        perlin_moist: &Perlin,
        perlin_ridge: &Perlin,
        continent_centers: &Vec<(f64, f64)>,
        continent_radius: f64,
    ) -> (Vec<Vec<f64>>, Vec<Vec<f64>>) {
        let p = &self.params;
        let mut elevation = vec![vec![0.0; self.height]; self.width];
        let mut moisture = vec![vec![0.0; self.height]; self.width];
        // New: Plateau and crater noise
        let perlin_plateau = Perlin::new(self.seed.wrapping_add(100));
        let perlin_crater = Perlin::new(self.seed.wrapping_add(200));
        // Generate random craters
        let mut craters = Vec::new();
        let num_craters = 5;
        let mut rng = rand::rngs::StdRng::seed_from_u64(self.seed as u64 + 999);
        for _ in 0..num_craters {
            let cx = rng.gen_range(0.1..0.9) * self.width as f64;
            let cy = rng.gen_range(0.1..0.9) * self.height as f64;
            let r = rng.gen_range(8.0..24.0);
            craters.push((cx, cy, r));
        }
        for x in 0..self.width {
            for y in 0..self.height {
                let nx = x as f64 / self.width as f64 - 0.5;
                let ny = y as f64 / self.height as f64 - 0.5;
                let c = fractal_noise(
                    perlin_continent,
                    nx * self.scale * p.continent_scale * 1.5,
                    ny * self.scale * p.continent_scale * 1.5,
                    p.octaves_continent,
                    p.persistence,
                );
                let continental_mask = (c - 0.2).clamp(0.0, 1.0);
                let mut min_dist = 1.0;
                for &(cx, cy) in continent_centers {
                    let dx = x as f64 - cx;
                    let dy = y as f64 - cy;
                    let dist = ((dx * dx + dy * dy).sqrt()) / continent_radius;
                    if dist < min_dist {
                        min_dist = dist;
                    }
                }
                let jagged = fractal_noise(perlin_detail, nx * 2.0, ny * 2.0, 2, 0.5);
                // Modulate continent falloff with extra noise for more jagged edges
                let falloff_noise = perlin_detail.get([nx * 3.0, ny * 3.0]);
                let continent_falloff = ((1.0 - min_dist) + 0.3 * jagged + 0.15 * falloff_noise).clamp(0.0, 1.0);
                let d = fractal_noise(
                    perlin_detail,
                    nx * self.scale * p.detail_scale,
                    ny * self.scale * p.detail_scale,
                    p.octaves_detail,
                    p.persistence,
                );
                // Plateau noise for mesas and highlands
                let plateau = (perlin_plateau.get([nx * self.scale * 0.7, ny * self.scale * 0.7]) * 0.5 + 0.5).powf(2.0) * 0.18;
                // Ridge noise for mountains (stronger influence)
                let r = 1.0 - perlin_ridge.get([nx * self.scale * 2.0, ny * self.scale * 2.0]).abs();
                let ridge = (r.powi(3)) * 0.7;
                // Crater effect: subtract elevation in crater centers
                let mut crater_effect = 0.0;
                for &(cx, cy, rad) in &craters {
                    let dx = x as f64 - cx;
                    let dy = y as f64 - cy;
                    let dist = (dx * dx + dy * dy).sqrt();
                    if dist < rad {
                        let norm: f64 = 1.0 - (dist / rad);
                        crater_effect -= norm.powf(1.5) * 0.25;
                    }
                }
                // Lower the baseline and reduce some weights
                let mut e = continental_mask * 0.5 + d * 0.15 + ridge + continent_falloff * 0.4 + plateau + crater_effect - 0.15;
                e = e.clamp(0.0, 1.0);
                elevation[x][y] = e;
                let m = perlin_moist.get([nx * self.scale, ny * self.scale]);
                moisture[x][y] = m;
            }
        }
        // Print elevation stats
        let mut min_elev = 1.0;
        let mut max_elev = 0.0;
        let mut sum_elev = 0.0;
        let mut count = 0.0;
        for x in 0..self.width {
            for y in 0..self.height {
                let e = elevation[x][y];
                if e < min_elev { min_elev = e; }
                if e > max_elev { max_elev = e; }
                sum_elev += e;
                count += 1.0;
            }
        }
        println!("Elevation stats: min={:.3}, max={:.3}, mean={:.3}", min_elev, max_elev, sum_elev / count);
        (elevation, moisture)
    }

    /// Adds connected mountain ranges by drawing noisy lines and boosting elevation along those lines.
    fn add_mountain_ranges(&self, elevation: &mut Vec<Vec<f64>>) {
        use rand::{Rng, SeedableRng};
        use rand::rngs::StdRng;
        let mut rng = StdRng::seed_from_u64(self.seed as u64 + 42);
        let num_ranges = 5;
        let range_width = (self.width.max(self.height) as f64 * 0.03).max(2.0) as isize; // width in cells
        let range_height = 0.25; // how much to boost elevation at the center
        for _ in 0..num_ranges {
            let x0 = rng.gen_range(0..self.width as isize);
            let y0 = rng.gen_range(0..self.height as isize);
            let x1 = rng.gen_range(0..self.width as isize);
            let y1 = rng.gen_range(0..self.height as isize);
            let path = self.generate_noisy_line(x0, y0, x1, y1, &mut rng);
            for &(px, py) in &path {
                for dx in -range_width..=range_width {
                    for dy in -range_width..=range_width {
                        let nx = px + dx;
                        let ny = py + dy;
                        if nx >= 0 && nx < self.width as isize && ny >= 0 && ny < self.height as isize {
                            let dist = ((dx * dx + dy * dy) as f64).sqrt();
                            let falloff = (1.0 - dist / range_width as f64).max(0.0);
                            elevation[nx as usize][ny as usize] += falloff * range_height;
                        }
                    }
                }
            }
        }
        // Clamp elevation to [0, 1]
        for x in 0..self.width {
            for y in 0..self.height {
                elevation[x][y] = elevation[x][y].clamp(0.0, 1.0);
            }
        }
    }

    /// Generates a noisy line (with jitter) between two points.
    fn generate_noisy_line(&self, x0: isize, y0: isize, x1: isize, y1: isize, rng: &mut impl rand::Rng) -> Vec<(isize, isize)> {
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
            if xi >= 0 && xi < self.width as isize && yi >= 0 && yi < self.height as isize {
                points.push((xi, yi));
            }
        }
        points
    }

    /// Generate a temperature map based on latitude and elevation
    fn generate_temperature_map(&self, elevation: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
        let mut temperature = vec![vec![0.0; self.height]; self.width];
        let equator = self.height as f64 / 2.0;
        for x in 0..self.width {
            for y in 0..self.height {
                let latitude = ((y as f64 - equator).abs()) / equator; // 0 at equator, 1 at poles
                let elev = elevation[x][y];
                // Base temp: hot at equator, cold at poles, colder at high elevation
                let base_temp = 1.0 - latitude - (elev * 0.5); // tweak as needed
                // Clamp to [0, 1] for normalized temperature
                temperature[x][y] = base_temp.clamp(0.0, 1.0);
            }
        }
        temperature
    }

    /// Generate a precipitation map using Perlin noise
    fn generate_precipitation_map(&self) -> Vec<Vec<f64>> {
        let perlin = Perlin::new(self.seed.wrapping_add(10));
        let mut precipitation = vec![vec![0.0; self.height]; self.width];
        for x in 0..self.width {
            for y in 0..self.height {
                let nx = x as f64 / self.width as f64 - 0.5;
                let ny = y as f64 / self.height as f64 - 0.5;
                let p = perlin.get([nx * self.scale, ny * self.scale]);
                precipitation[x][y] = ((p + 1.0) / 2.0).clamp(0.0, 1.0); // Normalize to [0,1]
            }
        }
        precipitation
    }

    /// Generate a wind direction map (simple: constant eastward with some noise)
    fn generate_wind_direction_map(&self) -> Vec<Vec<(f64, f64)>> {
        let perlin = Perlin::new(self.seed.wrapping_add(20));
        let mut wind = vec![vec![(1.0, 0.0); self.height]; self.width];
        for x in 0..self.width {
            for y in 0..self.height {
                let nx = x as f64 / self.width as f64 - 0.5;
                let ny = y as f64 / self.height as f64 - 0.5;
                let angle = perlin.get([nx * self.scale, ny * self.scale]) * std::f64::consts::PI;
                wind[x][y] = (angle.cos(), angle.sin());
            }
        }
        wind
    }

    /// Generate soil fertility map (high near rivers, moderate with good precipitation, low at high elevation)
    fn generate_soil_fertility_map(&self, elevation: &Vec<Vec<f64>>, precipitation: &Vec<Vec<f64>>, rivers: &Vec<Vec<bool>>) -> Vec<Vec<f64>> {
        let mut fertility = vec![vec![0.0; self.height]; self.width];
        for x in 0..self.width {
            for y in 0..self.height {
                let mut f = 0.5 * precipitation[x][y] + 0.3 * (1.0 - elevation[x][y]);
                if rivers[x][y] {
                    f += 0.3;
                }
                fertility[x][y] = f.clamp(0.0, 1.0);
            }
        }
        fertility
    }

    /// Generate vegetation map (based on temperature, precipitation, and soil fertility)
    fn generate_vegetation_map(&self, temperature: &Vec<Vec<f64>>, precipitation: &Vec<Vec<f64>>, soil_fertility: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
        let mut vegetation = vec![vec![0.0; self.height]; self.width];
        for x in 0..self.width {
            for y in 0..self.height {
                let v = 0.4 * temperature[x][y] + 0.4 * precipitation[x][y] + 0.2 * soil_fertility[x][y];
                vegetation[x][y] = v.clamp(0.0, 1.0);
            }
        }
        vegetation
    }

    /// Generate resources map (randomly assign some resources)
    fn generate_resources_map(&self) -> Vec<Vec<Option<crate::world::worldmap::world_map::ResourceType>>> {
        use crate::world::worldmap::world_map::ResourceType;
        let perlin = Perlin::new(self.seed.wrapping_add(30));
        let mut resources = vec![vec![None; self.height]; self.width];
        for x in 0..self.width {
            for y in 0..self.height {
                let nx = x as f64 / self.width as f64 - 0.5;
                let ny = y as f64 / self.height as f64 - 0.5;
                let noise = perlin.get([nx * self.scale * 2.0, ny * self.scale * 2.0]);
                let v = ((noise + 1.0) / 2.0).clamp(0.0, 1.0);
                resources[x][y] = if v > 0.97 {
                    // 3% chance for a resource
                    let r = ((x * 31 + y * 17 + self.seed as usize) % 5) as u8;
                    Some(match r {
                        0 => ResourceType::Iron,
                        1 => ResourceType::Gold,
                        2 => ResourceType::Coal,
                        3 => ResourceType::Gems,
                        _ => ResourceType::Oil,
                    })
                } else {
                    None
                };
            }
        }
        resources
    }

    fn apply_erosion(&self, elevation: &mut Vec<Vec<f64>>) {
        let p = &self.params;
        for _ in 0..p.erosion_iterations {
            erosion_pass(elevation);
        }
    }

    fn calculate_percentiles(&self, elevation: &Vec<Vec<f64>>) -> (f64, f64, f64) {
        let p = &self.params;
        let mut flat: Vec<f64> = elevation.iter().flatten().copied().collect();
        let sea_level = percentile(&mut flat, p.ocean_percent);
        let coast_level = percentile(&mut flat, p.ocean_percent + p.coast_percent);
        let mountain_level = percentile(&mut flat, 1.0 - p.mountain_percent);
        (sea_level, coast_level, mountain_level)
    }

    fn accumulate_river_flow(&self, elevation: &Vec<Vec<f64>>, coast_level: f64, sea_level: f64) -> Vec<Vec<f64>> {
        let mut flow = vec![vec![0.0; self.height]; self.width];
        for x in 0..self.width {
            for y in 0..self.height {
                if elevation[x][y] > coast_level {
                    accumulate_flow(elevation, &mut flow, x, y, sea_level);
                }
            }
        }
        flow
    }

    fn classify_biomes(
        &self,
        elevation: &Vec<Vec<f64>>,
        moisture: &Vec<Vec<f64>>,
        flow: &Vec<Vec<f64>>,
        perlin_ridge: &Perlin,
        sea_level: f64,
        coast_level: f64,
        mountain_level: f64,
    ) -> Vec<Vec<BiomeId>> {
        let p = &self.params;
        let mut biomes = vec![vec![BiomeId::Plains; self.height]; self.width];
        let lat_factor = 1.0 / (self.height as f64 / 2.0);
        let temperature = &self.generate_temperature_map(elevation);
        let precipitation = &self.generate_precipitation_map();
        let soil_fertility = &self.generate_soil_fertility_map(elevation, precipitation, &self.build_river_mask(flow));
        let vegetation = &self.generate_vegetation_map(temperature, precipitation, soil_fertility);
        for x in 0..self.width {
            for y in 0..self.height {
                let e = elevation[x][y];
                let m = moisture[x][y];
                let nx = x as f64 / self.width as f64 - 0.5;
                let ny = y as f64 / self.height as f64 - 0.5;
                let r = 1.0 - perlin_ridge.get([nx * self.scale * 2.0, ny * self.scale * 2.0]).abs();
                let ridge = (r.powi(3)) * 0.6;
                let latitude = ((y as f64 - self.height as f64 / 2.0).abs()) * lat_factor;
                let temp = temperature[x][y];
                let precip = precipitation[x][y];
                let fert = soil_fertility[x][y];
                let veg = vegetation[x][y];
                let river_here = flow[x][y] > p.river_threshold;
                biomes[x][y] = classify_biome(
                    x, y, self.width, self.height, e, m, ridge, temp, river_here,
                    sea_level, coast_level, mountain_level,
                    precip, fert, veg
                );
            }
        }
        biomes
    }

    fn build_river_mask(&self, flow: &Vec<Vec<f64>>) -> Vec<Vec<bool>> {
        let p = &self.params;
        let mut rivers = vec![vec![false; self.height]; self.width];
        for x in 0..self.width {
            for y in 0..self.height {
                if flow[x][y] > p.river_threshold {
                    rivers[x][y] = true;
                }
            }
        }
        rivers
    }

    /// Generate trade routes between capitals of civs at Peace. This is a stub to be filled with pathfinding logic.
    fn generate_trade_routes(
        &self,
        capitals: &std::collections::HashMap<crate::world::worldmap::Civilization, (usize, usize)>,
        civ_types: &[crate::world::worldmap::Civilization],
        civ_relations: &crate::world::worldmap::CivilizationRelations,
        cities: &[crate::world::worldmap::city::City],
        elevation: &Vec<Vec<f64>>,
        rivers: &Vec<Vec<bool>>,
        sea_level: f64,
    ) -> Vec<crate::world::worldmap::TradeRoute> {
        // TODO: Implement pathfinding for land/sea/river trade routes
        // For now, just return direct from-to routes as before, with only endpoints in path
        let mut trade_routes = Vec::new();
        for i in 0..civ_types.len() {
            for j in (i+1)..civ_types.len() {
                let civ_a = civ_types[i];
                let civ_b = civ_types[j];
                if let Some(crate::world::worldmap::Relation::Peace) = civ_relations.relations.get(&(civ_a, civ_b)) {
                    if let (Some(&(ax, ay)), Some(&(bx, by))) = (capitals.get(&civ_a), capitals.get(&civ_b)) {
                        trade_routes.push(crate::world::worldmap::TradeRoute {
                            from: (ax, ay),
                            to: (bx, by),
                            civ_a,
                            civ_b,
                            path: vec![(ax, ay), (bx, by)], // TODO: Replace with full path
                        });
                    }
                }
            }
        }
        trade_routes
    }
} 