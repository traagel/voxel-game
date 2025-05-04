use crate::world::localmap::terrain_material::TerrainMaterial;
use crate::world::localmap::world::World;
use super::Creature;

impl Creature {
    fn is_walkable(mat: TerrainMaterial) -> bool {
        matches!(mat, TerrainMaterial::Dirt | TerrainMaterial::Air)
    }

    pub fn find_nearest_dig_target(&mut self, world: &World) {
        let search_radius = 100;
        let start_x = self.x as i32;
        let start_y = self.y as i32;
        let mut best_target: Option<(i32, i32)> = None;
        let mut best_distance = f32::MAX;
        for dx in -search_radius..=search_radius {
            for dy in -search_radius..=search_radius {
                let check_x = start_x + dx;
                let check_y = start_y + dy;
                let chunk_x = check_x.div_euclid(32 * 8);
                let chunk_y = check_y.div_euclid(32 * 8);
                let local_x = check_x.rem_euclid(32 * 8);
                let local_y = check_y.rem_euclid(32 * 8);
                let tile_x = local_x.div_euclid(8);
                let tile_y = local_y.div_euclid(8);
                let sub_x = local_x.rem_euclid(8);
                let sub_y = local_y.rem_euclid(8);
                if let Some(zlevel) = world.z_levels.get(0) {
                    if let Some(chunk) = zlevel.chunks.get(&(chunk_x, chunk_y)) {
                        if let Some(tile) = chunk
                            .tiles
                            .get(tile_x as usize)
                            .and_then(|r| r.get(tile_y as usize))
                        {
                            let subpixel = &tile.subgrid[sub_x as usize][sub_y as usize];
                            if subpixel.dig_target {
                                let dx = self.x - check_x as f32;
                                let dy = self.y - check_y as f32;
                                let dist_sq = dx * dx + dy * dy;
                                if dist_sq < best_distance {
                                    best_distance = dist_sq;
                                    best_target = Some((check_x, check_y));
                                }
                            }
                        }
                    }
                }
            }
        }
        self.target = best_target;
    }

    pub fn move_toward_target(&mut self, world: &World) {
        if let Some((target_x, target_y)) = self.target {
            let dx = target_x as f32 - self.x;
            let dy = target_y as f32 - self.y;
            let (step_x, step_y) = if dx.abs() > dy.abs() {
                (dx.signum() * 0.5, 0.0)
            } else {
                (0.0, dy.signum() * 0.5)
            };
            let next_x = self.x + step_x;
            let next_y = self.y + step_y;
            let primary_terrain = world.get_material_at(next_x as i32, next_y as i32);
            if Self::is_walkable(primary_terrain) {
                self.x = next_x;
                self.y = next_y;
            } else {
                // Try the other axis instead
                let (alt_step_x, alt_step_y) = if step_x == 0.0 {
                    (dx.signum() * 0.5, 0.0)
                } else {
                    (0.0, dy.signum() * 0.5)
                };
                let alt_x = self.x + alt_step_x;
                let alt_y = self.y + alt_step_y;
                let alt_terrain = world.get_material_at(alt_x as i32, alt_y as i32);
                if Self::is_walkable(alt_terrain) {
                    self.x = alt_x;
                    self.y = alt_y;
                } else {
                    // Fully stuck
                    self.target = None;
                }
            }
        }
    }
} 