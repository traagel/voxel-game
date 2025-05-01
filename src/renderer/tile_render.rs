use macroquad::prelude::*;
use crate::world::terrain_material::TerrainMaterial;

pub struct TileRenderer;

impl Default for TileRenderer {
    fn default() -> Self {
        TileRenderer
    }
}

impl TileRenderer {
    pub fn draw_tile(
        &self,
        tile: &crate::world::tile::Tile,
        world_x: i32,
        world_y: i32,
        camera_x: f32,
        camera_y: f32,
        zoom: f32,
        tile_size: i32,
        world_left: i32,
        world_right: i32,
        world_top: i32,
        world_bottom: i32,
    ) {
        if is_mixed_material(tile) {
            // Per-subpixel render
            let subgrid_size = tile.subgrid.len();
            for sub_x in 0..subgrid_size {
                for sub_y in 0..subgrid_size {
                    let subpixel = &tile.subgrid[sub_x][sub_y];
                    if subpixel.material == TerrainMaterial::Air {
                        continue;
                    }
                    let pixel_x = world_x + sub_x as i32;
                    let pixel_y = world_y + sub_y as i32;

                    // Subpixel culling
                    if pixel_x < world_left
                        || pixel_x > world_right
                        || pixel_y < world_top
                        || pixel_y > world_bottom
                    {
                        continue;
                    }

                    let sx = (pixel_x as f32 - camera_x) * zoom;
                    let sy = (pixel_y as f32 - camera_y) * zoom;

                    let color = match subpixel.material {
                        TerrainMaterial::Dirt => BROWN,
                        TerrainMaterial::Rock => GRAY,
                        TerrainMaterial::Water => BLUE,
                        _ => WHITE,
                    };

                    draw_rectangle(sx, sy, zoom, zoom, color);

                    if subpixel.dig_target {
                        let overlay = Color::new(1.0, 0.0, 0.0, 0.4);
                        draw_rectangle(sx, sy, zoom, zoom, overlay);
                    }
                }
            }
        } else {
            // Solid tile
            let mat = tile.subgrid[0][0].material;
            if mat == TerrainMaterial::Air {
                // Don't draw anything for air tiles
                return;
            }
            let fallback_color = match mat {
                TerrainMaterial::Dirt => BROWN,
                TerrainMaterial::Rock => GRAY,
                TerrainMaterial::Water => BLUE,
                _ => DARKGRAY,
            };

            let screen_x = (world_x as f32 - camera_x) * zoom;
            let screen_y = (world_y as f32 - camera_y) * zoom;
            draw_rectangle(
                screen_x,
                screen_y,
                tile_size as f32 * zoom,
                tile_size as f32 * zoom,
                fallback_color,
            );
        }
    }
}

fn is_mixed_material(tile: &crate::world::tile::Tile) -> bool {
    let first = tile.subgrid[0][0].material;
    for row in &tile.subgrid {
        for sub in row {
            if sub.material != first {
                return true;
            }
        }
    }
    false
} 