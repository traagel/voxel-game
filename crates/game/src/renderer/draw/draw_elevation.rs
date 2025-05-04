use macroquad::prelude::*;
use crate::world::worldmap::world_map::WorldMap;

fn elevation_gradient_color(level: usize) -> Color {
    // Blue (low) to White (high)
    let t = level as f32 / 9.0;
    Color::new(
        t + (1.0 - t) * 0.0, // R: 0.0 to 1.0
        t + (1.0 - t) * 0.2, // G: 0.2 to 1.0
        1.0,                 // B: always 1.0
        1.0,
    )
}

pub fn draw_elevation_tile(world_map: &WorldMap, x: usize, y: usize, sx: f32, sy: f32, draw_size: f32) {
    let elev = world_map.elevation[x][y]; // 0.0..1.0
    let level = (elev * 10.0).floor().min(9.0).max(0.0) as usize;
    let color = elevation_gradient_color(level);
    draw_rectangle(sx, sy, draw_size, draw_size, color);
} 