use macroquad::prelude::*;
use crate::renderer::camera::Camera;
use crate::world::worldmap::world_map::WorldMap;
use crate::world::worldmap::biome::{BiomeId, TemperatureType, VegetationType, PrecipitationType, ElevationType};

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

pub struct WorldMapRenderer;

impl WorldMapRenderer {
    pub fn draw_world_map(&self, world_map: &WorldMap, camera: &Camera) {
        // Default: pass sea_level as 0.0 (for compatibility)
        self.draw_world_map_with_view(world_map, camera, MapView::Biome, 0.0);
    }

    pub fn draw_world_map_with_view(&self, world_map: &WorldMap, camera: &Camera, view: MapView, sea_level: f64) {
        clear_background(BLACK);
        let cell_size = 8.0 * camera.zoom;
        for x in 0..world_map.width {
            for y in 0..world_map.height {
                let sx = (x as f32 - camera.x) * cell_size;
                let sy = (y as f32 - camera.y) * cell_size;
                match view {
                    MapView::Biome => {
                        let color = world_map.biomes[x][y].color();
                        draw_rectangle(sx, sy, cell_size, cell_size, color);
                    },
                    MapView::Temperature => {
                        let color = world_map.temperature_map[x][y].color();
                        draw_rectangle(sx, sy, cell_size, cell_size, color);
                    },
                    MapView::Vegetation => {
                        let color = world_map.vegetation_map[x][y].color();
                        draw_rectangle(sx, sy, cell_size, cell_size, color);
                    },
                    MapView::Precipitation => {
                        let color = world_map.precipitation_map[x][y].color();
                        draw_rectangle(sx, sy, cell_size, cell_size, color);
                    },
                    MapView::Elevation => {
                        let color = world_map.elevation_map[x][y].color();
                        draw_rectangle(sx, sy, cell_size, cell_size, color);
                    },
                    MapView::Civilization => {
                        let color = if let Some(civ_inst) = &world_map.civilization_map[x][y] {
                            civ_inst.civ_type.color()
                        } else {
                            DARKGRAY
                        };
                        draw_rectangle(sx, sy, cell_size, cell_size, color);
                    },
                    MapView::BiomeWithCivOverlay => {
                        let base = world_map.biomes[x][y].color();
                        draw_rectangle(sx, sy, cell_size, cell_size, base);
                        if let Some(civ_inst) = &world_map.civilization_map[x][y] {
                            let mut civ_color = civ_inst.civ_type.color();
                            civ_color.a = 0.4;
                            draw_rectangle(sx, sy, cell_size, cell_size, civ_color);
                        }
                    }
                }
            }
        }
        // Draw cities as white circles
        for city in &world_map.cities {
            let sx = (city.x as f32 - camera.x) * cell_size;
            let sy = (city.y as f32 - camera.y) * cell_size;
            draw_circle(sx + cell_size / 2.0, sy + cell_size / 2.0, cell_size * 0.4, WHITE);
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
                    // Land route: solid road (brown)
                    draw_line(sx1, sy1, sx2, sy2, cell_size * 0.18, BROWN);
                } else {
                    // Sea route: dashed blue line
                    let segments = 4;
                    for i in 0..segments {
                        let t0 = i as f32 / segments as f32;
                        let t1 = (i as f32 + 0.5) / segments as f32;
                        let x0 = sx1 + (sx2 - sx1) * t0;
                        let y0 = sy1 + (sy2 - sy1) * t0;
                        let x1 = sx1 + (sx2 - sx1) * t1;
                        let y1 = sy1 + (sy2 - sy1) * t1;
                        draw_line(x0, y0, x1, y1, cell_size * 0.14, SKYBLUE);
                    }
                }
            }
        }
    }
} 