use bevy_ecs::prelude::*;
use crate::ecs::resources::{
    gui_state::GuiStateRes,
    game_view::{GameViewRes, GameView, RenderMode},
    window_manager::{
        WindowManagerRes,
        CityInfoStateRes,
        MainMenuStateRes,
        WorldGenWindowStateRes,
        WorkerInfoStateRes,
    },
    world_map::WorldMapRes,
    portraits::CivPortraitsRes,
};
use crate::gui::windows::window_state::WindowState;
use macroquad::prelude::*;

pub fn draw_gui(
    gui_state: Res<GuiStateRes>,
    mut game_view: ResMut<GameViewRes>,
    mut window_manager: ResMut<WindowManagerRes>,
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
    draw_text(&format!("Show UI: {}", gui_state.0.show_ui), debug_rect_pos.x + 5.0, debug_rect_pos.y + 40.0, 14.0, WHITE);
    draw_text(&format!("Game View: {:?}", game_view.active_view), debug_rect_pos.x + 5.0, debug_rect_pos.y + 60.0, 14.0, WHITE);
    draw_text(&format!("MainMenu Visible: {}", main_menu.0.is_visible()), debug_rect_pos.x + 5.0, debug_rect_pos.y + 80.0, 14.0, WHITE);
    draw_text(&format!("WorldGen Visible: {}", worldgen.0.is_visible()), debug_rect_pos.x + 5.0, debug_rect_pos.y + 100.0, 14.0, WHITE);
    
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
        main_menu.0.show();
        println!("Debug: Forcing MainMenu visibility ON");
    }
    
    // Add key to toggle window gen window
    if is_key_pressed(KeyCode::F2) {
        worldgen.0.toggle();
        println!("Debug: Toggling WorldGen window, now: {}", worldgen.0.is_visible());
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
    if !gui_state.0.show_ui {
        // Restore previous camera before returning
        pop_camera_state();
        return;
    }

    // Debug draw - Only draw the main menu if it's visible or we're in MainMenu view
    if main_menu.0.is_visible() || matches!(game_view.active_view, GameView::MainMenu) {
        main_menu.0.draw();
    }
    
    // Normal game view-dependent GUI
    match game_view.active_view {
        GameView::MainMenu => {
            // Main menu is already drawn above
        },
        GameView::WorldGen => {
            crate::gui::windows::worldgen::draw_worldgen_window(&mut worldgen.0);
        },
        GameView::WorldMap => {
            // Draw world generation settings window
            crate::gui::windows::worldgen::draw_worldgen_window(&mut worldgen.0);
            
            // Draw city info window if a city is selected
            let city_info_selected = city_info.0.selected_city.clone();
            
            if let (Some(city), Some(portraits_res)) = (city_info_selected.as_ref(), &portraits) {
                crate::gui::windows::city_info::city_info_window(
                    &mut city_info.0,
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
            crate::gui::windows::worker_info::draw_worker_info_window(&mut worker_info.0);
        },
        GameView::CityInfo => {
            // World gen settings are shown in city info view too
            crate::gui::windows::worldgen::draw_worldgen_window(&mut worldgen.0);
            
            // Draw city info window if a city is selected
            let city_info_selected = city_info.0.selected_city.clone();
            
            if let (Some(city), Some(portraits_res)) = (city_info_selected.as_ref(), &portraits) {
                crate::gui::windows::city_info::city_info_window(
                    &mut city_info.0,
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

// (If your GUI was entirely inside WindowManager, adjust the signature to take only that resource.) 