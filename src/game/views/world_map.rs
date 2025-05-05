use crate::renderer::camera::Camera;
use crate::renderer::world_map_renderer::{WorldMapRenderer, MapView};
use crate::world::worldmap::world_map::WorldMap;
use crate::gui::windows::worldgen::draw_worldgen_window;
use crate::gui::windows::window_manager::WindowManager;
use macroquad::prelude::*;

pub fn render(
    world_map: &WorldMap,
    world_map_renderer: &WorldMapRenderer,
    world_map_camera: &Camera,
    worldgen_window: &mut crate::gui::windows::worldgen::WorldGenWindowState,
) {
    world_map_renderer.draw_world_map_with_view(
        world_map,
        world_map_camera,
        MapView::Biome,
        world_map.sea_level,
    );
    draw_worldgen_window(worldgen_window);
}

pub fn handle_city_click(
    world_map: &WorldMap,
    world_map_camera: &Camera,
    mouse_x: f32,
    mouse_y: f32,
    window_manager: &mut WindowManager,
) -> Option<super::GameView> {
    const TILE_PX: f32 = 8.0;
    let world_x = world_map_camera.x + mouse_x / (TILE_PX * world_map_camera.zoom);
    let world_y = world_map_camera.y + mouse_y / (TILE_PX * world_map_camera.zoom);
    
    // Find city under cursor (within a radius)
    let city_radius = 0.5; // in world units
    if let Some(city) = world_map.cities.iter().find(|city| {
        let dx = city.x as f32 - world_x;
        let dy = city.y as f32 - world_y;
        (dx * dx + dy * dy).sqrt() < city_radius
    }).cloned() {
        window_manager.city_info.selected_city = Some(city);
        window_manager.city_info.show = true;
        return Some(super::GameView::CityInfo);
    }
    None
} 