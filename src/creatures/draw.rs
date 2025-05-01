use macroquad::prelude::*;
use super::Creature;

impl Creature {
    pub fn draw(&self, camera_x: f32, camera_y: f32, zoom: f32) {
        let screen_x = (self.x - camera_x) * zoom;
        let screen_y = (self.y - camera_y) * zoom;
        draw_circle(screen_x, screen_y, self.size * zoom, self.color);
    }
} 