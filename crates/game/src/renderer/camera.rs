use macroquad::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub x: f32,
    pub y: f32,
    pub zoom: f32,
}

impl Camera {
    pub fn new(x: f32, y: f32, zoom: f32) -> Self {
        Self { x, y, zoom }
    }

    pub fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            zoom: 4.0,
        }
    }

    pub fn move_delta(&mut self, dx: f32, dy: f32) {
        self.x += dx;
        self.y += dy;
    }

    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom = zoom;
    }

    pub fn get_zoom(&self) -> f32 {
        self.zoom
    }

    pub fn delta_zoom(&mut self, zoom: f32) {
        self.zoom += zoom;
    }

    pub fn get_x(&self) -> f32 {
        self.x
    }

    pub fn get_y(&self) -> f32 {
        self.y
    }
} 