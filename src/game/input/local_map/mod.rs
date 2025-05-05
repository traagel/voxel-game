use macroquad::prelude::*;
use crate::renderer::local_map_renderer::LocalMapRenderer;
use crate::world::localmap::world::World;
use crate::game::views::local_map as view_local_map;
use crate::input::manager::InputManager;
use crate::input::event::InputEvent;

pub fn center_camera(local_map_renderer: &mut LocalMapRenderer, x: f32, y: f32) {
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
}

pub fn handle_input(
    input: &InputManager,
    previous_mouse_x: &mut f32,
    previous_mouse_y: &mut f32,
    local_map_renderer: &mut LocalMapRenderer,
    world: &mut World,
) -> bool {
    let mut input_handled = false;
    let move_speed = 200.0 * get_frame_time();
    let zoom_speed = 0.2;
    let state = input.state();
    let events = input.events();

    // Center map on 'C'
    if events.iter().any(|e| matches!(e, InputEvent::KeyDown(KeyCode::C))) {
        if let Some(zlevel) = world.z_levels.get(0) {
            // gather all loaded chunk coordinates
            let mut xs: Vec<i32> = zlevel.chunks.keys().map(|(cx, _)| *cx).collect();
            let mut ys: Vec<i32> = zlevel.chunks.keys().map(|(_, cy)| *cy).collect();
            if !xs.is_empty() {
                xs.sort_unstable();
                ys.sort_unstable();

                // compute world-unit extents
                let min_cx = xs[0]           as f32;
                let max_cx = xs[xs.len()-1]  as f32 + 1.0;
                let min_cy = ys[0]           as f32;
                let max_cy = ys[ys.len()-1]  as f32 + 1.0;

                // each chunk is 32 tiles × 8 px per tile
                const CHUNK_SIZE: f32 = 32.0;
                const TILE_PX:     f32 = 8.0;
                let subpx_per_chunk = CHUNK_SIZE * TILE_PX;

                // true map center in world-subpixels
                let world_center_x = (min_cx + max_cx) * 0.5 * subpx_per_chunk;
                let world_center_y = (min_cy + max_cy) * 0.5 * subpx_per_chunk;

                // convert screen half-width to world units
                let zoom = local_map_renderer.get_zoom();
                let sw   = screen_width();
                let sh   = screen_height();
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
                input_handled = true;
            }
        }
    }

    // WASD pan
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
    
    if movement {
        input_handled = true;
    }
    
    // Zoom around cursor
    let wheel = state.mouse_scroll;
    if wheel != 0.0 {
        let old_zoom = local_map_renderer.get_zoom();
        let new_zoom = (old_zoom + wheel * zoom_speed).clamp(1.0, 10.0);
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
            input_handled = true;
        }
    }

    // Start drag
    if events.iter().any(|e| matches!(e, InputEvent::MouseDown(MouseButton::Middle))) {
        let (mx, my) = state.mouse_position;
        *previous_mouse_x = mx;
        *previous_mouse_y = my;
        input_handled = true;
    }
    
    // Continue drag (1:1)
    if state.mouse_buttons.contains(&MouseButton::Middle) {
        let (mx, my) = state.mouse_position;
        let dx = mx - *previous_mouse_x;
        let dy = my - *previous_mouse_y;

        // divide by zoom so 1px mouse = 1px world
        let inv_zoom = 1.0 / local_map_renderer.get_zoom();
        local_map_renderer
            .move_camera_delta(-dx * inv_zoom, -dy * inv_zoom);

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

    // Logic for painting and digging with mouse buttons
    let (mouse_x, mouse_y) = state.mouse_position;
    let mouse_left = state.mouse_buttons.contains(&MouseButton::Left);
    let mouse_right = state.mouse_buttons.contains(&MouseButton::Right);
    
    if mouse_left || mouse_right {
        view_local_map::paint_with_mouse(
            world,
            local_map_renderer,
            mouse_x,
            mouse_y,
            mouse_left,
            mouse_right
        );
        input_handled = true;
    }
    
    input_handled
} 