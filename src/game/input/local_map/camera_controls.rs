use macroquad::prelude::*;
use crate::renderer::local_map_renderer::LocalMapRenderer;
use crate::world::localmap::world::World;
use crate::input::manager::InputManager;
use crate::input::event::InputEvent;
use super::constants::*;

/// Enforces camera boundaries to prevent moving too far off the map
/// 
/// Takes into account:
/// - The map dimensions (from loaded chunks)
/// - The current zoom level
/// - The screen dimensions
/// 
/// Returns true if the camera position was adjusted, false otherwise
pub fn enforce_camera_boundaries(
    local_map_renderer: &mut LocalMapRenderer,
    world: &World,
) -> bool {
    let original_x = local_map_renderer.get_camera_x();
    let original_y = local_map_renderer.get_camera_y();
    
    // Get the map dimensions from loaded chunks
    if let Some(zlevel) = world.z_levels.get(0) {
        if zlevel.chunks.is_empty() {
            return false; // No chunks loaded, nothing to constrain to
        }
        
        // gather all loaded chunk coordinates
        let mut xs: Vec<i32> = zlevel.chunks.keys().map(|(cx, _)| *cx).collect();
        let mut ys: Vec<i32> = zlevel.chunks.keys().map(|(_, cy)| *cy).collect();
        
        if !xs.is_empty() {
            xs.sort_unstable();
            ys.sort_unstable();

            // compute world-unit extents
            let min_cx = xs[0] as f32;
            let max_cx = xs[xs.len()-1] as f32 + 1.0;
            let min_cy = ys[0] as f32;
            let max_cy = ys[ys.len()-1] as f32 + 1.0;

            // Calculate world units per chunk
            let subpx_per_chunk = CHUNK_SIZE * TILE_PX;
            
            // Map bounds in world units
            let map_min_x = min_cx * subpx_per_chunk;
            let map_max_x = max_cx * subpx_per_chunk;
            let map_min_y = min_cy * subpx_per_chunk;
            let map_max_y = max_cy * subpx_per_chunk;
            
            // Get the visible area in world units
            let zoom = local_map_renderer.get_zoom();
            let visible_width = screen_width() / zoom;
            let visible_height = screen_height() / zoom;
            
            // Calculate the minimum and maximum allowed camera positions
            // This ensures at least some part of the map is always visible
            let min_x = map_min_x + MIN_CAMERA_DISTANCE;
            let min_y = map_min_y + MIN_CAMERA_DISTANCE;
            let max_x = map_max_x - visible_width + CAMERA_MARGIN;
            let max_y = map_max_y - visible_height + CAMERA_MARGIN;
            
            // Apply constraints, making sure min values don't exceed max values
            // This can happen with very high zoom levels or large screens
            let constrained_min_x = min_x.min(max_x);
            let constrained_min_y = min_y.min(max_y);
            
            // Clamp the camera position
            let new_x = local_map_renderer.get_camera_x().clamp(constrained_min_x, max_x);
            let new_y = local_map_renderer.get_camera_y().clamp(constrained_min_y, max_y);
            
            // Set the new position if it changed
            if (new_x - original_x).abs() > f32::EPSILON || (new_y - original_y).abs() > f32::EPSILON {
                // Use move_camera_delta to set the position relative to the current position
                local_map_renderer.move_camera_delta(
                    new_x - original_x,
                    new_y - original_y
                );
                return true;
            }
        }
    }
    
    false
}

