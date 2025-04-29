// use crate::world::chunk::Chunk;
use crate::world::terrain_material::TerrainMaterial;
use crate::world::world::World;
use macroquad::prelude::*;

pub struct Renderer {
    pub camera_x: f32,
    pub camera_y: f32,
    pub zoom: f32,
}

impl Renderer {
    pub fn default() -> Self {
        Self {
            camera_x: 0.0,
            camera_y: 0.0,
            zoom: 4.0,
        }
    }

    pub fn move_camera_delta(&mut self, dx: f32, dy: f32) {
        self.camera_x += dx;
        self.camera_y += dy;
    }

    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom = zoom
    }

    pub fn get_zoom(&self) -> f32 {
        self.zoom
    }

    pub fn delta_zoom(&mut self, zoom: f32) {
        self.zoom += zoom
    }

    pub fn render(&mut self, world: &mut World) {
        clear_background(BLACK);

        for (z_index, zlevel) in world.z_levels.iter_mut().enumerate() {
            if z_index != 0 {
                continue;
            }

            let screen_width_px = screen_width();
            let screen_height_px = screen_height();

            let screen_world_w = screen_width_px / self.zoom;
            let screen_world_h = screen_height_px / self.zoom;

            let world_left = self.camera_x.floor() as i32;
            let world_top = self.camera_y.floor() as i32;
            let world_right = (self.camera_x + screen_world_w).ceil() as i32;
            let world_bottom = (self.camera_y + screen_world_h).ceil() as i32;

            for ((chunk_x, chunk_y), chunk) in &mut zlevel.chunks {
                let chunk_pixel_x = chunk_x * 32 * 8;
                let chunk_pixel_y = chunk_y * 32 * 8;

                // ===== Chunk Culling =====
                if chunk_pixel_x > world_right || chunk_pixel_x + (32 * 8) < world_left {
                    continue;
                }
                if chunk_pixel_y > world_bottom || chunk_pixel_y + (32 * 8) < world_top {
                    continue;
                }

                for tile_x in 0..chunk.tiles.len() {
                    for tile_y in 0..chunk.tiles[tile_x].len() {
                        let tile = &chunk.tiles[tile_x][tile_y];

                        let world_x = (chunk_x * 32 + tile_x as i32) * 8;
                        let world_y = (chunk_y * 32 + tile_y as i32) * 8;

                        // Tile culling
                        if world_x + 8 < world_left
                            || world_x > world_right
                            || world_y + 8 < world_top
                            || world_y > world_bottom
                        {
                            continue;
                        }

                        let screen_x = (world_x as f32 - self.camera_x) * self.zoom;
                        let screen_y = (world_y as f32 - self.camera_y) * self.zoom;

                        if !tile.dirty {
                            // Fast path: draw solid tile
                            let fallback_color = match tile.subgrid[0][0].material {
                                TerrainMaterial::Dirt => BROWN,
                                TerrainMaterial::Rock => GRAY,
                                TerrainMaterial::Water => BLUE,
                                _ => DARKGRAY,
                            };

                            draw_rectangle(
                                screen_x,
                                screen_y,
                                8.0 * self.zoom,
                                8.0 * self.zoom,
                                fallback_color,
                            );

                            continue;
                        }

                        // Slow path: per-subpixel render
                        for sub_x in 0..tile.subgrid.len() {
                            for sub_y in 0..tile.subgrid[sub_x].len() {
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

                                let sx = (pixel_x as f32 - self.camera_x) * self.zoom;
                                let sy = (pixel_y as f32 - self.camera_y) * self.zoom;

                                let color = match subpixel.material {
                                    TerrainMaterial::Dirt => BROWN,
                                    TerrainMaterial::Rock => GRAY,
                                    TerrainMaterial::Water => BLUE,
                                    _ => WHITE,
                                };

                                draw_rectangle(sx, sy, self.zoom, self.zoom, color);

                                if subpixel.dig_target {
                                    let overlay = Color::new(1.0, 0.0, 0.0, 0.4);
                                    draw_rectangle(sx, sy, self.zoom, self.zoom, overlay);
                                }
                            }
                        }

                        // Clear dirty flag after rendering
                        chunk.tiles[tile_x][tile_y].dirty = false;
                    }
                }
            }

            // ===== Draw the grid (after terrain) =====
            let tile_size = 8;
            let grid_color = Color::new(1.0, 1.0, 1.0, 0.1);

            for x in (world_left..world_right).step_by(tile_size) {
                let screen_x = (x as f32 - self.camera_x) * self.zoom;
                draw_line(screen_x, 0.0, screen_x, screen_height_px, 1.0, grid_color);
            }
            for y in (world_top..world_bottom).step_by(tile_size) {
                let screen_y = (y as f32 - self.camera_y) * self.zoom;
                draw_line(0.0, screen_y, screen_width_px, screen_y, 1.0, grid_color);
            }
        }
    }
}
