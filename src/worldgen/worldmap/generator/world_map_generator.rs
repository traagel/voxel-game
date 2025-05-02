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
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::cmp::Ordering;
use std::time::Instant;

#[derive(Copy, Clone)]
struct Node {
    pos: (usize, usize),
    cost: f64,
    est_total: f64,
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Reverse for min-heap
        other.est_total.partial_cmp(&self.est_total).unwrap_or(std::cmp::Ordering::Equal)
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}
impl Eq for Node {}

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
        let total_start = Instant::now();
        let p = &self.params;
        let perlin_continent = Perlin::new(self.seed);
        let perlin_detail = Perlin::new(self.seed.wrapping_add(1));
        let perlin_moist = Perlin::new(self.seed.wrapping_add(2));
        let perlin_ridge = Perlin::new(self.seed.wrapping_add(3));

        // Generate continent centers
        let continent_centers = generate_continent_centers(self.seed, self.width, self.height, p.num_continents.max(1));
        let continent_radius = (self.width.min(self.height) as f64) * 0.33;

        // 1. Generate base elevation & moisture
        let elev_start = Instant::now();
        let (mut elevation, mut moisture) = self.generate_elevation_and_moisture(
            &perlin_continent,
            &perlin_detail,
            &perlin_moist,
            &perlin_ridge,
            &continent_centers,
            continent_radius,
        );
        println!("[GEN] Elevation & moisture: {:.2?}", elev_start.elapsed());

        // === Add connected mountain ranges ===
        let mountain_start = Instant::now();
        self.add_mountain_ranges(&mut elevation);
        println!("[GEN] Mountain ranges: {:.2?}", mountain_start.elapsed());

        // Generate temperature map
        let temp_start = Instant::now();
        let temperature = self.generate_temperature_map(&elevation);
        println!("[GEN] Temperature map: {:.2?}", temp_start.elapsed());

        // Generate precipitation map
        let precip_start = Instant::now();
        let precipitation = self.generate_precipitation_map();
        println!("[GEN] Precipitation map: {:.2?}", precip_start.elapsed());

        // Generate wind direction map
        let wind_start = Instant::now();
        let wind_direction = self.generate_wind_direction_map();
        println!("[GEN] Wind direction map: {:.2?}", wind_start.elapsed());

        // Generate soil fertility map (based on elevation, precipitation, and proximity to rivers)
        let soil_start = Instant::now();
        let soil_fertility = self.generate_soil_fertility_map(&elevation, &precipitation, &self.build_river_mask(&self.accumulate_river_flow(&elevation, 0.5, 0.0)));
        println!("[GEN] Soil fertility map: {:.2?}", soil_start.elapsed());

        // Generate vegetation map (based on temperature, precipitation, and soil fertility)
        let veg_start = Instant::now();
        let vegetation = self.generate_vegetation_map(&temperature, &precipitation, &soil_fertility);
        println!("[GEN] Vegetation map: {:.2?}", veg_start.elapsed());

        // Generate resources map (randomly assign some resources)
        let res_start = Instant::now();
        let resources = self.generate_resources_map();
        println!("[GEN] Resources map: {:.2?}", res_start.elapsed());

        // 2. Erosion
        let erosion_start = Instant::now();
        self.apply_erosion(&mut elevation);
        println!("[GEN] Erosion: {:.2?}", erosion_start.elapsed());

        // 3. Global percentiles for thresholds
        let percent_start = Instant::now();
        let (sea_level, coast_level, mountain_level) = self.calculate_percentiles(&elevation);
        println!("[GEN] Percentiles: {:.2?}", percent_start.elapsed());

        // 5. Classify biomes
        let biome_start = Instant::now();
        let biomes = self.classify_biomes(
            &elevation,
            &moisture,
            &self.accumulate_river_flow(&elevation, coast_level, sea_level),
            &perlin_ridge,
            sea_level,
            coast_level,
            mountain_level,
        );
        println!("[GEN] Biome classification: {:.2?}", biome_start.elapsed());

        // 6. Build river mask
        let river_start = Instant::now();
        let rivers = self.build_river_mask(&self.accumulate_river_flow(&elevation, coast_level, sea_level));
        println!("[GEN] River mask: {:.2?}", river_start.elapsed());

        // === New: Generate category maps ===
        let catmap_start = Instant::now();
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
        println!("[GEN] Category maps: {:.2?}", catmap_start.elapsed());

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
        let civ_start = Instant::now();
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
        // Place multiple civilization seeds per type
        let mut civ_seeds = Vec::new();
        for &civ in &civ_types {
            let num_seeds = rng.gen_range(2..=4); // 2-4 per civ type
            let preferred_biomes = civ.preferred_biomes();
            for _ in 0..num_seeds {
                let mut found = false;
                // Try to find a preferred biome tile for the civ seed
                for _ in 0..100 {
                    let x = rng.gen_range(0..self.width);
                    let y = rng.gen_range(0..self.height);
                    if elevation[x][y] > sea_level
                        && preferred_biomes.contains(&biomes[x][y])
                        && !civ_seeds.iter().any(|(cx,cy,_,_)| (*cx as isize - x as isize).abs() < 10 && (*cy as isize - y as isize).abs() < 10)
                    {
                        let culture = Culture {
                            alignment: *alignments.choose(&mut rng).unwrap(),
                            tradition: traditions.choose(&mut rng).unwrap().to_string(),
                            religion: religions.choose(&mut rng).unwrap().to_string(),
                            trait_: *traits.choose(&mut rng).unwrap(),
                        };
                        let instance_id = civ_seeds.len();
                        civ_seeds.push((x, y, CivilizationInstance { civ_type: civ, culture }, instance_id));
                        found = true;
                        break;
                    }
                }
                // Fallback: any land tile if no preferred biome found
                if !found {
                    for _ in 0..100 {
                        let x = rng.gen_range(0..self.width);
                        let y = rng.gen_range(0..self.height);
                        if elevation[x][y] > sea_level
                            && !civ_seeds.iter().any(|(cx,cy,_,_)| (*cx as isize - x as isize).abs() < 10 && (*cy as isize - y as isize).abs() < 10)
                        {
                            let culture = Culture {
                                alignment: *alignments.choose(&mut rng).unwrap(),
                                tradition: traditions.choose(&mut rng).unwrap().to_string(),
                                religion: religions.choose(&mut rng).unwrap().to_string(),
                                trait_: *traits.choose(&mut rng).unwrap(),
                            };
                            let instance_id = civ_seeds.len();
                            civ_seeds.push((x, y, CivilizationInstance { civ_type: civ, culture }, instance_id));
                            break;
                        }
                    }
                }
            }
        }
        // Assign each tile to the nearest civ seed within a radius (clustered influence)
        let influence_radius = 20.0; // adjust as desired
        let mut civilization_map = vec![vec![None; self.height]; self.width];
        for x in 0..self.width {
            for y in 0..self.height {
                if elevation[x][y] > sea_level {
                    let mut min_dist = f64::MAX;
                    let mut nearest = None;
                    for (cx, cy, civ, _instance_id) in &civ_seeds {
                        let dist = (((*cx as isize - x as isize).pow(2) + (*cy as isize - y as isize).pow(2)) as f64).sqrt();
                        if dist < min_dist {
                            min_dist = dist;
                            nearest = Some(civ.clone());
                        }
                    }
                    if min_dist <= influence_radius {
                        civilization_map[x][y] = nearest;
                    } else {
                        civilization_map[x][y] = None; // wilderness
                    }
                }
            }
        }
        // Place cities for each civilization instance
        let mut cities = Vec::new();
        let city_names = [
            "Aldoria", "Brighthaven", "Stonehelm", "Rivermouth", "Frostford", "Sunspire", "Shadowfen", "Goldport", "Ironhold", "Starfall", "Mistwood", "Deepmere", "Windrest", "Moonwatch", "Thundertop"
        ];
        for (seed_x, seed_y, civ_inst, instance_id) in &civ_seeds {
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
            let name = format!("{} Capital {}", match civ_inst.civ_type {
                Civilization::Human => "Human",
                Civilization::Elf => "Elf",
                Civilization::Dwarf => "Dwarf",
                Civilization::GnomeHalfling => "Gnome/Halfling",
                Civilization::OrcGoblin => "Orc/Goblin",
                Civilization::Merfolk => "Merfolk",
                Civilization::Lizardfolk => "Lizardfolk",
                Civilization::FairyFae => "Fairy/Fae",
                Civilization::Kobold => "Kobold",
            }, instance_id);
            let population = rng.gen_range(50_000..500_000);
            cities.push(City {
                name,
                civ: civ_inst.civ_type,
                x: *seed_x,
                y: *seed_y,
                population,
            });
        }
        println!("[GEN] Civilization/city placement: {:.2?}", civ_start.elapsed());

        // === Civilization Relations and Trade Routes ===
        let trade_start = Instant::now();
        use crate::world::worldmap::{Relation, CivilizationRelations, TradeRoute};
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
        for (seed_x, seed_y, civ_inst, _instance_id) in &civ_seeds {
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
        println!("[GEN] Trade routes: {:.2?}", trade_start.elapsed());

        let total_tiles = self.width * self.height;
        println!("[GEN] Total tiles generated: {}", total_tiles);
        println!("[GEN] World generation TOTAL: {:.2?}", total_start.elapsed());

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

        // Calculate elevation thresholds for hills and mountains
        let mut flat: Vec<f64> = elevation.iter().flatten().copied().collect();
        flat.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let mountain_level = flat[(flat.len() as f64 * 0.90) as usize]; // top 10% as mountains
        let hill_level = flat[(flat.len() as f64 * 0.80) as usize];     // next 10% as hills

        // Collect candidate points for hills and mountains
        let mut mountain_points = Vec::new();
        let mut hill_points = Vec::new();
        for x in 0..self.width as isize {
            for y in 0..self.height as isize {
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
                let x0 = rng.gen_range(0..self.width as isize);
                let y0 = rng.gen_range(0..self.height as isize);
                let x1 = rng.gen_range(0..self.width as isize);
                let y1 = rng.gen_range(0..self.height as isize);
                ((x0, y0), (x1, y1))
            };

            let path = self.generate_noisy_line(start.0, start.1, end.0, end.1, &mut rng);
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

    fn neighbors(width: usize, height: usize, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
        let mut n = Vec::with_capacity(8);
        for dx in -1i32..=1 {
            for dy in -1i32..=1 {
                if dx == 0 && dy == 0 { continue; }
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx >= 0 && ny >= 0 && (nx as usize) < width && (ny as usize) < height {
                    n.push((nx as usize, ny as usize));
                }
            }
        }
        n
    }

    fn heuristic(a: (usize, usize), b: (usize, usize)) -> f64 {
        let dx = a.0 as f64 - b.0 as f64;
        let dy = a.1 as f64 - b.1 as f64;
        (dx * dx + dy * dy).sqrt()
    }

    fn reconstruct_path(mut came_from: HashMap<(usize, usize), (usize, usize)>, mut current: (usize, usize)) -> Vec<(usize, usize)> {
        let mut path = vec![current];
        while let Some(&prev) = came_from.get(&current) {
            current = prev;
            path.push(current);
        }
        path.reverse();
        path
    }

    fn is_coast(elevation: &Vec<Vec<f64>>, sea_level: f64, x: usize, y: usize) -> bool {
        if elevation[x][y] > sea_level { return false; }
        for (nx, ny) in Self::neighbors(elevation.len(), elevation[0].len(), (x, y)) {
            if elevation[nx][ny] > sea_level {
                return true;
            }
        }
        false
    }

    fn is_river(rivers: &Vec<Vec<bool>>, x: usize, y: usize) -> bool {
        rivers[x][y]
    }

    // Land A* pathfinding, now with preferred road tiles
    fn astar_land_with_roads(&self, elevation: &Vec<Vec<f64>>, sea_level: f64, mountain_level: f64, start: (usize, usize), goal: (usize, usize), road_tiles: &HashSet<(usize, usize)>) -> Option<Vec<(usize, usize)>> {
        let (w, h) = (elevation.len(), elevation[0].len());
        let mut open = BinaryHeap::new();
        let mut came_from = HashMap::new();
        let mut g_score = HashMap::new();
        let mut rng = rand::thread_rng();
        g_score.insert(start, 0.0);
        open.push(Node { pos: start, cost: 0.0, est_total: Self::heuristic(start, goal) });
        while let Some(Node { pos, cost: _, est_total: _ }) = open.pop() {
            if pos == goal {
                return Some(Self::reconstruct_path(came_from, pos));
            }
            for (nx, ny) in Self::neighbors(w, h, pos) {
                if elevation[nx][ny] <= sea_level { continue; } // Only land
                let elev_from = elevation[pos.0][pos.1];
                let elev_to = elevation[nx][ny];
                let mut move_cost = 1.0;
                if elev_to > elev_from {
                    move_cost += 10.0 * (elev_to - elev_from); // Penalize uphill
                }
                // Angle penalty (steep slopes)
                let angle = (elev_to - elev_from).atan();
                if angle > 0.5 { move_cost += 2.0; }
                // Mountain penalty
                if elev_to >= mountain_level {
                    // 1 in 1000 chance to ignore penalty (rare pass)
                    if rng.gen_range(0..1000) == 0 {
                        move_cost += 10.0; // still a bit more expensive
                    } else {
                        move_cost += 100.0; // huge penalty, avoid mountains
                    }
                }
                // Prefer existing road tiles
                if road_tiles.contains(&(nx, ny)) {
                    move_cost -= 0.5; // bonus for using existing road
                }
                let tentative_g = g_score.get(&pos).unwrap() + move_cost;
                if tentative_g < *g_score.get(&(nx, ny)).unwrap_or(&f64::INFINITY) {
                    came_from.insert((nx, ny), pos);
                    g_score.insert((nx, ny), tentative_g);
                    open.push(Node { pos: (nx, ny), cost: tentative_g, est_total: tentative_g + Self::heuristic((nx, ny), goal) });
                }
            }
        }
        None
    }

    // Water A* pathfinding, now with preferred road tiles
    fn astar_water_with_roads(&self, elevation: &Vec<Vec<f64>>, rivers: &Vec<Vec<bool>>, sea_level: f64, start: (usize, usize), goal: (usize, usize), road_tiles: &HashSet<(usize, usize)>) -> Option<Vec<(usize, usize)>> {
        let (w, h) = (elevation.len(), elevation[0].len());
        let mut open = BinaryHeap::new();
        let mut came_from = HashMap::new();
        let mut g_score = HashMap::new();
        g_score.insert(start, 0.0);
        open.push(Node { pos: start, cost: 0.0, est_total: Self::heuristic(start, goal) });
        while let Some(Node { pos, cost: _, est_total: _ }) = open.pop() {
            if pos == goal {
                return Some(Self::reconstruct_path(came_from, pos));
            }
            for (nx, ny) in Self::neighbors(w, h, pos) {
                let is_sea = elevation[nx][ny] <= sea_level;
                let is_riv = rivers[nx][ny];
                if !is_sea && !is_riv { continue; }
                let mut move_cost = 1.0;
                // Prefer coast
                if is_sea && Self::is_coast(elevation, sea_level, nx, ny) { move_cost -= 0.2; }
                // Prefer river
                if is_riv { move_cost -= 0.5; }
                // Penalize open ocean
                if is_sea && !Self::is_coast(elevation, sea_level, nx, ny) { move_cost += 1.0; }
                // Prefer existing road tiles
                if road_tiles.contains(&(nx, ny)) {
                    move_cost -= 0.5;
                }
                let tentative_g = g_score.get(&pos).unwrap() + move_cost;
                if tentative_g < *g_score.get(&(nx, ny)).unwrap_or(&f64::INFINITY) {
                    came_from.insert((nx, ny), pos);
                    g_score.insert((nx, ny), tentative_g);
                    open.push(Node { pos: (nx, ny), cost: tentative_g, est_total: tentative_g + Self::heuristic((nx, ny), goal) });
                }
            }
        }
        None
    }

    // Find nearest coast/river tile from a land city
    fn nearest_water(&self, elevation: &Vec<Vec<f64>>, rivers: &Vec<Vec<bool>>, sea_level: f64, start: (usize, usize)) -> Option<(usize, usize)> {
        let (w, h) = (elevation.len(), elevation[0].len());
        let mut visited = HashSet::new();
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(start);
        visited.insert(start);
        while let Some((x, y)) = queue.pop_front() {
            let is_sea = elevation[x][y] <= sea_level;
            let is_riv = rivers[x][y];
            if is_sea || is_riv {
                return Some((x, y));
            }
            for (nx, ny) in Self::neighbors(w, h, (x, y)) {
                if !visited.contains(&(nx, ny)) {
                    visited.insert((nx, ny));
                    queue.push_back((nx, ny));
                }
            }
        }
        None
    }

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
        // Calculate mountain_level for this world
        let mut flat: Vec<f64> = elevation.iter().flatten().copied().collect();
        flat.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let mountain_level = flat[(flat.len() as f64 * 0.90) as usize]; // top 10% as mountains (or use your threshold)

        let mut trade_routes = Vec::new();
        let mut road_tiles = HashSet::new();
        // === Inter-civilization (capitals) ===
        for i in 0..civ_types.len() {
            for j in (i+1)..civ_types.len() {
                let civ_a = civ_types[i];
                let civ_b = civ_types[j];
                if let Some(crate::world::worldmap::Relation::Peace) = civ_relations.relations.get(&(civ_a, civ_b)) {
                    if let (Some(&(ax, ay)), Some(&(bx, by))) = (capitals.get(&civ_a), capitals.get(&civ_b)) {
                        // Decide route type
                        let a_land = elevation[ax][ay] > sea_level;
                        let b_land = elevation[bx][by] > sea_level;
                        let a_water = !a_land;
                        let b_water = !b_land;
                        let mut path = None;
                        if a_land && b_land {
                            path = self.astar_land_with_roads(elevation, sea_level, mountain_level, (ax, ay), (bx, by), &road_tiles);
                        } else if a_water && b_water {
                            path = self.astar_water_with_roads(elevation, rivers, sea_level, (ax, ay), (bx, by), &road_tiles);
                        } else {
                            let (land_city, water_city) = if a_land { ((ax, ay), (bx, by)) } else { ((bx, by), (ax, ay)) };
                            if let Some(water_entry) = self.nearest_water(elevation, rivers, sea_level, land_city) {
                                if let Some(water_path) = self.astar_land_with_roads(elevation, sea_level, mountain_level, land_city, water_entry, &road_tiles) {
                                    if let Some(water_exit) = self.nearest_water(elevation, rivers, sea_level, water_city) {
                                        if let Some(sea_path) = self.astar_water_with_roads(elevation, rivers, sea_level, water_entry, water_exit, &road_tiles) {
                                            if let Some(final_leg) = self.astar_land_with_roads(elevation, sea_level, mountain_level, water_exit, water_city, &road_tiles) {
                                                let mut full = water_path;
                                                full.pop();
                                                full.extend(sea_path);
                                                full.pop();
                                                full.extend(final_leg);
                                                path = Some(full);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        let path = path.unwrap_or_else(|| vec![(ax, ay), (bx, by)]);
                        for &pt in &path { road_tiles.insert(pt); }
                        trade_routes.push(crate::world::worldmap::TradeRoute {
                            from: (ax, ay),
                            to: (bx, by),
                            civ_a,
                            civ_b,
                            path,
                        });
                    }
                }
            }
        }
        // === Intra-civilization (all cities of same civ, MST) ===
        use std::collections::HashSet;
        let mut intra_civ_pairs = HashSet::new();
        for &civ in civ_types {
            let civ_cities: Vec<_> = cities.iter().filter(|c| c.civ == civ).collect();
            if civ_cities.len() < 2 { continue; }
            // Build MST using Prim's algorithm
            let mut in_tree = vec![false; civ_cities.len()];
            in_tree[0] = true;
            let mut edges = Vec::new();
            for _ in 1..civ_cities.len() {
                let mut best = None;
                let mut best_dist = f64::INFINITY;
                let mut best_from = 0;
                let mut best_to = 0;
                for i in 0..civ_cities.len() {
                    if !in_tree[i] { continue; }
                    for j in 0..civ_cities.len() {
                        if in_tree[j] { continue; }
                        let dx = civ_cities[i].x as f64 - civ_cities[j].x as f64;
                        let dy = civ_cities[i].y as f64 - civ_cities[j].y as f64;
                        let dist = (dx*dx + dy*dy).sqrt();
                        if dist < best_dist {
                            best_dist = dist;
                            best = Some((i, j));
                            best_from = i;
                            best_to = j;
                        }
                    }
                }
                if let Some((from, to)) = best {
                    edges.push((from, to));
                    in_tree[to] = true;
                }
            }
            // For each MST edge, generate a route
            for &(i, j) in &edges {
                let a = civ_cities[i];
                let b = civ_cities[j];
                // Avoid duplicate routes
                let key = if a.x < b.x || (a.x == b.x && a.y <= b.y) {
                    ((a.x, a.y), (b.x, b.y))
                } else {
                    ((b.x, b.y), (a.x, a.y))
                };
                if intra_civ_pairs.contains(&key) { continue; }
                intra_civ_pairs.insert(key);
                // Decide route type
                let a_land = elevation[a.x][a.y] > sea_level;
                let b_land = elevation[b.x][b.y] > sea_level;
                let a_water = !a_land;
                let b_water = !b_land;
                let mut path = None;
                if a_land && b_land {
                    path = self.astar_land_with_roads(elevation, sea_level, mountain_level, (a.x, a.y), (b.x, b.y), &road_tiles);
                } else if a_water && b_water {
                    path = self.astar_water_with_roads(elevation, rivers, sea_level, (a.x, a.y), (b.x, b.y), &road_tiles);
                } else {
                    let (land_city, water_city) = if a_land { ((a.x, a.y), (b.x, b.y)) } else { ((b.x, b.y), (a.x, a.y)) };
                    if let Some(water_entry) = self.nearest_water(elevation, rivers, sea_level, land_city) {
                        if let Some(water_path) = self.astar_land_with_roads(elevation, sea_level, mountain_level, land_city, water_entry, &road_tiles) {
                            if let Some(water_exit) = self.nearest_water(elevation, rivers, sea_level, water_city) {
                                if let Some(sea_path) = self.astar_water_with_roads(elevation, rivers, sea_level, water_entry, water_exit, &road_tiles) {
                                    if let Some(final_leg) = self.astar_land_with_roads(elevation, sea_level, mountain_level, water_exit, water_city, &road_tiles) {
                                        let mut full = water_path;
                                        full.pop();
                                        full.extend(sea_path);
                                        full.pop();
                                        full.extend(final_leg);
                                        path = Some(full);
                                    }
                                }
                            }
                        }
                    }
                }
                let path = path.unwrap_or_else(|| vec![(a.x, a.y), (b.x, b.y)]);
                for &pt in &path { road_tiles.insert(pt); }
                trade_routes.push(crate::world::worldmap::TradeRoute {
                    from: (a.x, a.y),
                    to: (b.x, b.y),
                    civ_a: civ,
                    civ_b: civ,
                    path,
                });
            }
        }
        trade_routes
    }
} 