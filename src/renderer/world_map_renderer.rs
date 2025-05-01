use macroquad::prelude::*;
use crate::renderer::camera::Camera;
use crate::world::worldmap::world_map::WorldMap;
use crate::world::worldmap::biome::BiomeId;

const PLAINS_COLOR: Color = GREEN;
const MOUNTAIN_COLOR: Color = GRAY;
const OCEAN_COLOR: Color = BLUE;
const SEA_COLOR: Color = SKYBLUE;
const RIVER_COLOR: Color = DARKBLUE;
const BEACH_COLOR: Color = YELLOW;
const DESERT_COLOR: Color = ORANGE;
const FOREST_COLOR: Color = DARKGREEN;
const JUNGLE_COLOR: Color = LIME;
const TUNDRA_COLOR: Color = LIGHTGRAY;
const SWAMP_COLOR: Color = DARKPURPLE;
const LAKE_COLOR: Color = SKYBLUE;
const HILLS_COLOR: Color = BROWN;
const SNOW_COLOR: Color = WHITE;
const SAVANNA_COLOR: Color = GOLD;
const TAIGA_COLOR: Color = Color::new(0.5, 0.5, 0.0, 1.0);
const DEFAULT_COLOR: Color = WHITE;

pub struct WorldMapRenderer;

impl WorldMapRenderer {
    pub fn draw_world_map(&self, world_map: &WorldMap, camera: &Camera) {
        clear_background(BLACK);
        let cell_size = 8.0 * camera.zoom;
        for x in 0..world_map.width {
            for y in 0..world_map.height {
                let biome = world_map.biomes[x][y];
                let color = match biome {
                    BiomeId::Plains => PLAINS_COLOR,
                    BiomeId::Mountain => MOUNTAIN_COLOR,
                    BiomeId::Ocean => OCEAN_COLOR,
                    BiomeId::Sea => SEA_COLOR,
                    BiomeId::River => RIVER_COLOR,
                    BiomeId::Beach => BEACH_COLOR,
                    BiomeId::Desert => DESERT_COLOR,
                    BiomeId::Forest => FOREST_COLOR,
                    BiomeId::Jungle => JUNGLE_COLOR,
                    BiomeId::Tundra => TUNDRA_COLOR,
                    BiomeId::Swamp => SWAMP_COLOR,
                    BiomeId::Lake => LAKE_COLOR,
                    BiomeId::Hills => HILLS_COLOR,
                    BiomeId::Snow => SNOW_COLOR,
                    BiomeId::Savanna => SAVANNA_COLOR,
                    BiomeId::Taiga => TAIGA_COLOR,
                    _ => DEFAULT_COLOR,
                };
                let sx = (x as f32 - camera.x) * cell_size;
                let sy = (y as f32 - camera.y) * cell_size;
                draw_rectangle(sx, sy, cell_size, cell_size, color);
            }
        }
    }
} 