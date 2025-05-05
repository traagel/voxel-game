use macroquad::prelude::*;
use crate::renderer::local_map_renderer::LocalMapRenderer;
use crate::world::localmap::world::World;
use crate::input::manager::InputManager;
use crate::input::event::InputEvent;
use crate::game::views::local_map as view_local_map;
use super::camera_controls::enforce_camera_boundaries;

/// Stores the drag state for middle mouse button camera control
pub struct DragState {
    pub previous_x: f32,
    pub previous_y: f32,
}

impl Default for DragState {
    fn default() -> Self {
        Self {
            previous_x: 0.0,
            previous_y: 0.0,
        }
    }
}

/// Handles the start of camera dragging with middle mouse button
pub fn handle_drag_start(
    input: &InputManager,
    drag_state: &mut DragState,
) -> bool {
    if input.mouse().pressed(MouseButton::Middle) {
        let (mx, my) = input.state.mouse_position;
        drag_state.previous_x = mx;
        drag_state.previous_y = my;
        return true;
    }
    false
}

/// Handles camera dragging with middle mouse button
pub fn handle_drag_movement(
    input: &InputManager,
    drag_state: &mut DragState,
    local_map_renderer: &mut LocalMapRenderer,
    world: &World,
) -> bool {
    if input.mouse().held(MouseButton::Middle) {
        let (mx, my) = input.state.mouse_position;
        let dx = mx - drag_state.previous_x;
        let dy = my - drag_state.previous_y;

        // divide by zoom so 1px mouse = 1px world
        let inv_zoom = 1.0 / local_map_renderer.get_zoom();
        local_map_renderer.move_camera_delta(-dx * inv_zoom, -dy * inv_zoom);
        
        // Enforce boundaries after drag movement
        enforce_camera_boundaries(local_map_renderer, world);

        drag_state.previous_x = mx;
        drag_state.previous_y = my;
        return true;
    }
    false
}

/// Handles the end of camera dragging with middle mouse button
pub fn handle_drag_end(
    input: &InputManager,
    drag_state: &mut DragState,
) -> bool {
    if input.mouse().released(MouseButton::Middle) {
        drag_state.previous_x = 0.0;
        drag_state.previous_y = 0.0;
        return true;
    }
    false
}

/// Handles mouse-based painting and digging
pub fn handle_mouse_painting(
    input: &InputManager,
    local_map_renderer: &mut LocalMapRenderer,
    world: &mut World,
) -> bool {
    let (mouse_x, mouse_y) = input.state.mouse_position;
    let mouse_left = input.mouse().held(MouseButton::Left);
    let mouse_right = input.mouse().held(MouseButton::Right);
    
    if mouse_left || mouse_right {
        view_local_map::paint_with_mouse(
            world,
            local_map_renderer,
            mouse_x,
            mouse_y,
            mouse_left,
            mouse_right
        );
        return true;
    }
    
    false
} 