/// Handles camera centering (C key)
pub fn handle_center_camera(
    input: &InputManager,
    local_map_renderer: &mut LocalMapRenderer,
    world: &World,
) -> bool {
    if input.key().pressed(KeyCode::C) {
        if let Some(zlevel) = world.z_levels.get(0) {
            // gather all loaded chunk coordinates
            let mut xs: Vec<i32> = zlevel.chunks.keys().map(|(cx, _)| *cx).collect();
            let mut ys: Vec<i32> = zlevel.chunks.keys().map(|(_, cy)| *cy).collect();
            
            if !xs.is_empty() {
                xs.sort_unstable();
                ys.sort_unstable();

                // compute world-unit extents
                let min_cx = xs[0] as f32;
                let max_cx = xs[xs.len()-1] as f32 + 1.0;
                let min_cy = ys[0] as f32;
                let max_cy = ys[ys.len()-1] as f32 + 1.0;

                // Calculate world units per chunk
                let subpx_per_chunk = CHUNK_SIZE * TILE_PX;
                
                // true map center in world-subpixels
                let world_center_x = (min_cx + max_cx) * 0.5 * subpx_per_chunk;
                let world_center_y = (min_cy + max_cy) * 0.5 * subpx_per_chunk;

                // convert screen half-width to world units
                let zoom = local_map_renderer.get_zoom();
                let sw = screen_width();
                let sh = screen_height();
                let half_wu_x = (sw * 0.5) / zoom;
                let half_wu_y = (sh * 0.5) / zoom;

                // desired camera position
                let desired_cam_x = world_center_x - half_wu_x;
                let desired_cam_y = world_center_y - half_wu_y;

                // apply delta
                local_map_renderer.move_camera_delta(
                    desired_cam_x - local_map_renderer.get_camera_x(),
                    desired_cam_y - local_map_renderer.get_camera_y(),
                );
                
                // Enforce boundaries after centering
                enforce_camera_boundaries(local_map_renderer, world);
                return true;
            }
        }
    }
    false
}

/// Handle keyboard-based camera movement (WASD keys)
pub fn handle_keyboard_movement(
    input: &InputManager,
    local_map_renderer: &mut LocalMapRenderer,
    world: &World,
) -> bool {
    let state = &input.state;
    let move_speed = BASE_MOVE_SPEED * get_frame_time();
    let mut movement = false;

    if state.keys_down.contains(&KeyCode::W) { 
        local_map_renderer.move_camera_delta(0.0, -move_speed);
        movement = true;
    }
    if state.keys_down.contains(&KeyCode::S) { 
        local_map_renderer.move_camera_delta(0.0, move_speed); 
        movement = true;
    }
    if state.keys_down.contains(&KeyCode::A) { 
        local_map_renderer.move_camera_delta(-move_speed, 0.0);
        movement = true;
    }
    if state.keys_down.contains(&KeyCode::D) { 
        local_map_renderer.move_camera_delta(move_speed, 0.0);
        movement = true;
    }
    
    // If there was movement, enforce boundaries
    if movement {
        enforce_camera_boundaries(local_map_renderer, world);
    }
    
    movement
}

/// Handle mouse wheel zoom
pub fn handle_zoom(
    input: &InputManager,
    local_map_renderer: &mut LocalMapRenderer,
    world: &World,
) -> bool {
    let state = &input.state;
    let wheel = state.mouse_scroll;
    
    if wheel != 0.0 {
        let old_zoom = local_map_renderer.get_zoom();
        let new_zoom = (old_zoom + wheel * ZOOM_SPEED).clamp(MIN_ZOOM, MAX_ZOOM);
        
        if (new_zoom - old_zoom).abs() > f32::EPSILON {
            // compute world-space point under cursor pre-zoom
            let (mx, my) = state.mouse_position;
            let world_x = local_map_renderer.get_camera_x() + mx / old_zoom;
            let world_y = local_map_renderer.get_camera_y() + my / old_zoom;

            // apply new zoom
            local_map_renderer.set_zoom(new_zoom);

            // recenter camera so (mx,my) stays over (world_x,world_y)
            let new_cam_x = world_x - mx / new_zoom;
            let new_cam_y = world_y - my / new_zoom;
            local_map_renderer.move_camera_delta(
                new_cam_x - local_map_renderer.get_camera_x(),
                new_cam_y - local_map_renderer.get_camera_y(),
            );
            
            // Enforce boundaries after zooming
            enforce_camera_boundaries(local_map_renderer, world);
            return true;
        }
    }
    
    false
}

/// Public utility function to center the camera on a specific point
pub fn center_camera(local_map_renderer: &mut LocalMapRenderer, x: f32, y: f32, world: &World) {
    let zoom = local_map_renderer.get_zoom();
    let sw = screen_width();
    let sh = screen_height();
    
    // Calculate the camera position to center on (x,y)
    let cam_x = x - sw / (2.0 * zoom);
    let cam_y = y - sh / (2.0 * zoom);
    
    // Apply the delta
    local_map_renderer.move_camera_delta(
        cam_x - local_map_renderer.get_camera_x(),
        cam_y - local_map_renderer.get_camera_y(),
    );
    
    // Enforce boundaries after centering
    enforce_camera_boundaries(local_map_renderer, world);
} 