use bevy_ecs::prelude::*;
use crate::worldgen::worldmap::params::WorldGenParams;

#[derive(Resource, Default)]
pub struct WorldGenWindowStateRes {
    pub params: WorldGenParams,
    pub seed: u32,
    pub width: usize,
    pub height: usize,
    pub regenerate_requested: bool,
    pub show: bool,
}

impl WorldGenWindowStateRes {
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
    
    pub fn is_visible(&self) -> bool { self.show }
    pub fn show(&mut self) { self.show = true; }
    pub fn hide(&mut self) { self.show = false; }
    pub fn toggle(&mut self) { self.show = !self.show; }
    
    pub fn to_settings(&self) -> WorldGenSettings {
        WorldGenSettings {
            params: self.params.clone(),
            seed: self.seed,
            width: self.width,
            height: self.height,
        }
    }
}

#[derive(Clone, Default)]
pub struct WorldGenSettings {
    pub params: WorldGenParams,
    pub seed: u32,
    pub width: usize,
    pub height: usize,
} 