use macroquad::prelude::*;
use crate::game::views::GameView;
use crate::game::input::RenderMode;
use crate::renderer::local_map_renderer::LocalMapRenderer;
use crate::renderer::camera::Camera;
use crate::world::localmap::world::World;
use crate::gui::windows::window_manager::WindowManager;
use crate::world::worldmap::world_map::WorldMap;
use crate::input::manager::InputManager;
use crate::input::event::InputEvent;

// Import view modules with their original names
use crate::game::views::{world_map as view_world_map, local_map as view_local_map};

// Import the new input handler modules with different names
use crate::game::input::local_map as input_local_map;
use crate::game::input::world_map as input_world_map;

pub struct InputHandler {
    previous_mouse_x: f32,
    previous_mouse_y: f32,
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            previous_mouse_x: 0.0,
            previous_mouse_y: 0.0,
        }
    }

    pub fn handle_input(
        &mut self,
        input: &InputManager,
        render_mode: RenderMode,
        active_view: &mut GameView,
        window_manager: &mut WindowManager,
        local_map_renderer: &mut LocalMapRenderer,
        world: &mut World,
        world_map: &WorldMap,
        world_map_camera: &mut Camera,
    ) -> bool {
        // ESC key toggles main menu
        if input.key().pressed(KeyCode::Escape) {
            window_manager.main_menu.toggle_main();
            return true;
        }

        // Switch between WorldMap and LocalMap views
        if input.key().pressed(KeyCode::Tab) {
            *active_view = match *active_view {
                GameView::WorldMap => GameView::LocalMap,
                GameView::LocalMap => GameView::WorldMap,
                _ => GameView::WorldMap,
            };
            return true;
        }

        // Process mode-specific input based on the current render_mode
        let handled = match render_mode {
            RenderMode::LocalMap => {
                // Handle local map input through our local_map module
                input_local_map::handle_input(
                    input,
                    &mut self.previous_mouse_x,
                    &mut self.previous_mouse_y,
                    local_map_renderer,
                    world
                )
            },
            RenderMode::WorldMap => {
                // Handle world map input through our world_map module
                let view_changed = input_world_map::handle_input(
                    input,
                    &mut self.previous_mouse_x,
                    &mut self.previous_mouse_y,
                    world_map_camera, 
                    world_map,
                    window_manager
                );
                if let Some(new_view) = view_changed {
                    *active_view = new_view;
                    return true;
                }
                false
            }
        };

        handled
    }
} 