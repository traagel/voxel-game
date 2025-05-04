use macroquad::prelude::*;
use crate::ecs::resources::{
    gui_state::GuiStateRes,
    game_view::GameViewRes,
    window_manager::{
        MainMenuStateRes,
        WorldGenWindowStateRes,
    },
};

/// Draw debug information on screen
pub fn draw_debug_info(
    gui_state: &GuiStateRes,
    game_view: &GameViewRes,
    main_menu: &MainMenuStateRes,
    worldgen: &WorldGenWindowStateRes,
) {
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
    let screen_center = vec2(screen_width() / 2.0, screen_height() / 2.0);
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
    let important_msg = if game_view.active_view == crate::ecs::resources::game_view::GameView::WorldMap {
        "** WORLD MAP SHOULD BE VISIBLE **"
    } else {
        "** PRESS F3 TO VIEW WORLD MAP **"
    };
    draw_text(important_msg, 
        screen_width() / 2.0 - 180.0, 
        screen_height() - 60.0, 
        20.0, 
        Color::new(1.0, 0.8, 0.0, 1.0));
} 