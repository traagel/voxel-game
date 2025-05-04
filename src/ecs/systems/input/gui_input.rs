use bevy_ecs::prelude::*;
use macroquad::prelude::*;
use crate::ecs::resources::{
    window_manager::{
        CityInfoStateRes,
        MainMenuStateRes,
        WorldGenWindowStateRes,
        WorkerInfoStateRes,
    },
    game_view::{GameViewRes, GameView, RenderMode},
};

/// System to handle GUI-related keyboard/mouse input
pub fn handle_gui_input(
    mut city_info: ResMut<CityInfoStateRes>,
    mut main_menu: ResMut<MainMenuStateRes>,
    mut worldgen: ResMut<WorldGenWindowStateRes>,
    mut worker_info: ResMut<WorkerInfoStateRes>,
    game_view: Res<GameViewRes>,
) {
    // Handle Escape key to toggle main menu
    if is_key_pressed(KeyCode::Escape) {
        main_menu.toggle();
    }
    
    // Game view specific input handling
    match game_view.active_view {
        GameView::MainMenu => {
            // Main menu specific input
        },
        GameView::WorldMap => {
            // World map specific GUI input
            if is_key_pressed(KeyCode::W) {
                worldgen.toggle();
            }
            if is_key_pressed(KeyCode::C) {
                city_info.toggle();
            }
        },
        GameView::LocalMap => {
            // Local map specific GUI input
            if is_key_pressed(KeyCode::I) {
                worker_info.toggle();
            }
        },
        _ => {
            // Other views
        }
    }
} 