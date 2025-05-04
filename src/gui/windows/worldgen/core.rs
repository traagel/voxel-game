use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui};
use crate::gui::windows::worldgen::state::WorldGenWindowState;

pub fn draw_worldgen_window(state: &mut WorldGenWindowState) {
    // Debug visualization to show this function is being called
    draw_rectangle(420.0, 10.0, 200.0, 30.0, Color::new(0.7, 0.2, 0.3, 0.7));
    draw_text(&format!("WorldGen.draw() show={}", state.show), 430.0, 30.0, 16.0, WHITE);
    
    // Return early if the window shouldn't be shown
    if !state.show {
        return;
    }
    
    let win_pos = vec2(300.0, 20.0);
    let win_size = vec2(320.0, 500.0);

    root_ui().window(hash!("worldgen"), win_pos, win_size, |ui| {
        ui.label(None, "World Generation");
        ui.separator();
        // Seed controls
        ui.label(None, &format!("Seed: {}", state.seed));
        if ui.button(None, "Randomize Seed") {
            state.seed = macroquad::rand::rand();
            state.regenerate_requested = true;
        }
        ui.separator();
        // Map size controls
        ui.label(None, "Map Width:");
        for &w in &[128, 256, 512] {
            let selected = state.width == w;
            let label = format!("{}{}", if selected { "● " } else { "○ " }, w);
            if ui.button(None, label.as_str()) {
                state.width = w;
            }
        }
        ui.separator();
        ui.label(None, "Map Height:");
        for &h in &[128, 256, 512] {
            let selected = state.height == h;
            let label = format!("{}{}", if selected { "● " } else { "○ " }, h);
            if ui.button(None, label.as_str()) {
                state.height = h;
            }
        }
        ui.separator();
        // ocean_percent (f64)
        let mut ocean_percent = state.params.ocean_percent as f32;
        ui.slider(hash!("ocean_percent"), "Ocean %", 0.0..0.8, &mut ocean_percent);
        state.params.ocean_percent = ocean_percent as f64;
        // coast_percent (f64)
        let mut coast_percent = state.params.coast_percent as f32;
        ui.slider(hash!("coast_percent"), "Coast %", 0.0..0.3, &mut coast_percent);
        state.params.coast_percent = coast_percent as f64;
        // mountain_percent (f64)
        let mut mountain_percent = state.params.mountain_percent as f32;
        ui.slider(hash!("mountain_percent"), "Mountain %", 0.0..0.3, &mut mountain_percent);
        state.params.mountain_percent = mountain_percent as f64;
        // erosion_iterations (usize)
        let mut erosion_iterations = state.params.erosion_iterations as f32;
        ui.slider(hash!("erosion_iterations"), "Erosion Iterations", 0.0..100.0, &mut erosion_iterations);
        state.params.erosion_iterations = erosion_iterations.clamp(0.0, 100.0) as usize;
        // river_threshold (f64)
        let mut river_threshold = state.params.river_threshold as f32;
        ui.slider(hash!("river_threshold"), "River Threshold", 10.0..100.0, &mut river_threshold);
        state.params.river_threshold = river_threshold as f64;
        // continent_scale (f64)
        let mut continent_scale = state.params.continent_scale as f32;
        ui.slider(hash!("continent_scale"), "Continent Scale", 0.05..1.0, &mut continent_scale);
        state.params.continent_scale = continent_scale as f64;
        // detail_scale (f64)
        let mut detail_scale = state.params.detail_scale as f32;
        ui.slider(hash!("detail_scale"), "Detail Scale", 8.0..40.0, &mut detail_scale);
        state.params.detail_scale = detail_scale as f64;
        // octaves_continent (usize)
        let mut octaves_continent = state.params.octaves_continent as f32;
        ui.slider(hash!("octaves_continent"), "Octaves Continent", 3.0..12.0, &mut octaves_continent);
        state.params.octaves_continent = octaves_continent.clamp(3.0, 12.0) as usize;
        // octaves_detail (usize)
        let mut octaves_detail = state.params.octaves_detail as f32;
        ui.slider(hash!("octaves_detail"), "Octaves Detail", 5.0..16.0, &mut octaves_detail);
        state.params.octaves_detail = octaves_detail.clamp(5.0, 16.0) as usize;
        // persistence (f64)
        let mut persistence = state.params.persistence as f32;
        ui.slider(hash!("persistence"), "Persistence", 0.7..2.0, &mut persistence);
        state.params.persistence = persistence as f64;
        // num_continents (usize)
        let mut num_continents = state.params.num_continents as f32;
        ui.slider(hash!("num_continents"), "Num Continents", 1.0..8.0, &mut num_continents);
        state.params.num_continents = num_continents.clamp(1.0, 8.0) as usize;
        if ui.button(None, "Regenerate World Map") {
            state.regenerate_requested = true;
        }
        // Add a close button
        if ui.button(None, "Close") {
            state.show = false;
        }
    });
} 