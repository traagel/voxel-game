use macroquad::prelude::*;
use crate::world::worldmap::world_map::WorldMap;
use crate::renderer::camera::Camera;

pub fn draw_trade_routes(world_map: &WorldMap, camera: &Camera, cell_size: f32, sea_level: f64) {
    for route in &world_map.trade_routes {
        if route.path.len() < 2 { continue; }
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
                draw_line(sx1, sy1, sx2, sy2, cell_size * 0.32, BLACK);
                draw_line(sx1, sy1, sx2, sy2, cell_size * 0.20, BROWN);
            } else {
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