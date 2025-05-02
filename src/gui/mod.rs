use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui};
use crate::world::localmap::world::World;
use crate::world::localmap::terrain_material::TerrainMaterial;
use crate::worldgen::worldmap::generator::WorldGenParams;
use crate::game::game::RenderMode;
use crate::world::worldmap::world_map::WorldMap;
use crate::renderer::camera::Camera;
use crate::renderer::world_map_renderer::MapView;
use crate::world::worldmap::city::City;
use crate::world::worldmap::civilization::Civilization;
pub mod city_info_window;
pub use city_info_window::city_info_window_at;
pub mod civ_portraits;

pub struct GuiState {
    pub show_ui: bool,
    pub paused: bool,
    pub dig_jobs: usize,
    pub worldgen_params: WorldGenParams,
    pub regenerate_requested: bool,
    pub worldgen_seed: u32,
    pub map_view: MapView,
    pub worldgen_width: usize,
    pub worldgen_height: usize,
    pub selected_city: Option<City>,
    pub show_city_info: bool,
    pub selected_civ: Option<Civilization>,
}

impl GuiState {
    pub fn new() -> Self {
        Self {
            show_ui: true,
            paused: false,
            dig_jobs: 0,
            worldgen_params: WorldGenParams::default(),
            regenerate_requested: false,
            worldgen_seed: 42,
            map_view: MapView::Biome,
            worldgen_width: 128,
            worldgen_height: 128,
            selected_city: None,
            show_city_info: false,
            selected_civ: None,
        }
    }

