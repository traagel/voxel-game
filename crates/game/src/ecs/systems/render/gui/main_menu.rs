use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui};
use crate::ecs::resources::window_manager::MainMenuStateRes;

/// Draw the main menu window
pub fn draw_main_menu(state: &mut MainMenuStateRes) {
    // Draw a debug indicator to show this method was called
    draw_rectangle(200.0, 10.0, 200.0, 30.0, Color::new(0.2, 0.7, 0.3, 0.7));
    draw_text("MainMenu.draw() called", 210.0, 30.0, 16.0, WHITE);
    
    if state.show_main {
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
                state.toggle();
            }
            if ui.button(None, "Settings") {
                state.show_settings = true;
            }
            
            // Add some debug info
            ui.separator();
            ui.label(None, "Debug Info");
            ui.label(None, &format!("Screen: {}x{}", sw, sh));
            ui.label(None, &format!("show_main: {}", state.show_main));
            ui.label(None, &format!("show_settings: {}", state.show_settings));
            
            // Test button to make sure UI is responding
            if ui.button(None, "DEBUG: Test Button") {
                println!("Test button clicked!");
            }
        });
    }
    
    if state.show_settings {
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
                state.show_settings = false;
            }
        });
    }
} 