use crate::creatures::Creature;
use crate::game::game_state::GameState;
use crate::gui::GuiState;
use crate::particle::Particle;
use crate::player::actions::{count_dig_jobs, paint_dig_target, paint_rock};
use crate::renderer::camera::Camera;
use crate::renderer::local_map_renderer::LocalMapRenderer;
use crate::renderer::world_map_renderer::WorldMapRenderer;
use crate::world::localmap::terrain_material::TerrainMaterial;
use crate::world::localmap::world::World;
use crate::world::worldmap::world_map::WorldMap;
use crate::worldgen::localmap::builder::WorldGeneratorBuilder;
use crate::worldgen::worldmap::WorldMapGenerator;
use macroquad::prelude::*;
use macroquad::rand::gen_range;
use macroquad::prelude::{*, screen_width, screen_height};
use crate::gui::windows::city_info::portraits::CivPortraits;
use macroquad::ui::{hash, root_ui};
use crate::world::worldmap::civilization::Civilization;
use crate::gui::windows::main_menu::MainMenuState;
use crate::gui::windows::city_info::city_info_window;
use crate::gui::windows::window_manager::WindowManager;
use crate::gui::windows::worldgen::draw_worldgen_window;
use crate::input::{poll_actions, actions::Action};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RenderMode {
    WorldMap,
    LocalMap,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum GameView {
    MainMenu,
    WorldGen,
    WorldMap,
    LocalMap,
    CityInfo,
    RegionMap,
}

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
    previous_mouse_x: f32,
    previous_mouse_y: f32,
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

        // Set a default selected civilization for testing
        // window_manager.city_info.selected_civ = Some(Civilization::Human); // Uncomment if you want a default

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
            previous_mouse_x: 0.0, // Initialize previous mouse position
            previous_mouse_y: 0.0, // Initialize previous mouse position
            portraits,
            window_manager,
            active_view: GameView::WorldMap, // Start in WorldMap view
        }
    }

    pub fn init(&mut self) {
        let generator = WorldGeneratorBuilder::new(42).build();
        let area = (-1..=1)
            .flat_map(|x| (-1..=1).map(move |y| (x, y)))
            .collect::<Vec<_>>();
        generator.generate_into_world(&mut self.world, &area);

        for _ in 0..10 {
            if let Some((spawn_x, spawn_y)) = Self::find_spawn_point(&self.world) {
                self.creatures
                    .push(Creature::new(spawn_x, spawn_y, 2.0, RED));
            }
        }
    }

    fn find_spawn_point(world: &World) -> Option<(f32, f32)> {
        let tries = 100;
        for _ in 0..tries {
            let x = gen_range(0, 256) as i32;
            let y = gen_range(0, 256) as i32;
            let material = world.get_material_at(x, y);
            if material == TerrainMaterial::Dirt {
                return Some((x as f32, y as f32));
            }
        }
        None
    }

    fn handle_input(&mut self) {
        let move_speed = 200.0 * get_frame_time();
        let zoom_speed = 0.2;
        for action in poll_actions() {
            match action {
                Action::OpenMenu => {
                    self.window_manager.main_menu.toggle_main();
                }
                Action::SwitchView => {
                    self.active_view = match self.active_view {
                        GameView::WorldMap => GameView::LocalMap,
                        GameView::LocalMap => GameView::WorldMap,
                        _ => GameView::WorldMap,
                    };
                }
                Action::CenterCamera => {
                    match self.render_mode {
                        RenderMode::LocalMap => {
                            if let Some(zlevel) = self.world.z_levels.get(0) {
                                let mut xs: Vec<i32> = zlevel.chunks.keys().map(|(cx, _)| *cx).collect();
                                let mut ys: Vec<i32> = zlevel.chunks.keys().map(|(_, cy)| *cy).collect();
                                if !xs.is_empty() {
                                    xs.sort_unstable();
                                    ys.sort_unstable();
                                    const CHUNK_SIZE: f32 = 32.0;
                                    const TILE_PX:     f32 = 8.0;
                                    let subpx_per_chunk = CHUNK_SIZE * TILE_PX;
                                    let min_cx = xs[0]           as f32;
                                    let max_cx = xs[xs.len()-1]  as f32 + 1.0;
                                    let min_cy = ys[0]           as f32;
                                    let max_cy = ys[ys.len()-1]  as f32 + 1.0;
                                    let world_center_x = (min_cx + max_cx) * 0.5 * subpx_per_chunk;
                                    let world_center_y = (min_cy + max_cy) * 0.5 * subpx_per_chunk;
                                    let zoom = self.local_map_renderer.get_zoom();
                                    let sw   = screen_width();
                                    let sh   = screen_height();
                                    let half_wu_x = (sw * 0.5) / zoom;
                                    let half_wu_y = (sh * 0.5) / zoom;
                                    let desired_cam_x = world_center_x - half_wu_x;
                                    let desired_cam_y = world_center_y - half_wu_y;
                                    self.local_map_renderer.move_camera_delta(
                                        desired_cam_x - self.local_map_renderer.get_camera_x(),
                                        desired_cam_y - self.local_map_renderer.get_camera_y(),
                                    );
                                }
                            }
                        }
                        RenderMode::WorldMap => {
                            let w = self.world_map.width  as f32;
                            let h = self.world_map.height as f32;
                            const TILE_PX: f32 = 8.0;
                            let zoom = self.world_map_camera.zoom;
                            let sw   = screen_width();
                            let sh   = screen_height();
                            let half_wu_x = (sw * 0.5) / (TILE_PX * zoom);
                            let half_wu_y = (sh * 0.5) / (TILE_PX * zoom);
                            let desired_cam_x = w * 0.5 - half_wu_x;
                            let desired_cam_y = h * 0.5 - half_wu_y;
                            self.world_map_camera.move_delta(
                                desired_cam_x - self.world_map_camera.x,
                                desired_cam_y - self.world_map_camera.y,
                            );
                        }
                    }
                }
                Action::PanCamera { dx, dy } => {
                    match self.render_mode {
                        RenderMode::LocalMap => {
                            self.local_map_renderer.move_camera_delta(dx * move_speed, dy * move_speed);
                        }
                        RenderMode::WorldMap => {
                            self.world_map_camera.move_delta(dx * move_speed, dy * move_speed);
                        }
                    }
                }
                Action::PaintTile { x, y } => {
                    if let RenderMode::LocalMap = self.render_mode {
                        let world_x = self.local_map_renderer.get_camera_x() + x as f32 / self.local_map_renderer.get_zoom();
                        let world_y = self.local_map_renderer.get_camera_y() + y as f32 / self.local_map_renderer.get_zoom();
                        paint_rock(&mut self.world, world_x as i32, world_y as i32);
                    }
                }
                Action::DigTile { x, y } => {
                    if let RenderMode::LocalMap = self.render_mode {
                        let world_x = self.local_map_renderer.get_camera_x() + x as f32 / self.local_map_renderer.get_zoom();
                        let world_y = self.local_map_renderer.get_camera_y() + y as f32 / self.local_map_renderer.get_zoom();
                        paint_dig_target(&mut self.world, world_x as i32, world_y as i32);
                    }
                }
                Action::Zoom { delta, x, y } => {
                    match self.render_mode {
                        RenderMode::LocalMap => {
                            let old_zoom = self.local_map_renderer.get_zoom();
                            let new_zoom = (old_zoom + delta * zoom_speed).clamp(1.0, 10.0);
                            if (new_zoom - old_zoom).abs() > f32::EPSILON {
                                let world_x = self.local_map_renderer.get_camera_x() + x / old_zoom;
                                let world_y = self.local_map_renderer.get_camera_y() + y / old_zoom;
                                self.local_map_renderer.set_zoom(new_zoom);
                                let new_cam_x = world_x - x / new_zoom;
                                let new_cam_y = world_y - y / new_zoom;
                                self.local_map_renderer.move_camera_delta(
                                    new_cam_x - self.local_map_renderer.get_camera_x(),
                                    new_cam_y - self.local_map_renderer.get_camera_y(),
                                );
                            }
                        }
                        RenderMode::WorldMap => {
                            const TILE_PX: f32 = 8.0;
                            let old_zoom = self.world_map_camera.zoom;
                            let new_zoom = (old_zoom + delta * zoom_speed).clamp(1.0, 10.0);
                            let old_scale = TILE_PX * old_zoom;
                            let world_x = self.world_map_camera.x + x / old_scale;
                            let world_y = self.world_map_camera.y + y / old_scale;
                            self.world_map_camera.set_zoom(new_zoom);
                            let new_scale = TILE_PX * new_zoom;
                            let new_cam_x = world_x - x / new_scale;
                            let new_cam_y = world_y - y / new_scale;
                            self.world_map_camera.move_delta(
                                new_cam_x - self.world_map_camera.x,
                                new_cam_y - self.world_map_camera.y,
                            );
                        }
                    }
                }
                Action::StartDrag { x, y } => {
                    self.previous_mouse_x = x;
                    self.previous_mouse_y = y;
                }
                Action::Drag { x, y, dx, dy } => {
                    match self.render_mode {
                        RenderMode::LocalMap => {
                            let inv_zoom = 1.0 / self.local_map_renderer.get_zoom();
                            self.local_map_renderer.move_camera_delta(-dx * inv_zoom, -dy * inv_zoom);
                        }
                        RenderMode::WorldMap => {
                            const TILE_PX: f32 = 8.0;
                            let inv_scale = 1.0 / (TILE_PX * self.world_map_camera.zoom);
                            self.world_map_camera.move_delta(-dx * inv_scale, -dy * inv_scale);
                        }
                    }
                    self.previous_mouse_x = x;
                    self.previous_mouse_y = y;
                }
                Action::EndDrag => {
                    self.previous_mouse_x = 0.0;
                    self.previous_mouse_y = 0.0;
                }
                Action::CityClick { x, y } => {
                    if let RenderMode::WorldMap = self.render_mode {
                        const TILE_PX: f32 = 8.0;
                        let world_x = self.world_map_camera.x + x / (TILE_PX * self.world_map_camera.zoom);
                        let world_y = self.world_map_camera.y + y / (TILE_PX * self.world_map_camera.zoom);
                        let city_radius = 0.5;
                        if let Some(city) = self.world_map.cities.iter().find(|city| {
                            let dx = city.x as f32 - world_x;
                            let dy = city.y as f32 - world_y;
                            (dx * dx + dy * dy).sqrt() < city_radius
                        }).cloned() {
                            self.window_manager.city_info.selected_city = Some(city);
                            self.window_manager.city_info.show = true;
                            self.active_view = GameView::CityInfo;
                        }
                    }
                }
                _ => {}
            }
        }
    }
    
    
    fn update_creatures(&mut self) {
        if let RenderMode::LocalMap = self.render_mode {
            for creature in &mut self.creatures {
                if creature.target.is_none() {
                    creature.find_nearest_dig_target(&self.world);
                }
                creature.move_toward_target(&mut self.world);
                creature.dig_if_close(&mut self.world, &mut self.particles);
            }
        }
    }

    fn draw_creatures(&self) {
        if let RenderMode::LocalMap = self.render_mode {
            for creature in &self.creatures {
                creature.draw(
                    self.local_map_renderer.get_camera_x(),
                    self.local_map_renderer.get_camera_y(),
                    self.local_map_renderer.get_zoom(),
                );
            }
        }
    }

    fn update(&mut self) {
        self.update_creatures();
        self.gui.dig_jobs = count_dig_jobs(&self.world);
        self.gui.update(&self.world, self.render_mode);
        if let RenderMode::LocalMap = self.render_mode {
            for p in &mut self.particles {
                p.x += p.dx;
                p.y += p.dy;
                p.dy += 0.05;
                p.life = p.life.saturating_sub(1);
            }
            self.particles.retain(|p| p.life > 0);
        }
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
                self.window_manager.main_menu.draw();
            }
            GameView::WorldGen => {
                draw_worldgen_window(&mut self.window_manager.worldgen);
            }
            GameView::WorldMap => {
                // Draw world map and generator settings
                self.world_map_renderer
                    .draw_world_map_with_view(
                        &self.world_map,
                        &self.world_map_camera,
                        crate::renderer::world_map_renderer::MapView::Biome,
                        self.world_map.sea_level,
                    );
                draw_worldgen_window(&mut self.window_manager.worldgen);
            }
            GameView::RegionMap => {
                // TODO: Implement region map rendering
                // For now, just clear background and show a stub label
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
                self.local_map_renderer.draw(&state);
                self.draw_creatures();
                for p in &self.particles {
                    let sx = (p.x - self.local_map_renderer.get_camera_x())
                        * self.local_map_renderer.get_zoom();
                    let sy = (p.y - self.local_map_renderer.get_camera_y())
                        * self.local_map_renderer.get_zoom();
                    draw_circle(sx, sy, 0.2 * self.local_map_renderer.get_zoom(), YELLOW);
                }
                crate::gui::windows::worker_info::draw_worker_info_window(&mut self.window_manager.worker_info);
            }
            GameView::CityInfo => {
                // Draw world map in background
                self.world_map_renderer
                    .draw_world_map_with_view(
                        &self.world_map,
                        &self.world_map_camera,
                        crate::renderer::world_map_renderer::MapView::Biome,
                        self.world_map.sea_level,
                    );
                draw_worldgen_window(&mut self.window_manager.worldgen);
                let city_info_selected = self.window_manager.city_info.selected_city.clone();
                let portraits = self.portraits.as_ref();
                let city_info_state = &mut self.window_manager.city_info;
                if let (Some(city), Some(portraits)) = (city_info_selected.as_ref(), portraits) {
                    crate::gui::windows::city_info::city_info_window(
                        city_info_state,
                        city,
                        portraits,
                        &self.world_map,
                    );
                    // If the city info window was closed, return to WorldMap view
                    if !city_info_state.show {
                        self.active_view = GameView::WorldMap;
                    }
                } else {
                    // If no city is selected, return to WorldMap view
                    self.active_view = GameView::WorldMap;
                }
            }
        }
        // Reset to default screen-space camera before drawing UI
        set_default_camera();
    }

    pub async fn run(&mut self) {
        loop {
            self.handle_input();
            self.update();
            self.render();
            next_frame().await;
        }
    }
}

