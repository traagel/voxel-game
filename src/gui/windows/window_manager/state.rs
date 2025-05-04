use crate::worldgen::worldmap::params::WorldGenParams;
use crate::renderer::world_map_renderer::MapView;
use crate::world::worldmap::city::City;
use crate::world::worldmap::civilization::Civilization;

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

#[derive(Clone)]
pub struct MapViewState {
    pub mode: MapView,
}

impl MapViewState {
    pub fn new() -> Self {
        Self { mode: MapView::Biome }
    }
}

#[derive(Clone)]
pub struct CityInfoState {
    pub show: bool,
    pub selected_city: Option<City>,
    pub selected_civ: Option<Civilization>,
}

impl CityInfoState {
    pub fn new() -> Self {
        Self {
            show: false,
            selected_city: None,
            selected_civ: None,
        }
    }
} 