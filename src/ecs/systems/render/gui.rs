use bevy_ecs::prelude::*;
use crate::ecs::resources::{
    gui_state::GuiStateRes,
    game_view::{GameViewRes, GameView, RenderMode},
    window_manager::{
        CityInfoStateRes,
        MainMenuStateRes,
        WorldGenWindowStateRes,
        WorkerInfoStateRes,
    },
    world_map::WorldMapRes,
    portraits::CivPortraitsRes,
};
use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui};

pub fn draw_gui(
    gui_state: Res<GuiStateRes>,
    mut game_view: ResMut<GameViewRes>,
    mut city_info: ResMut<CityInfoStateRes>,
    mut main_menu: ResMut<MainMenuStateRes>,
    mut worldgen: ResMut<WorldGenWindowStateRes>,
    mut worker_info: ResMut<WorkerInfoStateRes>,
    world_map: Res<WorldMapRes>,
    portraits: Option<Res<CivPortraitsRes>>,
) {
    // We don't have direct access to current camera, so we'll use approximation
    // based on screen dimensions and position
    let screen_center = vec2(screen_width() / 2.0, screen_height() / 2.0);
    
    // Save camera state by pushing a new camera stack context
    push_camera_state();
    
    // Reset to default camera for UI drawing
    set_default_camera();
    
    // Debug visualization to ensure system is running
    let debug_rect_pos = vec2(10.0, 10.0);
    let debug_rect_size = vec2(250.0, 240.0);
    draw_rectangle(debug_rect_pos.x, debug_rect_pos.y, debug_rect_size.x, debug_rect_size.y, Color::new(0.1, 0.1, 0.1, 0.7));
    draw_text("GUI Debug Info:", debug_rect_pos.x + 5.0, debug_rect_pos.y + 20.0, 16.0, WHITE);
    draw_text(&format!("Show UI: {}", gui_state.show_ui), debug_rect_pos.x + 5.0, debug_rect_pos.y + 40.0, 14.0, WHITE);
    draw_text(&format!("Game View: {:?}", game_view.active_view), debug_rect_pos.x + 5.0, debug_rect_pos.y + 60.0, 14.0, WHITE);
    draw_text(&format!("MainMenu Visible: {}", main_menu.is_visible()), debug_rect_pos.x + 5.0, debug_rect_pos.y + 80.0, 14.0, WHITE);
    draw_text(&format!("WorldGen Visible: {}", worldgen.is_visible()), debug_rect_pos.x + 5.0, debug_rect_pos.y + 100.0, 14.0, WHITE);
    
    // Camera debug info
    draw_text(&format!("Screen Dimensions: {} x {}", screen_width(), screen_height()), 
        debug_rect_pos.x + 5.0, debug_rect_pos.y + 120.0, 13.0, YELLOW);
    draw_text(&format!("Screen Center: ({:.1}, {:.1})", screen_center.x, screen_center.y), 
        debug_rect_pos.x + 5.0, debug_rect_pos.y + 140.0, 13.0, GREEN);
    draw_text(&format!("Camera Mode: UI Default"), 
        debug_rect_pos.x + 5.0, debug_rect_pos.y + 160.0, 13.0, SKYBLUE);
    
    // Keyboard controls help
    draw_text("F1: Show Main Menu", debug_rect_pos.x + 5.0, debug_rect_pos.y + 180.0, 12.0, LIGHTGRAY);
    draw_text("F2: Toggle WorldGen", debug_rect_pos.x + 5.0, debug_rect_pos.y + 200.0, 12.0, LIGHTGRAY);
    draw_text("F3: Toggle between WorldMap/MainMenu", debug_rect_pos.x + 5.0, debug_rect_pos.y + 220.0, 12.0, LIGHTGRAY);
    
    // Important troubleshooting info to make bold and obvious
    let important_msg = if game_view.active_view == GameView::WorldMap {
        "** WORLD MAP SHOULD BE VISIBLE **"
    } else {
        "** PRESS F3 TO VIEW WORLD MAP **"
    };
    draw_text(important_msg, 
        screen_width() / 2.0 - 180.0, 
        screen_height() - 60.0, 
        20.0, 
        Color::new(1.0, 0.8, 0.0, 1.0));
        
    // Debug key to force show the main menu
    if is_key_pressed(KeyCode::F1) {
        main_menu.show();
        println!("Debug: Forcing MainMenu visibility ON");
    }
    
    // Add key to toggle window gen window
    if is_key_pressed(KeyCode::F2) {
        worldgen.toggle();
        println!("Debug: Toggling WorldGen window, now: {}", worldgen.is_visible());
    }
    
    // Add key to switch to world map view
    if is_key_pressed(KeyCode::F3) {
        if game_view.active_view == GameView::MainMenu {
            game_view.active_view = GameView::WorldMap;
            println!("Debug: Switching to WORLD MAP view");
        } else {
            game_view.active_view = GameView::MainMenu;
            println!("Debug: Switching to MAIN MENU view");
        }
    }
    
    // Skip drawing other GUI elements if GUI is disabled
    if !gui_state.show_ui {
        // Restore previous camera before returning
        pop_camera_state();
        return;
    }

    // Draw the main menu if it's visible or we're in MainMenu view
    if main_menu.is_visible() || matches!(game_view.active_view, GameView::MainMenu) {
        draw_main_menu(&mut main_menu);
    }
    
    // Normal game view-dependent GUI
    match game_view.active_view {
        GameView::MainMenu => {
            // Main menu is already drawn above
        },
        GameView::WorldGen => {
            draw_worldgen_window(&mut worldgen);
        },
        GameView::WorldMap => {
            // Draw world generation settings window
            draw_worldgen_window(&mut worldgen);
            
            // Draw city info window if a city is selected
            let city_info_selected = city_info.selected_city.clone();
            
            if let (Some(city), Some(portraits_res)) = (city_info_selected.as_ref(), &portraits) {
                draw_city_info_window(
                    &mut city_info,
                    city,
                    &portraits_res.0,
                    &world_map.0,
                );
            }
        },
        GameView::RegionMap => {
            // Simple placeholder for region map GUI
            draw_text("[Region Map View - TODO]", 100.0, 100.0, 32.0, WHITE);
        },
        GameView::LocalMap => {
            // Draw worker info window
            draw_worker_info_window(&mut worker_info);
        },
        GameView::CityInfo => {
            // World gen settings are shown in city info view too
            draw_worldgen_window(&mut worldgen);
            
            // Draw city info window if a city is selected
            let city_info_selected = city_info.selected_city.clone();
            
            if let (Some(city), Some(portraits_res)) = (city_info_selected.as_ref(), &portraits) {
                draw_city_info_window(
                    &mut city_info,
                    city,
                    &portraits_res.0,
                    &world_map.0,
                );
            }
        },
    }
    
    // Restore the previous camera state
    pop_camera_state();
    
    // Display camera state message after restoration
    let after_pos = vec2(10.0, screen_height() - 30.0);
    draw_rectangle(after_pos.x, after_pos.y, 400.0, 25.0, Color::new(0.1, 0.1, 0.1, 0.7));
    draw_text(&format!("Camera restored, GameView: {:?}", game_view.active_view), 
        after_pos.x + 5.0, after_pos.y + 20.0, 13.0, YELLOW);
}

