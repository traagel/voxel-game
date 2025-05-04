use bevy_ecs::prelude::*;
use crate::ecs::resources::{
    window_manager::{
        CityInfoStateRes,
        MainMenuStateRes,
        WorldGenWindowStateRes,
        WorkerInfoStateRes,
    },
    game_view::{GameViewRes, GameView},
    world_map::WorldMapRes,
};

/// System to handle interactions between windows and game state updates
pub fn update_window_interactions(
    mut city_info: ResMut<CityInfoStateRes>,
    mut worldgen: ResMut<WorldGenWindowStateRes>,
    game_view: Res<GameViewRes>,
    world_map: Res<WorldMapRes>,
) {
    // Handle world generation requests
    if worldgen.regenerate_requested {
        // Here we would trigger the world regeneration
        // This could involve sending an event or updating a flag in a resource
        worldgen.regenerate_requested = false;
    }
    
    // Update city info based on selections
    match game_view.active_view {
        GameView::WorldMap | GameView::CityInfo => {
            // Here we would handle city selection updates
            // This is a placeholder for the actual implementation
            if let Some(selected_city) = &city_info.selected_city {
                // If a city is selected, update any other dependent UI state
                // For example, update the civilization information
                city_info.selected_civ = Some(selected_city.civ);
            }
        },
        _ => {}
    }
} 