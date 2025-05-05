use crate::game::state::GameState;
use crate::renderer::local_map_renderer::LocalMapRenderer;
use crate::world::localmap::world::World;
use crate::particle::Particle;
use macroquad::prelude::*;

pub fn render(
    local_map_renderer: &LocalMapRenderer,
    state: &GameState,
    creatures: &[crate::creatures::Creature],
    particles: &[Particle],
    window_manager: &mut crate::gui::windows::window_manager::WindowManager,
) {
    // Render local map
    local_map_renderer.draw(state);
    
    // Draw creatures
    for creature in creatures {
        creature.draw(
            local_map_renderer.get_camera_x(),
            local_map_renderer.get_camera_y(),
            local_map_renderer.get_zoom(),
        );
    }
    
    // Draw particles
    for p in particles {
        let sx = (p.x - local_map_renderer.get_camera_x())
            * local_map_renderer.get_zoom();
        let sy = (p.y - local_map_renderer.get_camera_y())
            * local_map_renderer.get_zoom();
        draw_circle(sx, sy, 0.2 * local_map_renderer.get_zoom(), YELLOW);
    }
    
    // Draw worker info window
    crate::gui::windows::worker_info::draw_worker_info_window(&mut window_manager.worker_info);
}

pub fn paint_with_mouse(
    world: &mut World,
    local_map_renderer: &LocalMapRenderer,
    mouse_x: f32,
    mouse_y: f32,
    left_button: bool,
    right_button: bool,
) {
    let mouse_world_x = local_map_renderer.get_camera_x() + mouse_x / local_map_renderer.get_zoom();
    let mouse_world_y = local_map_renderer.get_camera_y() + mouse_y / local_map_renderer.get_zoom();

    if left_button {
        crate::player::actions::paint_rock(world, mouse_world_x as i32, mouse_world_y as i32);
    }
    if right_button {
        crate::player::actions::paint_dig_target(world, mouse_world_x as i32, mouse_world_y as i32);
    }
} 