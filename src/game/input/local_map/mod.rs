mod camera_controls;
mod mouse_controls;
mod constants;

use macroquad::prelude::*;
use crate::renderer::local_map_renderer::LocalMapRenderer;
use crate::world::localmap::world::World;
use crate::input::manager::InputManager;
use crate::input::event::InputEvent;

use camera_controls::{
    handle_center_camera,
    handle_keyboard_movement,
    handle_zoom,
    enforce_camera_boundaries,
    center_camera as internal_center_camera,
};
use mouse_controls::{
    DragState,
    handle_drag_start,
    handle_drag_movement,
    handle_drag_end,
    handle_mouse_painting,
};

/// Stores the state for local map input handling
pub struct LocalMapInputState {
    drag_state: DragState,
}

impl Default for LocalMapInputState {
    fn default() -> Self {
        Self {
            drag_state: DragState::default(),
        }
    }
}

/// Public utility function to center the camera on a specific point
pub fn center_camera(local_map_renderer: &mut LocalMapRenderer, x: f32, y: f32, world: &World) {
    internal_center_camera(local_map_renderer, x, y, world);
}

/// Handles all input for the local map view
/// 
/// Returns true if input was handled
pub fn handle_input(
    input: &InputManager,
    previous_mouse_x: &mut f32,
    previous_mouse_y: &mut f32,
    local_map_renderer: &mut LocalMapRenderer,
    world: &mut World,
) -> bool {
    // Migrate to using proper state management
    let mut drag_state = DragState {
        previous_x: *previous_mouse_x,
        previous_y: *previous_mouse_y,
    };
    
    // Always enforce camera boundaries at the start
    // This handles cases like window resizing or initial loading
    enforce_camera_boundaries(local_map_renderer, world);
    
    let events = &input.events;
    
    // Process input by priority
    let mut handled = false;
    
    // 1. Check for center camera input
    if !handled {
        handled = handle_center_camera(input, local_map_renderer, world);
    }
    
    // 2. Check for keyboard movement
    if !handled {
        handled = handle_keyboard_movement(input, local_map_renderer, world);
    }
    
    // 3. Check for zooming
    if !handled {
        handled = handle_zoom(input, local_map_renderer, world);
    }
    
    // 4. Check for drag input
    if !handled {
        handled = handle_drag_start(input, &mut drag_state);
    }
    
    // 5. Process drag movement
    if !handled {
        handled = handle_drag_movement(input, &mut drag_state, local_map_renderer, world);
    }
    
    // 6. Check for drag end
    if !handled {
        handled = handle_drag_end(input, &mut drag_state);
    }
    
    // 7. Check for mouse painting/digging
    if !handled {
        handled = handle_mouse_painting(input, local_map_renderer, world);
    }
    
    // Final boundary check to ensure the camera is always within bounds
    enforce_camera_boundaries(local_map_renderer, world);
    
    // Update the previous mouse position for next frame
    *previous_mouse_x = drag_state.previous_x;
    *previous_mouse_y = drag_state.previous_y;
    
    handled
} 