// Drawing helper functions - these would be similar to your existing GUI functions,
// but operate directly on ECS resources

fn draw_main_menu(state: &mut MainMenuStateRes) {
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

fn draw_city_info_window(
    state: &mut CityInfoStateRes, 
    city: &crate::world::worldmap::city::City, 
    portraits: &crate::ecs::resources::portraits::CivPortraits, 
    world_map: &crate::world::worldmap::world_map::WorldMap
) {
    if !state.show { return; }
    
    let win_pos = vec2(900.0, 80.0);
    let win_size = vec2(420.0, 340.0);
    
    // Calculate portrait position
    let portrait_size = 96.0;
    let px = win_pos.x + win_size.x - portrait_size - 16.0;
    let py = win_pos.y + 16.0;

    // Draw the portrait BEFORE the window, so the window doesn't cover it
    if let Some(src_rect) = portraits.get_portrait_rect(city.civ) {
        draw_texture_ex(
            portraits.get_texture(),
            px,
            py,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(portrait_size, portrait_size)),
                source: Some(src_rect),
                ..Default::default()
            },
        );
    }

    // Now draw the UI window (it will be drawn over the portrait, so leave space)
    root_ui().window(hash!("city_info_window"), win_pos, win_size, |ui| {
        ui.label(None, &format!("City: {}", city.name));
        ui.label(None, &format!("Population: {}", city.population));
        ui.label(None, &format!("Coordinates: ({}, {})", city.x, city.y));
        ui.separator();
        // Civilization info
        let civ = city.civ;
        let color = civ.color();
        ui.label(None, &format!("Civilization: {:?}", civ));
        ui.label(None, &format!("Civ Color: rgba({:.2}, {:.2}, {:.2}, {:.2})", color.r, color.g, color.b, color.a));
        // Preferred biomes
        let biomes = civ.preferred_biomes();
        let biome_names: Vec<String> = biomes.iter().map(|b| format!("{:?}", b)).collect();
        ui.label(None, &format!("Preferred Biomes: {}", biome_names.join(", ")));
        // --- Civilization extended info ---
        if let Some(civ_instance) = &world_map.civilization_map[city.x][city.y] {
            ui.separator();
            ui.label(None, &format!("Alignment: {:?}", civ_instance.culture.alignment));
            ui.label(None, &format!("Tradition: {}", civ_instance.culture.tradition));
            ui.label(None, &format!("Religion: {}", civ_instance.culture.religion));
            ui.label(None, &format!("Trait: {:?}", civ_instance.culture.trait_));
        }
        // Show relations to other civs
        ui.separator();
        ui.label(None, "Relations:");
        use crate::world::worldmap::civilization::Civilization as CivEnum;
        for other in [
            CivEnum::Human,
            CivEnum::Elf,
            CivEnum::Dwarf,
            CivEnum::GnomeHalfling,
            CivEnum::OrcGoblin,
            CivEnum::Merfolk,
            CivEnum::Lizardfolk,
            CivEnum::FairyFae,
            CivEnum::Kobold,
        ] {
            if other != civ {
                if let Some(rel) = world_map.civ_relations.relations.get(&(civ, other)) {
                    ui.label(None, &format!("  {:?} ↔ {:?}: {:?}", civ, other, rel));
                }
            }
        }
        ui.separator();
        // Close button
        if ui.button(None, "Close") {
            state.show = false;
        }
    });
}

fn draw_worldgen_window(state: &mut WorldGenWindowStateRes) {
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

fn draw_worker_info_window(state: &mut WorkerInfoStateRes) {
    // Return early if the window shouldn't be shown
    if !state.is_visible() {
        return;
    }
    
    let win_pos = vec2(40.0, 40.0);
    let win_size = vec2(300.0, 200.0);
    root_ui().window(hash!("worker_info_window"), win_pos, win_size, |ui| {
        ui.label(None, "[Worker Info Window - TODO]");
        
        // Add a close button
        if ui.button(None, "Close") {
            state.hide();
        }
    });
} 