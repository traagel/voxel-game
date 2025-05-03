use crate::world::worldmap::{CivilizationInstance, City, CivilizationRelations, TradeRoute, Civilization, Relation};
use std::collections::{HashMap, HashSet};
pub use super::roads;

pub fn generate_trade_routes(
    civ_seeds: &Vec<(usize, usize, CivilizationInstance, usize)>,
    cities: &Vec<City>,
    elevation: &Vec<Vec<f64>>,
    rivers: &Vec<Vec<bool>>,
    sea_level: f64,
    relations: &CivilizationRelations,
) -> Vec<TradeRoute> {
    let mut trade_routes = Vec::new();
    let mut road_tiles = HashSet::new();
    let width = elevation.len();
    let height = elevation[0].len();
    // Collect all civ types and capitals
    let mut civ_types = Vec::new();
    let mut capitals = HashMap::new();
    for (x, y, civ_inst, _) in civ_seeds {
        if !civ_types.contains(&civ_inst.civ_type) {
            civ_types.push(civ_inst.civ_type);
            capitals.insert(civ_inst.civ_type, (*x, *y));
        }
    }
    // Calculate mountain_level for this world
    let mut flat: Vec<f64> = elevation.iter().flatten().copied().collect();
    flat.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mountain_level = flat[(flat.len() as f64 * 0.90) as usize];
    // === Inter-civilization (capitals) ===
    for i in 0..civ_types.len() {
        for j in (i+1)..civ_types.len() {
            let civ_a = civ_types[i];
            let civ_b = civ_types[j];
            if let Some(Relation::Peace) = relations.relations.get(&(civ_a, civ_b)) {
                if let (Some(&(ax, ay)), Some(&(bx, by))) = (capitals.get(&civ_a), capitals.get(&civ_b)) {
                    // Decide route type
                    let a_land = elevation[ax][ay] > sea_level;
                    let b_land = elevation[bx][by] > sea_level;
                    let a_water = !a_land;
                    let b_water = !b_land;
                    let mut path = None;
                    if a_land && b_land {
                        path = roads::astar_land_with_roads(elevation, sea_level, mountain_level, (ax, ay), (bx, by), &road_tiles);
                    } else if a_water && b_water {
                        path = roads::astar_water_with_roads(elevation, rivers, sea_level, (ax, ay), (bx, by), &road_tiles);
                    } else {
                        let (land_city, water_city) = if a_land { ((ax, ay), (bx, by)) } else { ((bx, by), (ax, ay)) };
                        if let Some(water_entry) = roads::nearest_water(elevation, rivers, sea_level, land_city) {
                            if let Some(water_path) = roads::astar_land_with_roads(elevation, sea_level, mountain_level, land_city, water_entry, &road_tiles) {
                                if let Some(water_exit) = roads::nearest_water(elevation, rivers, sea_level, water_city) {
                                    if let Some(sea_path) = roads::astar_water_with_roads(elevation, rivers, sea_level, water_entry, water_exit, &road_tiles) {
                                        if let Some(final_leg) = roads::astar_land_with_roads(elevation, sea_level, mountain_level, water_exit, water_city, &road_tiles) {
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
                    trade_routes.push(TradeRoute {
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
    let mut intra_civ_pairs = HashSet::new();
    for &civ in &civ_types {
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
                path = roads::astar_land_with_roads(elevation, sea_level, mountain_level, (a.x, a.y), (b.x, b.y), &road_tiles);
            } else if a_water && b_water {
                path = roads::astar_water_with_roads(elevation, rivers, sea_level, (a.x, a.y), (b.x, b.y), &road_tiles);
            } else {
                let (land_city, water_city) = if a_land { ((a.x, a.y), (b.x, b.y)) } else { ((b.x, b.y), (a.x, a.y)) };
                if let Some(water_entry) = roads::nearest_water(elevation, rivers, sea_level, land_city) {
                    if let Some(water_path) = roads::astar_land_with_roads(elevation, sea_level, mountain_level, land_city, water_entry, &road_tiles) {
                        if let Some(water_exit) = roads::nearest_water(elevation, rivers, sea_level, water_city) {
                            if let Some(sea_path) = roads::astar_water_with_roads(elevation, rivers, sea_level, water_entry, water_exit, &road_tiles) {
                                if let Some(final_leg) = roads::astar_land_with_roads(elevation, sea_level, mountain_level, water_exit, water_city, &road_tiles) {
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
            trade_routes.push(TradeRoute {
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
