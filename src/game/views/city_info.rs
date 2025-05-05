use crate::gui::windows::city_info::city_info_window;
use crate::gui::windows::window_manager::WindowManager;
use crate::gui::windows::city_info::portraits::CivPortraits;
use crate::world::worldmap::world_map::WorldMap;
use crate::renderer::world_map_renderer::WorldMapRenderer;
use crate::renderer::camera::Camera;

pub fn render(
    world_map: &WorldMap, 
    world_map_renderer: &WorldMapRenderer,
    world_map_camera: &Camera,
    window_manager: &mut WindowManager,
    portraits: &CivPortraits,
) -> Option<super::GameView> {
    // Draw world map in background
    world_map_renderer.draw_world_map_with_view(
        world_map,
        world_map_camera,
        crate::renderer::world_map_renderer::MapView::Biome,
        world_map.sea_level,
    );
    
    crate::gui::windows::worldgen::draw_worldgen_window(&mut window_manager.worldgen);
    
    let city_info_state = &mut window_manager.city_info;
    
    // Extract the city first to avoid borrowing issues
    let city_option = city_info_state.selected_city.clone();
    
    if let Some(city) = city_option {
        city_info_window(
            city_info_state,
            &city, // Reference to our cloned city
            portraits,
            world_map,
        );
        
        // If the city info window was closed, return to WorldMap view
        if !city_info_state.show {
            return Some(super::GameView::WorldMap);
        }
    } else {
        // If no city is selected, return to WorldMap view
        return Some(super::GameView::WorldMap);
    }
    
    None
} 