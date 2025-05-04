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
use crate::worldgen::worldmap::builder::WorldMapBuilder;

/// System to handle interactions between windows and game state updates
pub fn update_window_interactions(
    mut city_info: ResMut<CityInfoStateRes>,
    mut worldgen: ResMut<WorldGenWindowStateRes>,
    game_view: Res<GameViewRes>,
    mut world_map: ResMut<WorldMapRes>,
) {
    // Handle world generation requests
    if worldgen.regenerate_requested {
        // Create a new WorldMapBuilder with settings from worldgen
        let builder = WorldMapBuilder::new(
            worldgen.seed,
            worldgen.width,
            worldgen.height,
            0.02, // Hardcoded scale for now - could be added to WorldGenWindowStateRes if needed
            Some(worldgen.params.clone()),
        );
        
        // Generate a new world map
        println!("Regenerating world map with new settings from UI");
        let new_world_map = builder.generate();
        
        // Update the world map resource
        world_map.0 = new_world_map;
        
        // Reset the flag
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