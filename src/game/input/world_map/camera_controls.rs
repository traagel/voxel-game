use macroquad::prelude::*;
use crate::renderer::camera::Camera;
use crate::input::manager::InputManager;
use crate::input::event::InputEvent;
use crate::world::worldmap::world_map::WorldMap;
use super::constants::*;

/// Enforces camera boundaries to prevent moving too far off the map
/// 
/// Takes into account:
/// - The map dimensions
/// - The current zoom level
/// - The screen dimensions
/// 
/// Returns true if the camera position was adjusted, false otherwise
pub fn enforce_camera_boundaries(
    camera: &mut Camera,
    world_map: &WorldMap,
) -> bool {
    let original_x = camera.x;
    let original_y = camera.y;
    
    // Get the map dimensions
    let map_width = world_map.width as f32;
    let map_height = world_map.height as f32;
    
    // Get the visible area in world units
    let zoom = camera.zoom;
    let visible_width = screen_width() / (TILE_PX * zoom);
    let visible_height = screen_height() / (TILE_PX * zoom);
    
    // Calculate the minimum and maximum allowed camera positions
    // This ensures at least some part of the map is always visible
    let min_x = MIN_CAMERA_DISTANCE;
    let min_y = MIN_CAMERA_DISTANCE;
    let max_x = map_width - visible_width + CAMERA_MARGIN;
    let max_y = map_height - visible_height + CAMERA_MARGIN;
    
    // Apply constraints, making sure min values don't exceed max values
    // This can happen with very high zoom levels or large screens
    let constrained_min_x = min_x.min(max_x);
    let constrained_min_y = min_y.min(max_y);
    
    // Clamp the camera position
    let new_x = camera.x.clamp(constrained_min_x, max_x);
    let new_y = camera.y.clamp(constrained_min_y, max_y);
    
    // Set the new position if it changed
    if new_x != camera.x || new_y != camera.y {
        camera.x = new_x;
        camera.y = new_y;
        return true;
    }
    
    false
}

/// Handles camera centering (C key)
pub fn handle_center_camera(
    input: &InputManager,
    world_map_camera: &mut Camera,
    world_map: &WorldMap,
) -> bool {
    if input.key().pressed(KeyCode::C) {
        let w = world_map.width as f32;
        let h = world_map.height as f32;
        let zoom = world_map_camera.zoom;
        let sw = screen_width();
        let sh = screen_height();
        let half_wu_x = (sw * 0.5) / (TILE_PX * zoom);
        let half_wu_y = (sh * 0.5) / (TILE_PX * zoom);
        let desired_cam_x = w * 0.5 - half_wu_x;
        let desired_cam_y = h * 0.5 - half_wu_y;

        world_map_camera.move_delta(
            desired_cam_x - world_map_camera.x,
            desired_cam_y - world_map_camera.y,
        );
        
        // Enforce boundaries after centering
        enforce_camera_boundaries(world_map_camera, world_map);
        return true;
    }
    false
}

/// Handles keyboard-based camera movement (WASD keys)
pub fn handle_keyboard_movement(
    input: &InputManager,
    world_map_camera: &mut Camera,
    world_map: &WorldMap,
) -> bool {
    let state = &input.state;
    let move_speed = BASE_MOVE_SPEED * get_frame_time();
    let mut movement = false;

    if state.keys_down.contains(&KeyCode::W) {
        world_map_camera.move_delta(0.0, -move_speed);
        movement = true;
    }
    if state.keys_down.contains(&KeyCode::S) {
        world_map_camera.move_delta(0.0, move_speed);
        movement = true;
    }
    if state.keys_down.contains(&KeyCode::A) {
        world_map_camera.move_delta(-move_speed, 0.0);
        movement = true;
    }
    if state.keys_down.contains(&KeyCode::D) {
        world_map_camera.move_delta(move_speed, 0.0);
        movement = true;
    }
    
    // If there was movement, enforce boundaries
    if movement {
        enforce_camera_boundaries(world_map_camera, world_map);
    }
    
    movement
}

/// Handles mouse wheel zoom
pub fn handle_zoom(
    input: &InputManager,
    world_map_camera: &mut Camera,
    world_map: &WorldMap,
) -> bool {
    let state = &input.state;
    let wheel = state.mouse_scroll;
    
    if wheel != 0.0 {
        let old_zoom = world_map_camera.zoom;
        let new_zoom = (old_zoom + wheel * ZOOM_SPEED).clamp(MIN_ZOOM, MAX_ZOOM);
        let old_scale = TILE_PX * old_zoom;
        let (mx, my) = state.mouse_position;
        let world_x = world_map_camera.x + mx / old_scale;
        let world_y = world_map_camera.y + my / old_scale;
        world_map_camera.set_zoom(new_zoom);
        let new_scale = TILE_PX * new_zoom;
        let new_cam_x = world_x - mx / new_scale;
        let new_cam_y = world_y - my / new_scale;
        world_map_camera.move_delta(
            new_cam_x - world_map_camera.x,
            new_cam_y - world_map_camera.y,
        );
        
        // Enforce boundaries after zooming
        enforce_camera_boundaries(world_map_camera, world_map);
        return true;
    }
    
    false
} 