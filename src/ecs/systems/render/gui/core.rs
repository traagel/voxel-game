use bevy_ecs::prelude::*;
use crate::ecs::resources::{
    gui_state::GuiStateRes,
    game_view::{GameViewRes, GameView},
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

use super::{
    debug,
    input,
    draw_main_menu,
    draw_city_info_window,
    draw_worldgen_window,
    draw_worker_info_window,
};

/// Main GUI drawing function that orchestrates the rendering of all GUI elements
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
    // Save camera state by pushing a new camera stack context
    push_camera_state();
    
    // Reset to default camera for UI drawing
    set_default_camera();
    
    // Draw debug information
    debug::draw_debug_info(&gui_state, &game_view, &main_menu, &worldgen);
    
    // Handle key inputs for view switching
    input::handle_key_inputs(&mut main_menu, &mut worldgen, &mut game_view);
    
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