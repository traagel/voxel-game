use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui};

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

    pub fn update(&mut self) {
        if is_key_pressed(KeyCode::Tab) {
            self.show_ui = !self.show_ui;
        }

        if self.show_ui {
            root_ui().window(hash!(), vec2(20.0, 20.0), vec2(250.0, 150.0), |ui| {
                ui.label(None, "Voxel Game Debug UI");

                if ui.button(None, if self.paused { "Resume" } else { "Pause" }) {
                    self.paused = !self.paused;
                }

                ui.separator();
                ui.label(None, &format!("Active Dig Jobs: {}", self.dig_jobs));
            });
        }
    }
}
