use crate::input::event::InputEvent;
use macroquad::prelude::*;
use crate::renderer::camera::Camera;
use crate::world::worldmap::world_map::WorldMap;
use crate::gui::windows::window_manager::WindowManager;
use crate::game::views::{GameView, world_map as view_world_map};
use crate::input::manager::InputManager;

pub fn handle_input(
    input: &InputManager,
    previous_mouse_x: &mut f32,
    previous_mouse_y: &mut f32,
    world_map_camera: &mut Camera,
    world_map: &WorldMap,
    window_manager: &mut WindowManager,
) -> Option<GameView> {
    let mut input_handled = false;
    let mut view_change = None;
    let move_speed = 200.0 * get_frame_time();
    let zoom_speed = 0.2;
    let state = input.state();
    let events = input.events();

    // Center map on 'C'
    if events.iter().any(|e| matches!(e, InputEvent::KeyDown(KeyCode::C))) {
        let w = world_map.width as f32;
        let h = world_map.height as f32;
        const TILE_PX: f32 = 8.0;
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
        input_handled = true;
    }

    // WASD pan
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
    if movement {
        input_handled = true;
    }

    // Zoom around cursor (8px tiles)
    let wheel = state.mouse_scroll;
    if wheel != 0.0 {
        const TILE_PX: f32 = 8.0;
        let old_zoom = world_map_camera.zoom;
        let new_zoom = (old_zoom + wheel * zoom_speed).clamp(1.0, 10.0);
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
        input_handled = true;
    }

    // Start drag
    if events.iter().any(|e| matches!(e, InputEvent::MouseDown(MouseButton::Middle))) {
        let (mx, my) = state.mouse_position;
        *previous_mouse_x = mx;
        *previous_mouse_y = my;
        input_handled = true;
    }

    // Continue drag
    if state.mouse_buttons.contains(&MouseButton::Middle) {
        let (mx, my) = state.mouse_position;
        let dx = mx - *previous_mouse_x;
        let dy = my - *previous_mouse_y;
        const TILE_PX: f32 = 8.0;
        let inv_scale = 1.0 / (TILE_PX * world_map_camera.zoom);
        world_map_camera.move_delta(-dx * inv_scale, -dy * inv_scale);
        *previous_mouse_x = mx;
        *previous_mouse_y = my;
        input_handled = true;
    }

    // End drag
    if events.iter().any(|e| matches!(e, InputEvent::MouseUp(MouseButton::Middle))) {
        *previous_mouse_x = 0.0;
        *previous_mouse_y = 0.0;
        input_handled = true;
    }

    // City click
    if events.iter().any(|e| matches!(e, InputEvent::MouseDown(MouseButton::Left))) {
        let (mx, my) = state.mouse_position;
        view_change = view_world_map::handle_city_click(
            world_map,
            world_map_camera,
            mx,
            my,
            window_manager,
        );
        input_handled = true;
    }

    view_change
}