    pub fn update(&mut self, world: &World, render_mode: RenderMode, world_map: Option<&WorldMap>, world_map_camera: Option<&Camera>, portraits: Option<&crate::gui::civ_portraits::CivPortraits>) {
        if is_key_pressed(KeyCode::Tab) {
            self.show_ui = !self.show_ui;
        }

        // --- City click detection (WorldMap mode only) ---
        if let (RenderMode::WorldMap, Some(world_map), Some(camera)) = (render_mode, world_map, world_map_camera) {
            let mouse = mouse_position();
            let cell_size = 8.0 * camera.zoom;
            let wx = camera.x + mouse.0 / cell_size;
            let wy = camera.y + mouse.1 / cell_size;
            let tx = wx.floor() as isize;
            let ty = wy.floor() as isize;
            if is_mouse_button_pressed(MouseButton::Left) {
                if tx >= 0 && ty >= 0 && (tx as usize) < world_map.width && (ty as usize) < world_map.height {
                    // Check if a city is at this tile
                    if let Some(city) = world_map.cities.iter().find(|c| c.x == tx as usize && c.y == ty as usize) {
                        self.selected_city = Some(city.clone());
                        self.show_city_info = true;
                        self.selected_civ = Some(city.civ);
                    }
                }
            }
        }

        if self.show_city_info {
            if let (Some(city), Some(portraits), Some(world_map)) = (&self.selected_city, portraits, world_map) {
                // Center the city info window below the civ portrait
                let win_width = 420.0;
                let win_height = 340.0;
                let portrait_size = 96.0;
                let portrait_y = 16.0;
                let gap = 24.0;
                let win_x = (screen_width() - win_width) / 2.0;
                let win_y = portrait_y + portrait_size + gap;
                city_info_window::city_info_window_at(city, &mut self.show_city_info, portraits, vec2(win_x, win_y), vec2(win_width, win_height), world_map);
                if !self.show_city_info {
                    self.selected_city = None;
                }
            } else if self.selected_city.is_some() && portraits.is_none() {
                // Optionally, show a loading message
                let win_width = 420.0;
                let win_height = 80.0;
                let portrait_size = 96.0;
                let portrait_y = 16.0;
                let gap = 24.0;
                let win_x = (screen_width() - win_width) / 2.0;
                let win_y = portrait_y + portrait_size + gap;
                root_ui().window(hash!("city_info_loading"), vec2(win_x, win_y), vec2(win_width, win_height), |ui| {
                    ui.label(None, "Loading civilization portraits...");
                });
            } else {
                self.show_city_info = false;
            }
        }

        if self.show_ui {
            root_ui().window(hash!(), vec2(20.0, 20.0), vec2(250.0, 200.0), |ui| {
                ui.label(None, "Voxel Game Debug UI");

                if ui.button(None, if self.paused { "Resume" } else { "Pause" }) {
                    self.paused = !self.paused;
                }

                ui.separator();
                ui.label(None, &format!("Active Dig Jobs: {}", self.dig_jobs));

                // Show block counts
                let counts = world.get_block_counts();
                ui.separator();
                ui.label(None, "Block Counts:");
                for (material, count) in counts.iter() {
                    ui.label(None, &format!("  {:?}: {}", material, count));
                }
            });

            // Show World Generation Controls only in WorldMap mode
            if let RenderMode::WorldMap = render_mode {
                root_ui().window(hash!("worldgen"), vec2(300.0, 20.0), vec2(320.0, 500.0), |ui| {
                    ui.label(None, "World Generation");
                    ui.separator();
                    // Seed controls
                    ui.label(None, &format!("Seed: {}", self.worldgen_seed));
                    if ui.button(None, "Randomize Seed") {
                        self.worldgen_seed = macroquad::rand::rand();
                    }
                    ui.separator();
                    // --- Map size controls ---
                    ui.label(None, "Map Width:");
                    for &w in &[128, 256, 512] {
                        let selected = self.worldgen_width == w;
                        let label = format!("{}{}", if selected { "● " } else { "○ " }, w);
                        if ui.button(None, label.as_str()) {
                            self.worldgen_width = w;
                        }
                    }
                    ui.separator();
                    ui.label(None, "Map Height:");
                    for &h in &[128, 256, 512] {
                        let selected = self.worldgen_height == h;
                        let label = format!("{}{}", if selected { "● " } else { "○ " }, h);
                        if ui.button(None, label.as_str()) {
                            self.worldgen_height = h;
                        }
                    }
                    ui.separator();
                    // ocean_percent (f64)
                    let mut ocean_percent = self.worldgen_params.ocean_percent as f32;
                    ui.slider(hash!("ocean_percent"), "Ocean %", 0.0..0.8, &mut ocean_percent);
                    self.worldgen_params.ocean_percent = ocean_percent as f64;
                    // coast_percent (f64)
                    let mut coast_percent = self.worldgen_params.coast_percent as f32;
                    ui.slider(hash!("coast_percent"), "Coast %", 0.0..0.3, &mut coast_percent);
                    self.worldgen_params.coast_percent = coast_percent as f64;
                    // mountain_percent (f64)
                    let mut mountain_percent = self.worldgen_params.mountain_percent as f32;
                    ui.slider(hash!("mountain_percent"), "Mountain %", 0.0..0.3, &mut mountain_percent);
                    self.worldgen_params.mountain_percent = mountain_percent as f64;
                    // erosion_iterations (usize)
                    let mut erosion_iterations = self.worldgen_params.erosion_iterations as f32;
                    ui.slider(hash!("erosion_iterations"), "Erosion Iterations", 0.0..100.0, &mut erosion_iterations);
                    self.worldgen_params.erosion_iterations = erosion_iterations.clamp(0.0, 100.0) as usize;
                    // river_threshold (f64)
                    let mut river_threshold = self.worldgen_params.river_threshold as f32;
                    ui.slider(hash!("river_threshold"), "River Threshold", 10.0..100.0, &mut river_threshold);
                    self.worldgen_params.river_threshold = river_threshold as f64;
                    // continent_scale (f64)
                    let mut continent_scale = self.worldgen_params.continent_scale as f32;
                    ui.slider(hash!("continent_scale"), "Continent Scale", 0.05..1.0, &mut continent_scale);
                    self.worldgen_params.continent_scale = continent_scale as f64;
                    // detail_scale (f64)
                    let mut detail_scale = self.worldgen_params.detail_scale as f32;
                    ui.slider(hash!("detail_scale"), "Detail Scale", 8.0..40.0, &mut detail_scale);
                    self.worldgen_params.detail_scale = detail_scale as f64;
                    // octaves_continent (usize)
                    let mut octaves_continent = self.worldgen_params.octaves_continent as f32;
                    ui.slider(hash!("octaves_continent"), "Octaves Continent", 3.0..12.0, &mut octaves_continent);
                    self.worldgen_params.octaves_continent = octaves_continent.clamp(3.0, 12.0) as usize;
                    // octaves_detail (usize)
                    let mut octaves_detail = self.worldgen_params.octaves_detail as f32;
                    ui.slider(hash!("octaves_detail"), "Octaves Detail", 5.0..16.0, &mut octaves_detail);
                    self.worldgen_params.octaves_detail = octaves_detail.clamp(5.0, 16.0) as usize;
                    // persistence (f64)
                    let mut persistence = self.worldgen_params.persistence as f32;
                    ui.slider(hash!("persistence"), "Persistence", 0.7..2.0, &mut persistence);
                    self.worldgen_params.persistence = persistence as f64;
                    // num_continents (usize)
                    let mut num_continents = self.worldgen_params.num_continents as f32;
                    ui.slider(hash!("num_continents"), "Num Continents", 1.0..8.0, &mut num_continents);
                    self.worldgen_params.num_continents = num_continents.clamp(1.0, 8.0) as usize;
                    // --- Map View Selection ---
                    ui.label(None, "Map View:");
                    let selected = matches!(self.map_view, MapView::Biome);
                    if ui.button(None, format!("{}{}", if selected { "● " } else { "○ " }, "Biome").as_str()) {
                        self.map_view = MapView::Biome;
                    }
                    let selected = matches!(self.map_view, MapView::Temperature);
                    if ui.button(None, format!("{}{}", if selected { "● " } else { "○ " }, "Temperature").as_str()) {
                        self.map_view = MapView::Temperature;
                    }
                    let selected = matches!(self.map_view, MapView::Vegetation);
                    if ui.button(None, format!("{}{}", if selected { "● " } else { "○ " }, "Vegetation").as_str()) {
                        self.map_view = MapView::Vegetation;
                    }
                    let selected = matches!(self.map_view, MapView::Precipitation);
                    if ui.button(None, format!("{}{}", if selected { "● " } else { "○ " }, "Precipitation").as_str()) {
                        self.map_view = MapView::Precipitation;
                    }
                    let selected = matches!(self.map_view, MapView::Elevation);
                    if ui.button(None, format!("{}{}", if selected { "● " } else { "○ " }, "Elevation").as_str()) {
                        self.map_view = MapView::Elevation;
                    }
                    let selected = matches!(self.map_view, MapView::Civilization);
                    if ui.button(None, format!("{}{}", if selected { "● " } else { "○ " }, "Civilization").as_str()) {
                        self.map_view = MapView::Civilization;
                    }
                    let selected = matches!(self.map_view, MapView::BiomeWithCivOverlay);
                    if ui.button(None, format!("{}{}", if selected { "● " } else { "○ " }, "Biome + Civ Overlay").as_str()) {
                        self.map_view = MapView::BiomeWithCivOverlay;
                    }
                    ui.separator();
                    if ui.button(None, "Regenerate World Map") {
                        self.regenerate_requested = true;
                    }
                });

                // --- World Map Tile Info Panel ---
                if let (Some(world_map), Some(camera)) = (world_map, world_map_camera) {
                    let mouse = mouse_position();
                    let cell_size = 8.0 * camera.zoom;
                    let wx = camera.x + mouse.0 / cell_size;
                    let wy = camera.y + mouse.1 / cell_size;
                    let tx = wx.floor() as isize;
                    let ty = wy.floor() as isize;
                    if tx >= 0 && ty >= 0 && (tx as usize) < world_map.width && (ty as usize) < world_map.height {
                        let x = tx as usize;
                        let y = ty as usize;
                        let biome = world_map.biomes[x][y];
                        let elevation = world_map.elevation[x][y];
                        let temp = world_map.temperature[x][y];
                        let precip = world_map.precipitation[x][y];
                        let fertility = world_map.soil_fertility[x][y];
                        let veg = world_map.vegetation[x][y];
                        let river = world_map.rivers[x][y];
                        let resource = world_map.resources[x][y];
                        root_ui().window(hash!("tileinfo"), vec2(650.0, 20.0), vec2(260.0, 350.0), |ui| {
                            ui.label(None, &format!("Tile ({}, {})", x, y));
                            ui.label(None, &format!("Biome: {:?}", biome));
                            ui.label(None, &format!("Elevation: {:.2}", elevation));
                            ui.label(None, &format!("Temperature: {:.2}", temp));
                            ui.label(None, &format!("Precipitation: {:.2}", precip));
                            ui.label(None, &format!("Soil Fertility: {:.2}", fertility));
                            ui.label(None, &format!("Vegetation: {:.2}", veg));
                            ui.label(None, &format!("River: {}", if river { "Yes" } else { "No" }));
                            if let Some(res) = resource {
                                ui.label(None, &format!("Resource: {:?}", res));
                            }
                            // Civilization info
                            if let Some(civ) = &world_map.civilization_map[x][y] {
                                use crate::world::worldmap::{Civilization, Relation};
                                ui.separator();
                                ui.label(None, &format!("Civilization: {:?}", civ.civ_type));
                                ui.label(None, &format!("  Alignment: {:?}", civ.culture.alignment));
                                ui.label(None, &format!("  Tradition: {}", civ.culture.tradition));
                                ui.label(None, &format!("  Religion: {}", civ.culture.religion));
                                ui.label(None, &format!("  Trait: {:?}", civ.culture.trait_));
                                // Show relations to other civs
                                ui.separator();
                                ui.label(None, "Relations:");
                                for other in [
                                    Civilization::Human,
                                    Civilization::Elf,
                                    Civilization::Dwarf,
                                    Civilization::GnomeHalfling,
                                    Civilization::OrcGoblin,
                                    Civilization::Merfolk,
                                    Civilization::Lizardfolk,
                                    Civilization::FairyFae,
                                    Civilization::Kobold,
                                ] {
                                    if other != civ.civ_type {
                                        if let Some(rel) = world_map.civ_relations.relations.get(&(civ.civ_type, other)) {
                                            ui.label(None, &format!("  {:?} ↔ {:?}: {:?}", civ.civ_type, other, rel));
                                        }
                                    }
                                }
                            }
                            // City info
                            if let Some(city) = world_map.cities.iter().find(|c| c.x == x && c.y == y) {
                                ui.separator();
                                ui.label(None, &format!("City: {}", city.name));
                                ui.label(None, &format!("  Population: {}", city.population));
                            }
                        });
                    }
                }
            }
        }
    }
}
