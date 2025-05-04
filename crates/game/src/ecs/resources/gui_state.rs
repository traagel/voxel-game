use bevy_ecs::prelude::*;
use crate::ecs::resources::game_view::RenderMode;

#[derive(Resource, Default)]
pub struct GuiStateRes {
    pub show_ui: bool,
    pub paused: bool,
    pub dig_jobs: usize,
}

impl GuiStateRes {
    pub fn new() -> Self {
        Self {
            show_ui: true,
            paused: false,
            dig_jobs: 0,
        }
    }

    pub fn update(&mut self, render_mode: RenderMode) {
        if macroquad::prelude::is_key_pressed(macroquad::prelude::KeyCode::Tab) {
            self.show_ui = !self.show_ui;
        }
        // Only keep debug UI logic here if needed
    }
} 