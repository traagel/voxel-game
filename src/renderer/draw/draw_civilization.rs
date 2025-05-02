use macroquad::prelude::*;
use crate::world::worldmap::world_map::WorldMap;

pub fn draw_civilization_tile(world_map: &WorldMap, x: usize, y: usize, sx: f32, sy: f32, draw_size: f32) {
    let color = if let Some(civ_inst) = &world_map.civilization_map[x][y] {
        civ_inst.civ_type.color()
    } else {
        DARKGRAY
    };
    draw_rectangle(sx, sy, draw_size, draw_size, color);
} 