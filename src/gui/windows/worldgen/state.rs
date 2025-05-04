use crate::worldgen::worldmap::params::WorldGenParams;

#[derive(Clone)]
pub struct WorldGenWindowState {
    pub params: WorldGenParams,
    pub seed: u32,
    pub width: usize,
    pub height: usize,
    pub regenerate_requested: bool,
}

impl WorldGenWindowState {
    pub fn new() -> Self {
        Self {
            params: WorldGenParams::default(),
            seed: 42,
            width: 128,
            height: 128,
            regenerate_requested: false,
        }
    }
} 