use crate::world::localmap::world::World;
use crate::game::input::RenderMode;
use crate::input::manager::InputManager;
use crate::input::event::InputEvent;

pub struct GuiState {
    pub show_ui: bool,
    pub paused: bool,
    pub dig_jobs: usize,
}

impl GuiState {
    pub fn new() -> Self {
        Self {
            show_ui: true,
            paused: false,
            dig_jobs: 0,
        }
    }

    pub fn update(&mut self, world: &World, render_mode: RenderMode, input: &InputManager) {
        if input.events().iter().any(|e| matches!(e, InputEvent::KeyDown(macroquad::prelude::KeyCode::Tab))) {
            self.show_ui = !self.show_ui;
        }
        // Only keep debug UI logic here if needed
    }
} 