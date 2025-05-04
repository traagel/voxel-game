use bevy_ecs::prelude::*;
use crate::ecs::resources::{
    gui_state::GuiStateRes,
    game_view::GameViewRes,
    game_view::GameView,
    window_manager::WindowManagerRes,
    world_map::WorldMapRes,
    portraits::CivPortraitsRes,
};
use macroquad::prelude::*;

pub fn draw_gui(
    gui_state: Res<GuiStateRes>,
    game_view: Res<GameViewRes>,
    mut window_manager: ResMut<WindowManagerRes>,
    world_map: Res<WorldMapRes>,
    portraits: Option<Res<CivPortraitsRes>>,
) {
    // Draw a debug indicator regardless of view to verify rendering is working
    draw_rectangle(10.0, 10.0, 50.0, 50.0, RED);
    draw_circle(100.0, 100.0, 30.0, BLUE);
    draw_text("RENDERING TEST", 150.0, 50.0, 20.0, WHITE);
    
    // Reset to default camera for UI drawing
    set_default_camera();

    match game_view.active_view {
        GameView::MainMenu => {
            window_manager.0.main_menu.draw();
        },
        GameView::WorldGen => {
            crate::gui::windows::worldgen::draw_worldgen_window(&mut window_manager.0.worldgen);
        },
        GameView::WorldMap => {
            // Draw world generation settings window
            crate::gui::windows::worldgen::draw_worldgen_window(&mut window_manager.0.worldgen);
        },
        GameView::RegionMap => {
            // Simple placeholder for region map GUI - REMOVE CLEAR BACKGROUND
            // clear_background(DARKGRAY);  // COMMENTED OUT - Was overwriting other rendering
            draw_text("[Region Map View - TODO]", 100.0, 100.0, 32.0, WHITE);
        },
        GameView::LocalMap => {
            // Draw worker info window
            crate::gui::windows::worker_info::draw_worker_info_window(&mut window_manager.0.worker_info);
        },
        GameView::CityInfo => {
            // World gen settings are shown in city info view too
            crate::gui::windows::worldgen::draw_worldgen_window(&mut window_manager.0.worldgen);
            
            // Draw city info window if a city is selected
            let city_info_selected = window_manager.0.city_info.selected_city.clone();
            
            if let (Some(city), Some(portraits)) = (city_info_selected.as_ref(), portraits) {
                crate::gui::windows::city_info::city_info_window(
                    &mut window_manager.0.city_info,
                    city,
                    &portraits.0,
                    &world_map.0,
                );
            }
        },
    }
}

// (If your GUI was entirely inside WindowManager, adjust the signature to take only that resource.) 