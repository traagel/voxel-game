use macroquad::prelude::*;
use macroquad::prelude::load_string;
use serde::Deserialize;
use crate::renderer::camera::Camera;
use crate::world::worldmap::world_map::WorldMap;
use crate::world::worldmap::biome::{BiomeId, TemperatureType, VegetationType, PrecipitationType, ElevationType};
use std::collections::HashMap;

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
struct SpriteInfo {
    filename: String,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

type BiomeSpriteMap = HashMap<String, SpriteInfo>;

fn biome_id_to_sprite_key(biome: BiomeId) -> Option<&'static str> {
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
        let tile_padding = 0.0_f32; // No distance between tiles, removes grid lines
        let draw_size = (cell_size - tile_padding).max(1.0); // Ensure at least 1px size
        let offset = tile_padding / 2.0;
        for x in 0..world_map.width {
            for y in 0..world_map.height {
                let sx = (x as f32 - camera.x) * cell_size + offset;
                let sy = (y as f32 - camera.y) * cell_size + offset;
                match view {
                    MapView::Biome => {
                        let biome = world_map.biomes[x][y];
                        if let Some(key) = biome_id_to_sprite_key(biome) {
                            if let Some(sprite) = self.biome_sprite_map.get(key) {
                                if let Some(tex) = self.biome_textures.get(&sprite.filename) {
                                    // Always use the full sprite region and scale it to fit the grid cell, preserving aspect ratio (fit in box)
                                    let src = Rect::new(sprite.x as f32, sprite.y as f32, sprite.width as f32, sprite.height as f32);
                                    let sprite_w = sprite.width as f32;
                                    let sprite_h = sprite.height as f32;
                                    let scale = (draw_size / sprite_w).min(draw_size / sprite_h);
                                    let dest_w = sprite_w * scale;
                                    let dest_h = sprite_h * scale;
                                    let dest_x = sx + (draw_size - dest_w) / 2.0;
                                    let dest_y = sy + (draw_size - dest_h) / 2.0;
                                    draw_texture_ex(
                                        tex,
                                        dest_x,
                                        dest_y,
                                        WHITE,
                                        DrawTextureParams {
                                            dest_size: Some(Vec2::new(dest_w, dest_h)),
                                            source: Some(src),
                                            ..Default::default()
                                        },
                                    );
                                    continue;
                                }
                            }
                        }
                        // Fallback: draw color
                        let color = biome.color();
                        draw_rectangle(sx, sy, draw_size, draw_size, color);
                    },
                    MapView::Temperature => {
                        let color = world_map.temperature_map[x][y].color();
                        draw_rectangle(sx, sy, draw_size, draw_size, color);
                    },
                    MapView::Vegetation => {
                        let color = world_map.vegetation_map[x][y].color();
                        draw_rectangle(sx, sy, draw_size, draw_size, color);
                    },
                    MapView::Precipitation => {
                        let color = world_map.precipitation_map[x][y].color();
                        draw_rectangle(sx, sy, draw_size, draw_size, color);
                    },
                    MapView::Elevation => {
                        let color = world_map.elevation_map[x][y].color();
                        draw_rectangle(sx, sy, draw_size, draw_size, color);
                    },
                    MapView::Civilization => {
                        let color = if let Some(civ_inst) = &world_map.civilization_map[x][y] {
                            civ_inst.civ_type.color()
                        } else {
                            DARKGRAY
                        };
                        draw_rectangle(sx, sy, draw_size, draw_size, color);
                    },
                    MapView::BiomeWithCivOverlay => {
                        let base = world_map.biomes[x][y].color();
                        draw_rectangle(sx, sy, draw_size, draw_size, base);
                        if let Some(civ_inst) = &world_map.civilization_map[x][y] {
                            let mut civ_color = civ_inst.civ_type.color();
                            civ_color.a = 0.4;
                            draw_rectangle(sx, sy, draw_size, draw_size, civ_color);
                        }
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
        self.draw_trade_routes(world_map, camera, cell_size, sea_level);
    }

    fn draw_trade_routes(&self, world_map: &WorldMap, camera: &Camera, cell_size: f32, sea_level: f64) {
        for route in &world_map.trade_routes {
            if route.path.len() < 2 { continue; }
            // Determine if this is a land or sea route by checking elevation at endpoints
            let (start_x, start_y) = route.path[0];
            let (end_x, end_y) = *route.path.last().unwrap();
            let elev_start = world_map.elevation[start_x][start_y];
            let elev_end = world_map.elevation[end_x][end_y];
            let is_land = elev_start > sea_level && elev_end > sea_level;
            for w in route.path.windows(2) {
                let (ax, ay) = w[0];
                let (bx, by) = w[1];
                let sx1 = (ax as f32 - camera.x) * cell_size + cell_size / 2.0;
                let sy1 = (ay as f32 - camera.y) * cell_size + cell_size / 2.0;
                let sx2 = (bx as f32 - camera.x) * cell_size + cell_size / 2.0;
                let sy2 = (by as f32 - camera.y) * cell_size + cell_size / 2.0;
                if is_land {
                    // Land route: thick black outline, thinner brown road
                    draw_line(sx1, sy1, sx2, sy2, cell_size * 0.32, BLACK); // outline
                    draw_line(sx1, sy1, sx2, sy2, cell_size * 0.20, BROWN); // road
                } else {
                    // Sea route: dashed purple line
                    let segments = 4;
                    for i in 0..segments {
                        let t0 = i as f32 / segments as f32;
                        let t1 = (i as f32 + 0.5) / segments as f32;
                        let x0 = sx1 + (sx2 - sx1) * t0;
                        let y0 = sy1 + (sy2 - sy1) * t0;
                        let x1 = sx1 + (sx2 - sx1) * t1;
                        let y1 = sy1 + (sy2 - sy1) * t1;
                        draw_line(x0, y0, x1, y1, cell_size * 0.16, PURPLE);
                    }
                }
            }
        }
    }
} 