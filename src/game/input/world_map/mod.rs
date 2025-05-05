mod camera_controls;
mod mouse_controls;
mod constants;

use crate::input::event::InputEvent;
use macroquad::prelude::*;
use crate::renderer::camera::Camera;
use crate::world::worldmap::world_map::WorldMap;
use crate::gui::windows::window_manager::WindowManager;
use crate::game::views::{GameView, world_map as view_world_map};
use crate::input::manager::InputManager;

use camera_controls::{
    handle_center_camera,
    handle_keyboard_movement,
    handle_zoom,
    enforce_camera_boundaries,
};
use mouse_controls::{
    DragState,
    handle_drag_start,
    handle_drag_movement,
    handle_drag_end,
    handle_city_click,
};

/// Stores the state for world map input handling
pub struct WorldMapInputState {
    drag_state: DragState,
}

impl Default for WorldMapInputState {
    fn default() -> Self {
        Self {
            drag_state: DragState::default(),
        }
    }
}

/// Handles all input for the world map view
/// 
/// Returns a GameView Option if a view change is requested
pub fn handle_input(
    input: &InputManager,
    previous_mouse_x: &mut f32,
    previous_mouse_y: &mut f32,
    world_map_camera: &mut Camera,
    world_map: &WorldMap,
    window_manager: &mut WindowManager,
) -> Option<GameView> {
    // Migrate to using proper state management
    let mut drag_state = DragState {
        previous_x: *previous_mouse_x,
        previous_y: *previous_mouse_y,
    };
    
    // Always enforce camera boundaries at the start
    // This handles cases like window resizing or initial loading
    enforce_camera_boundaries(world_map_camera, world_map);
    
    let mut view_change = None;

    // Process input by priority
    let mut handled = false;
    
    // 1. Check for center camera input
    if !handled {
        handled = handle_center_camera(input, world_map_camera, world_map);
    }
    
    // 2. Check for keyboard movement
    if !handled {
        handled = handle_keyboard_movement(input, world_map_camera, world_map);
    }
    
    // 3. Check for zooming
    if !handled {
        handled = handle_zoom(input, world_map_camera, world_map);
    }
    
    // 4. Check for drag input
    if !handled {
        handled = handle_drag_start(input, &mut drag_state);
    }
    
    // 5. Process drag movement
    if !handled {
        handled = handle_drag_movement(input, &mut drag_state, world_map_camera, world_map);
    }
    
    // 6. Check for drag end
    if !handled {
        handled = handle_drag_end(input, &mut drag_state);
    }
    
    // 7. Check for city clicks
    if !handled {
        view_change = handle_city_click(input, world_map, world_map_camera, window_manager);
    }
    
    // Final boundary check to ensure the camera is always within bounds
    enforce_camera_boundaries(world_map_camera, world_map);
    
    // Update the previous mouse position for next frame
    *previous_mouse_x = drag_state.previous_x;
    *previous_mouse_y = drag_state.previous_y;
    
    view_change
}
