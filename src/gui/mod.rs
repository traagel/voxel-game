use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui};
use crate::world::localmap::world::World;
use crate::world::localmap::terrain_material::TerrainMaterial;
use crate::worldgen::worldmap::params::WorldGenParams;
use crate::ecs::resources::game_view::RenderMode;
use crate::world::worldmap::world_map::WorldMap;
use crate::renderer::camera::Camera;
use crate::renderer::world_map_renderer::MapView;
use crate::world::worldmap::city::City;
use crate::world::worldmap::civilization::Civilization;

pub mod state;
pub use state::GuiState;

pub mod windows;
pub use windows::*;

pub mod widgets;
pub use widgets::*;
