use macroquad::prelude::*;
use macroquad::prelude::load_string;
use serde::Deserialize;
use crate::renderer::camera::Camera;
use crate::world::worldmap::world_map::WorldMap;
use crate::world::worldmap::biome::{BiomeId, TemperatureType, VegetationType, PrecipitationType, ElevationType};
use std::collections::HashMap;
use crate::renderer::draw::*;

#[derive(Copy, Clone)]
pub enum MapView {
    Biome,
    Temperature,
    Vegetation,
    Precipitation,
    Elevation,
    Civilization,
    BiomeWithCivOverlay,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SpriteInfo {
    pub filename: String,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub origin: Option<[u32; 2]>,
    pub tile_width: Option<u32>,
    pub tile_height: Option<u32>,
    pub core: Option<[u32; 4]>,
}

pub type BiomeSpriteMap = HashMap<String, SpriteInfo>;

pub fn biome_id_to_sprite_key(biome: BiomeId) -> Option<&'static str> {
    use BiomeId::*;
    match biome {
        Ocean => Some("Ocean"),
        Mountain => Some("Mountain"),
        Plains => None, // No PNG, fallback to color
        River => Some("River"),
        Sea => Some("Sea"),
        Lake => Some("Lake"),
        Hills => Some("Hills"),
        Snow => Some("Snow"),
        Forest => Some("Forest"),
        Jungle => Some("Jungle"),
        Desert => Some("Desert"),
        Savanna => None, // No PNG, fallback to color
        Tundra => Some("Tundra"),
        Taiga => Some("Taiga"),
        Swamp => Some("Swamp"),
        Beach => Some("Beach"),
        TemperateForest => Some("TemperateForest"),
        BorealForest => Some("BorealForest"),
        Rainforest => Some("Rainforest"),
    }
}

pub struct WorldMapRenderer {
    biome_textures: HashMap<String, Texture2D>,
    biome_sprite_map: BiomeSpriteMap,
}

impl WorldMapRenderer {
    pub async fn new() -> Self {
        // Load and parse the sprite map JSON
        let json_str = load_string("assets/biome_sprite_map.json").await.unwrap();
        let biome_sprite_map: BiomeSpriteMap = serde_json::from_str(&json_str).unwrap();
        // Load all unique textures referenced in the sprite map
        let mut biome_textures = HashMap::new();
        for sprite in biome_sprite_map.values() {
            if !biome_textures.contains_key(&sprite.filename) {
                let tex = load_texture(&format!("assets/{}", sprite.filename)).await.unwrap();
                tex.set_filter(FilterMode::Nearest);
                biome_textures.insert(sprite.filename.clone(), tex);
            }
        }
        Self { biome_textures, biome_sprite_map }
    }

    pub fn draw_world_map(&self, world_map: &WorldMap, camera: &Camera) {
        self.draw_world_map_with_view(world_map, camera, MapView::Biome, 0.0);
    }

    pub fn draw_world_map_with_view(&self, world_map: &WorldMap, camera: &Camera, view: MapView, sea_level: f64) {
        clear_background(BLACK);
        let cell_size = 8.0 * camera.zoom;
        let tile_padding = 0.0_f32;
        let draw_size = (cell_size - tile_padding).max(1.0);
        let offset = tile_padding / 2.0;
        for x in 0..world_map.width {
            for y in 0..world_map.height {
                let sx = (x as f32 - camera.x) * cell_size + offset;
                let sy = (y as f32 - camera.y) * cell_size + offset;
                match view {
                    MapView::Biome => {
                        draw_biome_tile(&self.biome_sprite_map, &self.biome_textures, world_map, x, y, sx, sy, draw_size, camera, offset);
                    },
                    MapView::Temperature => {
                        draw_temperature_tile(world_map, x, y, sx, sy, draw_size);
                    },
                    MapView::Vegetation => {
                        draw_vegetation_tile(world_map, x, y, sx, sy, draw_size);
                    },
                    MapView::Precipitation => {
                        draw_precipitation_tile(world_map, x, y, sx, sy, draw_size);
                    },
                    MapView::Elevation => {
                        draw_elevation_tile(world_map, x, y, sx, sy, draw_size);
                    },
                    MapView::Civilization => {
                        draw_civilization_tile(world_map, x, y, sx, sy, draw_size);
                    },
                    MapView::BiomeWithCivOverlay => {
                        draw_biome_with_civ_overlay_tile(world_map, x, y, sx, sy, draw_size);
                    }
                }
            }
        }
        // Draw cities as red circles
        for city in &world_map.cities {
            let sx = (city.x as f32 - camera.x) * cell_size + cell_size / 2.0;
            let sy = (city.y as f32 - camera.y) * cell_size + cell_size / 2.0;
            draw_circle(sx, sy, draw_size * 0.4, RED);
        }
        // Draw trade routes
        draw_trade_routes(world_map, camera, cell_size, sea_level);
    }
} 