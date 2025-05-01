use macroquad::prelude::*;
use crate::game::game_state::GameState;
use crate::renderer::camera::Camera;
use crate::renderer::grid::draw_grid;
use crate::renderer::tile_render::TileRenderer;

pub struct LocalMapRenderer {
    pub camera: Camera,
    tile_renderer: TileRenderer,
}

impl LocalMapRenderer {
    pub fn default() -> Self {
        Self {
            camera: Camera::default(),
            tile_renderer: TileRenderer::default(),
        }
    }

    pub fn move_camera_delta(&mut self, dx: f32, dy: f32) {
        self.camera.move_delta(dx, dy);
    }

    pub fn set_zoom(&mut self, zoom: f32) {
        self.camera.set_zoom(zoom);
    }

    pub fn get_zoom(&self) -> f32 {
        self.camera.get_zoom()
    }

    pub fn delta_zoom(&mut self, zoom: f32) {
        self.camera.delta_zoom(zoom);
    }

    pub fn get_camera_x(&self) -> f32 {
        self.camera.get_x()
    }

    pub fn get_camera_y(&self) -> f32 {
        self.camera.get_y()
    }

    /// The only public rendering method: draws the world using an immutable GameState DTO.
    pub fn draw(&self, state: &GameState) {
        clear_background(BLACK);

        if let Some(zlevel) = state.z_levels.get(0) {
            let screen_width_px = screen_width();
            let screen_height_px = screen_height();
            let zoom = self.camera.zoom;
            let camera_x = self.camera.x;
            let camera_y = self.camera.y;
            let screen_world_w = screen_width_px / zoom;
            let screen_world_h = screen_height_px / zoom;
            let world_left = camera_x.floor() as i32;
            let world_top = camera_y.floor() as i32;
            let world_right = (camera_x + screen_world_w).ceil() as i32;
            let world_bottom = (camera_y + screen_world_h).ceil() as i32;
            let chunk_size = 32;
            let tile_size = 8;
            let chunk_pixel_size = chunk_size * tile_size;

            for ((chunk_x, chunk_y), chunk) in zlevel.chunks.iter() {
                self.draw_chunk(
                    chunk_x,
                    chunk_y,
                    chunk,
                    chunk_pixel_size,
                    tile_size,
                    world_left,
                    world_right,
                    world_top,
                    world_bottom,
                    camera_x,
                    camera_y,
                    zoom,
                );
            }

            draw_grid(
                camera_x,
                camera_y,
                zoom,
                screen_width_px,
                screen_height_px,
                tile_size,
            );
        }
    }

    fn draw_chunk(
        &self,
        chunk_x: &i32,
        chunk_y: &i32,
        chunk: &crate::world::localmap::chunk::Chunk,
        chunk_pixel_size: i32,
        tile_size: i32,
        world_left: i32,
        world_right: i32,
        world_top: i32,
        world_bottom: i32,
        camera_x: f32,
        camera_y: f32,
        zoom: f32,
    ) {
        let chunk_pixel_x = chunk_x * chunk_pixel_size;
        let chunk_pixel_y = chunk_y * chunk_pixel_size;

        // Chunk culling (skip if completely outside view)
        if chunk_pixel_x + chunk_pixel_size < world_left
            || chunk_pixel_x > world_right
            || chunk_pixel_y + chunk_pixel_size < world_top
            || chunk_pixel_y > world_bottom
        {
            return;
        }

        // Precompute tile bounds for this chunk
        let tile_x_start = ((world_left - chunk_pixel_x).max(0) / tile_size) as usize;
        let tile_x_end = ((world_right - chunk_pixel_x).min(chunk_pixel_size - 1) / tile_size + 1).min(chunk.tiles.len() as i32) as usize;
        let tile_y_start = ((world_top - chunk_pixel_y).max(0) / tile_size) as usize;
        let tile_y_end = ((world_bottom - chunk_pixel_y).min(chunk_pixel_size - 1) / tile_size + 1).min(chunk.tiles[0].len() as i32) as usize;

        for tile_x in tile_x_start..tile_x_end {
            for tile_y in tile_y_start..tile_y_end {
                self.draw_tile_in_chunk(
                    &chunk.tiles[tile_x][tile_y],
                    chunk_pixel_x,
                    chunk_pixel_y,
                    tile_x,
                    tile_y,
                    tile_size,
                    world_left,
                    world_right,
                    world_top,
                    world_bottom,
                    camera_x,
                    camera_y,
                    zoom,
                );
            }
        }
    }

    fn draw_tile_in_chunk(
        &self,
        tile: &crate::world::localmap::tile::Tile,
        chunk_pixel_x: i32,
        chunk_pixel_y: i32,
        tile_x: usize,
        tile_y: usize,
        tile_size: i32,
        world_left: i32,
        world_right: i32,
        world_top: i32,
        world_bottom: i32,
        camera_x: f32,
        camera_y: f32,
        zoom: f32,
    ) {
        let world_x = chunk_pixel_x + tile_x as i32 * tile_size;
        let world_y = chunk_pixel_y + tile_y as i32 * tile_size;

        // Tile culling (should be redundant, but double-check)
        if world_x + tile_size < world_left
            || world_x > world_right
            || world_y + tile_size < world_top
            || world_y > world_bottom
        {
            return;
        }

        self.tile_renderer.draw_tile(
            tile,
            world_x,
            world_y,
            camera_x,
            camera_y,
            zoom,
            tile_size,
            world_left,
            world_right,
            world_top,
            world_bottom,
        );
    }
} 