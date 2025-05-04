use crate::gui::windows::main_menu::MainMenuState;
use crate::gui::windows::city_info::CityInfoState;
use crate::gui::windows::worldgen::WorldGenWindowState;
use crate::gui::windows::worldgen::draw_worldgen_window;
use crate::gui::windows::worker_info::WorkerInfoState;
use crate::gui::GuiState;
use crate::game::game::RenderMode;
use crate::gui::windows::window_state::WindowState;

pub struct WindowManager {
    pub main_menu: MainMenuState,
    pub city_info: CityInfoState,
    pub worldgen: WorldGenWindowState,
    pub worker_info: WorkerInfoState,
    // Add other window states here as needed
}

impl WindowManager {
    pub fn new() -> Self {
        Self {
            main_menu: MainMenuState::new(),
            city_info: CityInfoState::new(),
            worldgen: WorldGenWindowState::new(),
            worker_info: WorkerInfoState::new(),
            // Initialize other windows here
        }
    }

    /// Draw all windows based on the current game state and render mode
    pub fn draw(
        &mut self,
        render_mode: RenderMode,
        // Add other params as needed (e.g., city, portraits, world_map, etc.)
        city: Option<&crate::world::worldmap::city::City>,
        portraits: Option<&crate::gui::windows::city_info::portraits::CivPortraits>,
        world_map: Option<&crate::world::worldmap::world_map::WorldMap>,
    ) {
        // Draw main menu if open
        self.main_menu.draw();

        // Draw city info window only in WorldMap mode and if visible
        if let (RenderMode::WorldMap, Some(city), Some(portraits), Some(world_map)) = (render_mode, city, portraits, world_map) {
            if self.city_info.is_visible() {
                crate::gui::windows::city_info::city_info_window(
                    &mut self.city_info,
                    city,
                    portraits,
                    world_map,
                );
            }
        }
        // Draw worldgen window only in WorldMap mode
        if let RenderMode::WorldMap = render_mode {
            draw_worldgen_window(&mut self.worldgen);
        }
        // Add logic for other windows as needed
    }
} 