use macroquad::prelude::*;
use crate::world::worldmap::world_map::WorldMap;

pub fn draw_biome_with_civ_overlay_tile(world_map: &WorldMap, x: usize, y: usize, sx: f32, sy: f32, draw_size: f32) {
    let base = world_map.biomes[x][y].color();
    draw_rectangle(sx, sy, draw_size, draw_size, base);
    if let Some(civ_inst) = &world_map.civilization_map[x][y] {
        let mut civ_color = civ_inst.civ_type.color();
        civ_color.a = 0.4;
        draw_rectangle(sx, sy, draw_size, draw_size, civ_color);
    }
} 