use macroquad::prelude::*;

pub fn draw_grid(
    camera_x: f32,
    camera_y: f32,
    zoom: f32,
    screen_width_px: f32,
    screen_height_px: f32,
    tile_size: i32,
) {
    let grid_color = Color::new(1.0, 1.0, 1.0, 0.1);
    let world_left = camera_x.floor() as i32;
    let world_top = camera_y.floor() as i32;
    let world_right = (camera_x + screen_width_px / zoom).ceil() as i32;
    let world_bottom = (camera_y + screen_height_px / zoom).ceil() as i32;

    let grid_start_x = (world_left / tile_size) * tile_size;
    let grid_end_x = (world_right / tile_size + 1) * tile_size;
    let grid_start_y = (world_top / tile_size) * tile_size;
    let grid_end_y = (world_bottom / tile_size + 1) * tile_size;

    for x in (grid_start_x..grid_end_x).step_by(tile_size as usize) {
        let screen_x = (x as f32 - camera_x) * zoom;
        draw_line(screen_x, 0.0, screen_x, screen_height_px, 1.0, grid_color);
    }
    for y in (grid_start_y..grid_end_y).step_by(tile_size as usize) {
        let screen_y = (y as f32 - camera_y) * zoom;
        draw_line(0.0, screen_y, screen_width_px, screen_y, 1.0, grid_color);
    }
} 