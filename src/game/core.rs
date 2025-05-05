use crate::creatures::Creature;
use crate::game::state::GameState;
use crate::gui::GuiState;
use crate::particle::Particle;
use crate::player::actions::count_dig_jobs;
use crate::renderer::camera::Camera;
use crate::renderer::local_map_renderer::LocalMapRenderer;
use crate::renderer::world_map_renderer::WorldMapRenderer;
use crate::world::localmap::world::World;
use crate::world::worldmap::world_map::WorldMap;
use crate::worldgen::localmap::builder::WorldGeneratorBuilder;
use crate::worldgen::worldmap::WorldMapGenerator;
use crate::gui::windows::city_info::portraits::CivPortraits;
use crate::gui::windows::window_manager::WindowManager;
use macroquad::prelude::*;

use crate::game::views::GameView;
use crate::game::input::{RenderMode, InputHandler};
use crate::game::entities::{creatures, particles};

pub struct Game {
    world: World,
    local_map_renderer: LocalMapRenderer,
    world_map_renderer: WorldMapRenderer,
    creatures: Vec<Creature>,
    particles: Vec<Particle>,
    gui: GuiState,
    world_map: WorldMap,
    render_mode: RenderMode,
    world_map_camera: Camera,
    input_handler: InputHandler,
    portraits: Option<CivPortraits>,
    window_manager: WindowManager,
    pub active_view: GameView,
}

impl Game {
    pub async fn new() -> Self {
        let gui = GuiState::new();
        let world_map_renderer = WorldMapRenderer::new().await;
        let window_manager = WindowManager::new();
        let world_map_gen = WorldMapGenerator::new(
            42,
            window_manager.worldgen.width,
            window_manager.worldgen.height,
            0.02,
            None,
        );
        let world_map = world_map_gen.generate();
        let portraits = Some(CivPortraits::load().await);

        Self {
            world: World::new(),
            local_map_renderer: LocalMapRenderer::default(),
            world_map_renderer,
            creatures: Vec::new(),
            particles: Vec::new(),
            gui,
            world_map,
            render_mode: RenderMode::WorldMap, // Start in world map mode
            world_map_camera: Camera::default(),
            input_handler: InputHandler::new(),
            portraits,
            window_manager,
            active_view: GameView::WorldMap, // Start in WorldMap view
        }
    }

    pub fn init(&mut self) {
        // Generate the world
        let generator = WorldGeneratorBuilder::new(42).build();
        let area = (-1..=1)
            .flat_map(|x| (-1..=1).map(move |y| (x, y)))
            .collect::<Vec<_>>();
        generator.generate_into_world(&mut self.world, &area);

        // Spawn creatures
        self.creatures = creatures::spawn_creatures(&self.world, 10);
    }

    fn update(&mut self) {
        // Update creatures if in local map mode
        if let RenderMode::LocalMap = self.render_mode {
            creatures::update_creatures(&mut self.creatures, &mut self.world, &mut self.particles);
            particles::update_particles(&mut self.particles);
        }

        // Update GUI
        self.gui.dig_jobs = count_dig_jobs(&self.world);
        self.gui.update(&self.world, self.render_mode);
        
        // World map regeneration logic
        if self.window_manager.worldgen.regenerate_requested {
            let params = self.window_manager.worldgen.params;
            let world_map_gen = WorldMapGenerator::new(
                self.window_manager.worldgen.seed,
                self.window_manager.worldgen.width,
                self.window_manager.worldgen.height,
                0.02,
                Some(params),
            );
            self.world_map = world_map_gen.generate();
            self.window_manager.worldgen.regenerate_requested = false;
        }
    }

    fn render(&mut self) {
        match self.active_view {
            GameView::MainMenu => {
                crate::game::views::main_menu::render(&mut self.window_manager);
            }
            GameView::WorldGen => {
                crate::gui::windows::worldgen::draw_worldgen_window(&mut self.window_manager.worldgen);
            }
            GameView::WorldMap => {
                crate::game::views::world_map::render(
                    &self.world_map,
                    &self.world_map_renderer,
                    &self.world_map_camera,
                    &mut self.window_manager.worldgen,
                );
            }
            GameView::RegionMap => {
                // Simple stub for region map view
                clear_background(DARKGRAY);
                draw_text("[Region Map View - TODO]", 100.0, 100.0, 32.0, WHITE);
            }
            GameView::LocalMap => {
                let state = GameState {
                    camera_x: self.local_map_renderer.get_camera_x(),
                    camera_y: self.local_map_renderer.get_camera_y(),
                    zoom:     self.local_map_renderer.get_zoom(),
                    z_levels: &self.world.z_levels,
                };
                
                crate::game::views::local_map::render(
                    &self.local_map_renderer,
                    &state,
                    &self.creatures,
                    &self.particles,
                    &mut self.window_manager,
                );
            }
            GameView::CityInfo => {
                if let Some(portraits) = self.portraits.as_ref() {
                    if let Some(next_view) = crate::game::views::city_info::render(
                        &self.world_map,
                        &self.world_map_renderer,
                        &self.world_map_camera,
                        &mut self.window_manager,
                        portraits,
                    ) {
                        self.active_view = next_view;
                    }
                } else {
                    // If portraits aren't loaded, go back to world map
                    self.active_view = GameView::WorldMap;
                }
            }
        }
        
        // Reset to default screen-space camera before drawing UI
        set_default_camera();
    }

    pub async fn run(&mut self) {
        loop {
            // Sync render_mode with active_view before handling input
            self.render_mode = match self.active_view {
                GameView::LocalMap => RenderMode::LocalMap,
                GameView::WorldMap => RenderMode::WorldMap,
                _ => self.render_mode, // Keep current mode for other views
            };
            
            // Handle input
            self.input_handler.handle_input(
                self.render_mode,
                &mut self.active_view,
                &mut self.window_manager,
                &mut self.local_map_renderer,
                &mut self.world,
                &self.world_map,
                &mut self.world_map_camera,
            );
            
            self.update();
            self.render();
            next_frame().await;
        }
    }
} 