use crate::world::world::World;
use crate::world::terrain_material::TerrainMaterial;

/// Paints a dig target area in the world at (x, y).
pub fn paint_dig_target(world: &mut World, x: i32, y: i32) {
    let radius = 2; // Adjust if you want bigger painting area

    for dx in -radius..=radius {
        for dy in -radius..=radius {
            let px = x + dx;
            let py = y + dy;

            let (chunk_x, chunk_y) = (px.div_euclid(32 * 8), py.div_euclid(32 * 8));
            let local_x = px.rem_euclid(32 * 8);
            let local_y = py.rem_euclid(32 * 8);
            let tile_x = local_x.div_euclid(8);
            let tile_y = local_y.div_euclid(8);
            let sub_x = local_x.rem_euclid(8);
            let sub_y = local_y.rem_euclid(8);

            if let Some(zlevel) = world.z_levels.get_mut(0) {
                if let Some(chunk) = zlevel.chunks.get_mut(&(chunk_x, chunk_y)) {
                    if let Some(tile) = chunk
                        .tiles
                        .get_mut(tile_x as usize)
                        .and_then(|r| r.get_mut(tile_y as usize))
                    {
                        tile.subgrid[sub_x as usize][sub_y as usize].dig_target = true;
                        chunk.dirty = true;
                    }
                }
            }
        }
    }
}

/// Counts the number of dig jobs in the world.
pub fn count_dig_jobs(world: &World) -> usize {
    let mut count = 0;

    for z in &world.z_levels {
        for chunk in z.chunks.values() {
            for row in &chunk.tiles {
                for tile in row {
                    for subrow in &tile.subgrid {
                        for sub in subrow {
                            if sub.dig_target {
                                count += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    count
}

/// Paints rock material in a small area around (x, y).
pub fn paint_rock(world: &mut World, x: i32, y: i32) {
    let radius = 2; // Paint a small 5x5 blob around the cursor

    for dx in -radius..=radius {
        for dy in -radius..=radius {
            let px = x + dx;
            let py = y + dy;
            world.set_material_at(px, py, TerrainMaterial::Rock);
        }
    }
} 