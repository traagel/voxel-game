use crate::world::terrain_material::TerrainMaterial;

#[derive(Clone, Copy)]
pub struct Subpixel {
    pub material: TerrainMaterial,
    pub dig_target: bool,
}

impl Default for Subpixel {
    fn default() -> Self {
        Self {
            material: TerrainMaterial::Air,
            dig_target: false,
        }
    }
}
