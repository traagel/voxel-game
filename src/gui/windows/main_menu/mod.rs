// Main menu window logic will go here

pub mod state;
pub use state::MainMenuState;

use crate::gui::GuiState;
use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui};

impl MainMenuState {
    /// Toggle the main‐menu on/off
    pub fn toggle_main(&mut self) {
        self.show_main = !self.show_main;
    }

    /// Draw the main menu (and settings window) when toggled on
    pub fn draw(&mut self) {
        if self.show_main {
            let win_size = vec2(400.0, 300.0);
            let sw = screen_width() as i32;
            let sh = screen_height() as i32;
            let win_pos = vec2(
                (sw as f32 - win_size.x) * 0.5,
                (sh as f32 - win_size.y) * 0.5,
            );
            let id = hash!("main_menu", sw, sh);
            root_ui().window(id, win_pos, win_size, |ui| {
                ui.label(None, "🛠️  Main Menu");
                ui.separator();
                if ui.button(None, "Resume (Esc)") {
                    self.toggle_main();
                }
                if ui.button(None, "Settings") {
                    self.show_settings = true;
                }
            });
        }
        if self.show_settings {
            let win_size = vec2(300.0, 200.0);
            let sw = screen_width() as i32;
            let sh = screen_height() as i32;
            let win_pos = vec2(
                (sw as f32 - win_size.x) * 0.5,
                (sh as f32 - win_size.y) * 0.5,
            );
            let id = hash!("settings", sw, sh);
            root_ui().window(id, win_pos, win_size, |ui| {
                ui.label(None, "⚙️  Settings go here");
                if ui.button(None, "Close Settings") {
                    self.show_settings = false;
                }
            });
        }
    }
} 