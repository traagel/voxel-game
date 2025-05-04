use macroquad::prelude::*;
use crate::world::worldmap::world_map::WorldMap;

pub fn draw_precipitation_tile(world_map: &WorldMap, x: usize, y: usize, sx: f32, sy: f32, draw_size: f32) {
    let color = world_map.precipitation_map[x][y].color();
    draw_rectangle(sx, sy, draw_size, draw_size, color);
} 