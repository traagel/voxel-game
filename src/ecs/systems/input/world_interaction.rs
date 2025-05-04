use bevy_ecs::prelude::*;
use crate::ecs::{
    components::camera::{Camera, WorldMapCamera},
    resources::{
        game_view::{GameViewRes, GameView},
        window_manager::CityInfoStateRes,
        world_map::WorldMapRes,
    },
};
use crate::input::actions::Action;
use crate::input::poll_actions;
use macroquad::prelude::*;

/// System to handle world map interactions like clicking on cities
pub fn handle_world_interactions(
    mut city_info: ResMut<CityInfoStateRes>,
    world_map: Res<WorldMapRes>,
    camera_query: Query<&Camera, With<WorldMapCamera>>,
    game_view: Res<GameViewRes>,
) {
    // Only process world interactions if we're in world map view
    if !matches!(game_view.active_view, GameView::WorldMap | GameView::CityInfo) {
        return;
    }

    // Get the camera to convert screen coordinates to world coordinates
    let camera = match camera_query.get_single() {
        Ok(camera) => camera,
        Err(_) => return, // No camera available
    };

    for action in poll_actions() {
        if let Action::CityClick { x, y } = action {
            // Convert screen coordinates to world coordinates
            let cell_size = 8.0 * camera.zoom;
            let world_x = (x / cell_size) + camera.x;
            let world_y = (y / cell_size) + camera.y;
            
            // Round to get tile coordinates
            let tile_x = world_x.floor() as usize;
            let tile_y = world_y.floor() as usize;
            
            // Check if coordinates are within world map bounds
            if tile_x >= world_map.0.width || tile_y >= world_map.0.height {
                continue;
            }
            
            // Find a city at or near these coordinates
            for city in &world_map.0.cities {
                // Use a distance check to make it easier to click on cities
                // This uses a simplified distance calculation for efficiency
                let distance = ((city.x as f32 - tile_x as f32).abs().powi(2) + 
                               (city.y as f32 - tile_y as f32).abs().powi(2)).sqrt();
                
                if distance <= 1.0 {  // Allow clicking within 1 tile of city center
                    // Found a city! Update the city info state
                    city_info.selected_city = Some(city.clone());
                    city_info.selected_civ = Some(city.civ);
                    city_info.show();
                    // Debug info
                    println!("Selected city: {} at ({}, {})", city.name, city.x, city.y);
                    return;
                }
            }
        }
    }
} 