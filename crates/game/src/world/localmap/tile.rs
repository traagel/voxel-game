use crate::world::localmap::subpixel::Subpixel;

pub const TILE_SIZE: usize = 8;

#[derive(Clone, Copy)]
pub struct Tile {
    pub subgrid: [[Subpixel; TILE_SIZE]; TILE_SIZE],
    pub dirty: bool,
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            subgrid: [[Subpixel::default(); TILE_SIZE]; TILE_SIZE],
            dirty: false,
        }
    }
}
