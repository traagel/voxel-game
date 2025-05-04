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
        // ESC key toggles main menu
        if is_key_pressed(KeyCode::Escape) {
            self.window_manager.main_menu.toggle_main();
        }

        // Switch between WorldMap and LocalMap views
        if is_key_pressed(KeyCode::Tab) {
            self.active_view = match self.active_view {
                GameView::WorldMap => GameView::LocalMap,
                GameView::LocalMap => GameView::WorldMap,
                _ => GameView::WorldMap,
            };
        }
    
        match self.render_mode {
                RenderMode::LocalMap => {
                    let move_speed = 200.0 * get_frame_time();
                    let zoom_speed = 0.2;

                    // Center map on 'C'
                    if is_key_pressed(KeyCode::C) {
                        if let Some(zlevel) = self.world.z_levels.get(0) {
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
                                let zoom = self.local_map_renderer.get_zoom();
                                let sw   = screen_width();
                                let sh   = screen_height();
                                let half_wu_x = (sw * 0.5) / zoom;
                                let half_wu_y = (sh * 0.5) / zoom;
        
                                // desired camera position
                                let desired_cam_x = world_center_x - half_wu_x;
                                let desired_cam_y = world_center_y - half_wu_y;
        
                                // apply delta
                                self.local_map_renderer.move_camera_delta(
                                    desired_cam_x - self.local_map_renderer.get_camera_x(),
                                    desired_cam_y - self.local_map_renderer.get_camera_y(),
                                );
                            }
                        }
                    }
        
                    // WASD pan
                    if is_key_down(KeyCode::W) { self.local_map_renderer.move_camera_delta(0.0, -move_speed); }
                    if is_key_down(KeyCode::S) { self.local_map_renderer.move_camera_delta(0.0,  move_speed); }
                    if is_key_down(KeyCode::A) { self.local_map_renderer.move_camera_delta(-move_speed, 0.0); }
                    if is_key_down(KeyCode::D) { self.local_map_renderer.move_camera_delta( move_speed, 0.0); }            
                    // Zoom around cursor
                    let wheel = mouse_wheel().1;
                    if wheel != 0.0 {
                        let old_zoom = self.local_map_renderer.get_zoom();
                        let new_zoom = (old_zoom + wheel * zoom_speed).clamp(1.0, 10.0);
                        if (new_zoom - old_zoom).abs() > f32::EPSILON {
                            // compute world-space point under cursor pre-zoom
                            let (mx, my) = mouse_position();
                            let world_x = self.local_map_renderer.get_camera_x() + mx / old_zoom;
                            let world_y = self.local_map_renderer.get_camera_y() + my / old_zoom;
            
                            // apply new zoom
                            self.local_map_renderer.set_zoom(new_zoom);
            
                            // recenter camera so (mx,my) stays over (world_x,world_y)
                            let new_cam_x = world_x - mx / new_zoom;
                            let new_cam_y = world_y - my / new_zoom;
                            self.local_map_renderer.move_camera_delta(
                                new_cam_x - self.local_map_renderer.get_camera_x(),
                                new_cam_y - self.local_map_renderer.get_camera_y(),
                            );
                        }
                    }
            
                    // Start drag
                    if is_mouse_button_pressed(MouseButton::Middle) {
                        let (mx, my) = mouse_position();
                        self.previous_mouse_x = mx;
                        self.previous_mouse_y = my;
                    }
                    // Continue drag (1:1)
                    if is_mouse_button_down(MouseButton::Middle) {
                        let (mx, my) = mouse_position();
                        let dx = mx - self.previous_mouse_x;
                        let dy = my - self.previous_mouse_y;
            
                        // divide by zoom so 1px mouse = 1px world
                        let inv_zoom = 1.0 / self.local_map_renderer.get_zoom();
                        self.local_map_renderer
                            .move_camera_delta(-dx * inv_zoom, -dy * inv_zoom);
            
                        self.previous_mouse_x = mx;
                        self.previous_mouse_y = my;
                    }
                    // End drag
                    if is_mouse_button_released(MouseButton::Middle) {
                        self.previous_mouse_x = 0.0;
                        self.previous_mouse_y = 0.0;
                    }

                // Logic for painting and digging with mouse buttons
                let mouse_pos = mouse_position();
                let mouse_world_x = self.local_map_renderer.get_camera_x() + mouse_pos.0 / self.local_map_renderer.get_zoom();
                let mouse_world_y = self.local_map_renderer.get_camera_y() + mouse_pos.1 / self.local_map_renderer.get_zoom();
            
                if is_mouse_button_down(MouseButton::Left) {
                    paint_rock(&mut self.world, mouse_world_x as i32, mouse_world_y as i32);
                }
                if is_mouse_button_down(MouseButton::Right) {
                    paint_dig_target(&mut self.world, mouse_world_x as i32, mouse_world_y as i32);
                }
            }
                RenderMode::WorldMap => {
                    let move_speed = 200.0 * get_frame_time();
                    let zoom_speed = 0.2;

                    // Center map on 'C'
                    if is_key_pressed(KeyCode::C) {
                        // map dims in tiles
                        let w = self.world_map.width  as f32;
                        let h = self.world_map.height as f32;
                        const TILE_PX: f32 = 8.0;
                        let zoom = self.world_map_camera.zoom;
                        // screen size
                        let sw   = screen_width();
                        let sh   = screen_height();
                        // half-screen in world-units
                        let half_wu_x = (sw * 0.5) / (TILE_PX * zoom);
                        let half_wu_y = (sh * 0.5) / (TILE_PX * zoom);
                        // desired camera position
                        let desired_cam_x = w * 0.5 - half_wu_x;
                        let desired_cam_y = h * 0.5 - half_wu_y;
        
                        self.world_map_camera.move_delta(
                            desired_cam_x - self.world_map_camera.x,
                            desired_cam_y - self.world_map_camera.y,
                        );
                    }
        
                    // WASD pan
                    if is_key_down(KeyCode::W) { self.world_map_camera.move_delta(0.0, -move_speed); }
                    if is_key_down(KeyCode::S) { self.world_map_camera.move_delta(0.0,  move_speed); }
                    if is_key_down(KeyCode::A) { self.world_map_camera.move_delta(-move_speed, 0.0); }
                    if is_key_down(KeyCode::D) { self.world_map_camera.move_delta( move_speed, 0.0); }

                    // Zoom around cursor (account for 8px tiles)
                    let wheel = mouse_wheel().1;
                    if wheel != 0.0 {
                        const TILE_PX: f32 = 8.0;
                        let old_zoom = self.world_map_camera.zoom;
                        let new_zoom = (old_zoom + wheel * zoom_speed).clamp(1.0, 10.0);
            
                        // old/full scale in px-per-world-unit
                        let old_scale = TILE_PX * old_zoom;
                        let (mx, my) = mouse_position();
                        let world_x = self.world_map_camera.x + mx / old_scale;
                        let world_y = self.world_map_camera.y + my / old_scale;
            
                        // apply new zoom
                        self.world_map_camera.set_zoom(new_zoom);
                        let new_scale = TILE_PX * new_zoom;
            
                        // recenter
                        let new_cam_x = world_x - mx / new_scale;
                        let new_cam_y = world_y - my / new_scale;
                        self.world_map_camera.move_delta(
                            new_cam_x - self.world_map_camera.x,
                            new_cam_y - self.world_map_camera.y,
                        );
                    }
            
                    // Start drag
                    if is_mouse_button_pressed(MouseButton::Middle) {
                        let (mx, my) = mouse_position();
                        self.previous_mouse_x = mx;
                        self.previous_mouse_y = my;
                    }
                    // Continue drag
                    if is_mouse_button_down(MouseButton::Middle) {
                        let (mx, my) = mouse_position();
                        let dx = mx - self.previous_mouse_x;
                        let dy = my - self.previous_mouse_y;
            
                        const TILE_PX: f32 = 8.0;
                        let inv_scale = 1.0 / (TILE_PX * self.world_map_camera.zoom);
                        self.world_map_camera
                            .move_delta(-dx * inv_scale, -dy * inv_scale);
            
                        self.previous_mouse_x = mx;
                        self.previous_mouse_y = my;
                    }
                    // End drag
                    if is_mouse_button_released(MouseButton::Middle) {
                        self.previous_mouse_x = 0.0;
                        self.previous_mouse_y = 0.0;
                    }

                    // --- City click handling ---
                    if is_mouse_button_pressed(MouseButton::Left) {
                        const TILE_PX: f32 = 8.0;
                        let (mx, my) = mouse_position();
                        let world_x = self.world_map_camera.x + mx / (TILE_PX * self.world_map_camera.zoom);
                        let world_y = self.world_map_camera.y + my / (TILE_PX * self.world_map_camera.zoom);
                        // Find city under cursor (within a radius)
                        let city_radius = 0.5; // in world units
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

