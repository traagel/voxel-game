use crate::worldgen::worldmap::params::WorldGenParams;
use crate::gui::windows::window_state::WindowState;

#[derive(Clone, Default)]
pub struct WorldGenSettings {
    pub params: WorldGenParams,
    pub seed: u32,
    pub width: usize,
    pub height: usize,
}

impl WorldGenSettings {
    pub fn new(seed: u32, width: usize, height: usize, params: WorldGenParams) -> Self {
        Self {
            params,
            seed,
            width,
            height,
        }
    }
}

#[derive(Clone, Default)]
pub struct WorldGenWindowState {
    pub params: WorldGenParams,
    pub seed: u32,
    pub width: usize,
    pub height: usize,
    pub regenerate_requested: bool,
    pub show: bool,
}

impl WorldGenWindowState {
    pub fn new() -> Self {
        Self {
            params: WorldGenParams::default(),
            seed: 42,
            width: 128,
            height: 128,
            regenerate_requested: false,
            show: false,
        }
    }
    
    pub fn to_settings(&self) -> WorldGenSettings {
        WorldGenSettings {
            params: self.params.clone(),
            seed: self.seed,
            width: self.width,
            height: self.height,
        }
    }
}

impl WindowState for WorldGenWindowState {
    fn is_visible(&self) -> bool { self.show }
    fn show(&mut self) { self.show = true; }
    fn hide(&mut self) { self.show = false; }
    fn toggle(&mut self) { self.show = !self.show; }
} 