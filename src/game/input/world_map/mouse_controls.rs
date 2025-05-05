use macroquad::prelude::*;
use crate::renderer::camera::Camera;
use crate::input::manager::InputManager;
use crate::input::event::InputEvent;
use crate::world::worldmap::world_map::WorldMap;
use crate::gui::windows::window_manager::WindowManager;
use crate::game::views::GameView;
use crate::game::views::world_map as view_world_map;
use super::constants::*;
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
    world_map_camera: &mut Camera,
    world_map: &WorldMap,
) -> bool {
    if input.mouse().held(MouseButton::Middle) {
        let (mx, my) = input.state.mouse_position;
        let dx = mx - drag_state.previous_x;
        let dy = my - drag_state.previous_y;
        let inv_scale = 1.0 / (TILE_PX * world_map_camera.zoom);
        world_map_camera.move_delta(-dx * inv_scale, -dy * inv_scale);
        
        // Enforce boundaries after drag movement
        enforce_camera_boundaries(world_map_camera, world_map);
        
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

/// Handles left mouse click on cities
pub fn handle_city_click(
    input: &InputManager,
    world_map: &WorldMap,
    world_map_camera: &Camera,
    window_manager: &mut WindowManager,
) -> Option<GameView> {
    if input.mouse().pressed(MouseButton::Left) {
        let (mx, my) = input.state.mouse_position;
        return view_world_map::handle_city_click(
            world_map,
            world_map_camera,
            mx,
            my,
            window_manager,
        );
    }
    None
} 