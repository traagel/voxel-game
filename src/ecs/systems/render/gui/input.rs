use macroquad::prelude::*;
use crate::ecs::resources::{
    game_view::{GameViewRes, GameView},
    window_manager::{
        MainMenuStateRes,
        WorldGenWindowStateRes,
    },
};

/// Handle keyboard inputs for controlling GUI state and view switching
pub fn handle_key_inputs(
    main_menu: &mut MainMenuStateRes,
    worldgen: &mut WorldGenWindowStateRes,
    game_view: &mut GameViewRes,
) {
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
} 