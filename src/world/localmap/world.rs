use crate::world::localmap::subpixel::Subpixel;
use crate::world::localmap::terrain_material::TerrainMaterial;
use crate::world::localmap::zlevel::ZLevel;
use std::collections::HashMap;

pub struct World {
    pub z_levels: Vec<ZLevel>,
    pub block_counts: HashMap<TerrainMaterial, usize>,
}

impl World {
    pub fn new() -> Self {
        Self {
            z_levels: vec![ZLevel::new(0)],
            block_counts: HashMap::new(),
        }
    }

    pub fn get_material_at(&self, x: i32, y: i32) -> TerrainMaterial {
        if let Some(zlevel) = self.z_levels.get(0) {
            let (chunk_x, chunk_y) = (x.div_euclid(32 * 8), y.div_euclid(32 * 8));
            if let Some(chunk) = zlevel.chunks.get(&(chunk_x, chunk_y)) {
                let local_x = x.rem_euclid(32 * 8);
                let local_y = y.rem_euclid(32 * 8);

                let tile_x = local_x.div_euclid(8);
                let tile_y = local_y.div_euclid(8);

                let sub_x = local_x.rem_euclid(8);
                let sub_y = local_y.rem_euclid(8);

                if let Some(tile) = chunk
                    .tiles
                    .get(tile_x as usize)
                    .and_then(|row| row.get(tile_y as usize))
                {
                    let subpixel = &tile.subgrid[sub_x as usize][sub_y as usize];
                    return subpixel.material;
                }
            }
        }
        TerrainMaterial::Air
    }

    pub fn set_material_at(&mut self, x: i32, y: i32, material: TerrainMaterial) {
        if let Some(zlevel) = self.z_levels.get_mut(0) {
            let (chunk_x, chunk_y) = (x.div_euclid(32 * 8), y.div_euclid(32 * 8));
            if let Some(chunk) = zlevel.chunks.get_mut(&(chunk_x, chunk_y)) {
                let local_x = x.rem_euclid(32 * 8);
                let local_y = y.rem_euclid(32 * 8);

                let tile_x = local_x.div_euclid(8);
                let tile_y = local_y.div_euclid(8);

                let sub_x = local_x.rem_euclid(8);
                let sub_y = local_y.rem_euclid(8);

                if let Some(tile) = chunk
                    .tiles
                    .get_mut(tile_x as usize)
                    .and_then(|row| row.get_mut(tile_y as usize))
                {
                    let subpixel = &mut tile.subgrid[sub_x as usize][sub_y as usize];
                    let old_material = subpixel.material;
                    if old_material != material {
                        // Update block_counts
                        *self.block_counts.entry(old_material).or_insert(1) -= 1;
                        *self.block_counts.entry(material).or_insert(0) += 1;
                    }
                    subpixel.material = material;
                    chunk.dirty = true; // Mark chunk dirty if you want later to re-render it
                    tile.dirty = false;
                }
            }
        }
    }

    pub fn get_subpixel_mut(&mut self, x: i32, y: i32) -> Option<&mut Subpixel> {
        if let Some(zlevel) = self.z_levels.get_mut(0) {
            let chunk_x = x.div_euclid(32 * 8);
            let chunk_y = y.div_euclid(32 * 8);
            let local_x = x.rem_euclid(32 * 8);
            let local_y = y.rem_euclid(32 * 8);

            let tile_x = local_x.div_euclid(8);
            let tile_y = local_y.div_euclid(8);
            let sub_x = local_x.rem_euclid(8);
            let sub_y = local_y.rem_euclid(8);

            if let Some(chunk) = zlevel.chunks.get_mut(&(chunk_x, chunk_y)) {
                if let Some(tile) = chunk
                    .tiles
                    .get_mut(tile_x as usize)
                    .and_then(|row| row.get_mut(tile_y as usize))
                {
                    return Some(&mut tile.subgrid[sub_x as usize][sub_y as usize]);
                }
            }
        }
        None
    }

    pub fn get_block_counts(&self) -> &HashMap<TerrainMaterial, usize> {
        &self.block_counts
    }
}
