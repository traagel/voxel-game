use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, BinaryHeap};
use rand::Rng;

#[derive(Copy, Clone)]
pub struct Node {
    pub pos: (usize, usize),
    pub cost: f64,
    pub est_total: f64,
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.est_total.partial_cmp(&self.est_total).unwrap_or(Ordering::Equal)
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}
impl Eq for Node {}

pub fn neighbors(width: usize, height: usize, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
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

pub fn heuristic(a: (usize, usize), b: (usize, usize)) -> f64 {
    let dx = a.0 as f64 - b.0 as f64;
    let dy = a.1 as f64 - b.1 as f64;
    (dx * dx + dy * dy).sqrt()
}

pub fn reconstruct_path(mut came_from: HashMap<(usize, usize), (usize, usize)>, mut current: (usize, usize)) -> Vec<(usize, usize)> {
    let mut path = vec![current];
    while let Some(&prev) = came_from.get(&current) {
        current = prev;
        path.push(current);
    }
    path.reverse();
    path
}

pub fn is_coast(elevation: &Vec<Vec<f64>>, sea_level: f64, x: usize, y: usize) -> bool {
    if elevation[x][y] > sea_level { return false; }
    for (nx, ny) in neighbors(elevation.len(), elevation[0].len(), (x, y)) {
        if elevation[nx][ny] > sea_level {
            return true;
        }
    }
    false
}

pub fn astar_land_with_roads(
    elevation: &Vec<Vec<f64>>,
    sea_level: f64,
    mountain_level: f64,
    start: (usize, usize),
    goal: (usize, usize),
    road_tiles: &HashSet<(usize, usize)>,
) -> Option<Vec<(usize, usize)>> {
    let (w, h) = (elevation.len(), elevation[0].len());
    let mut open = BinaryHeap::new();
    let mut came_from = HashMap::new();
    let mut g_score = HashMap::new();
    let mut rng = rand::thread_rng();
    g_score.insert(start, 0.0);
    open.push(Node { pos: start, cost: 0.0, est_total: heuristic(start, goal) });
    while let Some(Node { pos, .. }) = open.pop() {
        if pos == goal {
            return Some(reconstruct_path(came_from, pos));
        }
        for (nx, ny) in neighbors(w, h, pos) {
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
                if rng.gen_range(0..1000) == 0 {
                    move_cost += 10.0;
                } else {
                    move_cost += 100.0;
                }
            }
            // Prefer existing road tiles
            if road_tiles.contains(&(nx, ny)) {
                move_cost -= 0.5;
            }
            let tentative_g = g_score.get(&pos).unwrap() + move_cost;
            if tentative_g < *g_score.get(&(nx, ny)).unwrap_or(&f64::INFINITY) {
                came_from.insert((nx, ny), pos);
                g_score.insert((nx, ny), tentative_g);
                open.push(Node { pos: (nx, ny), cost: tentative_g, est_total: tentative_g + heuristic((nx, ny), goal) });
            }
        }
    }
    None
}

pub fn astar_water_with_roads(
    elevation: &Vec<Vec<f64>>,
    rivers: &Vec<Vec<bool>>,
    sea_level: f64,
    start: (usize, usize),
    goal: (usize, usize),
    road_tiles: &HashSet<(usize, usize)>,
) -> Option<Vec<(usize, usize)>> {
    let (w, h) = (elevation.len(), elevation[0].len());
    let mut open = BinaryHeap::new();
    let mut came_from = HashMap::new();
    let mut g_score = HashMap::new();
    g_score.insert(start, 0.0);
    open.push(Node { pos: start, cost: 0.0, est_total: heuristic(start, goal) });
    while let Some(Node { pos, .. }) = open.pop() {
        if pos == goal {
            return Some(reconstruct_path(came_from, pos));
        }
        for (nx, ny) in neighbors(w, h, pos) {
            let is_sea = elevation[nx][ny] <= sea_level;
            let is_riv = rivers[nx][ny];
            if !is_sea && !is_riv { continue; }
            let mut move_cost = 1.0;
            if is_sea && is_coast(elevation, sea_level, nx, ny) { move_cost -= 0.2; }
            if is_riv { move_cost -= 0.5; }
            if is_sea && !is_coast(elevation, sea_level, nx, ny) { move_cost += 1.0; }
            if road_tiles.contains(&(nx, ny)) {
                move_cost -= 0.5;
            }
            let tentative_g = g_score.get(&pos).unwrap() + move_cost;
            if tentative_g < *g_score.get(&(nx, ny)).unwrap_or(&f64::INFINITY) {
                came_from.insert((nx, ny), pos);
                g_score.insert((nx, ny), tentative_g);
                open.push(Node { pos: (nx, ny), cost: tentative_g, est_total: tentative_g + heuristic((nx, ny), goal) });
            }
        }
    }
    None
}

pub fn nearest_water(
    elevation: &Vec<Vec<f64>>,
    rivers: &Vec<Vec<bool>>,
    sea_level: f64,
    start: (usize, usize),
) -> Option<(usize, usize)> {
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
        for (nx, ny) in neighbors(w, h, (x, y)) {
            if !visited.contains(&(nx, ny)) {
                visited.insert((nx, ny));
                queue.push_back((nx, ny));
            }
        }
    }
    None
} 