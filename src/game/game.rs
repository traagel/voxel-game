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
use crate::gui::civ_portraits::CivPortraits;
use macroquad::ui::{hash, root_ui};
use crate::world::worldmap::civilization::Civilization;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RenderMode {
    WorldMap,
    LocalMap,
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
}

impl Game {
    pub async fn new() -> Self {
        let mut gui = GuiState::new();
        let world_map_renderer = WorldMapRenderer::new().await;
        let world_map_gen = WorldMapGenerator::new(42, gui.worldgen_width, gui.worldgen_height, 0.02, None);
        let world_map = world_map_gen.generate();
        let portraits = Some(CivPortraits::load().await);

        // Set a default selected civilization for testing
        gui.selected_civ = Some(Civilization::Human);

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
        if is_key_pressed(KeyCode::Tab) {
            self.render_mode = match self.render_mode {
                RenderMode::WorldMap => RenderMode::LocalMap,
                RenderMode::LocalMap => RenderMode::WorldMap,
            };
        }
    
        match self.render_mode {
                RenderMode::LocalMap => {
                    let move_speed = 200.0 * get_frame_time();
                    let zoom_speed = 0.2;
            
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
                            // compute world‐space point under cursor pre‐zoom
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
        self.gui.update(
            &self.world,
            self.render_mode,
            if self.render_mode == RenderMode::WorldMap { Some(&self.world_map) } else { None },
            if self.render_mode == RenderMode::WorldMap { Some(&self.world_map_camera) } else { None },
            self.portraits.as_ref(),
        );
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
        if self.gui.regenerate_requested {
            let params = self.gui.worldgen_params;
            let world_map_gen = WorldMapGenerator::new(
                self.gui.worldgen_seed,
                self.gui.worldgen_width,
                self.gui.worldgen_height,
                0.02,
                Some(params),
            );
            self.world_map = world_map_gen.generate();
            self.gui.regenerate_requested = false;
        }
    }

    fn render(&mut self) {
        match self.render_mode {
            RenderMode::WorldMap => {
                self.world_map_renderer
                    .draw_world_map_with_view(&self.world_map, &self.world_map_camera, self.gui.map_view, self.world_map.sea_level);
            }
            RenderMode::LocalMap => {
                let state = GameState {
                    camera_x: self.local_map_renderer.get_camera_x(),
                    camera_y: self.local_map_renderer.get_camera_y(),
                    zoom: self.local_map_renderer.get_zoom(),
                    z_levels: &self.world.z_levels,
                };
                self.local_map_renderer.draw(&state);
                self.draw_creatures();
                for p in &self.particles {
                    let sx = (p.x - self.local_map_renderer.get_camera_x()) * self.local_map_renderer.get_zoom();
                    let sy = (p.y - self.local_map_renderer.get_camera_y()) * self.local_map_renderer.get_zoom();
                    draw_circle(sx, sy, 0.2 * self.local_map_renderer.get_camera_y(), YELLOW);
                }
            }
        }
        // Draw only the selected civilization's portrait at the top middle of the screen
        if let (Some(civ), Some(portraits)) = (self.gui.selected_civ, self.portraits.as_ref()) {
            if let Some(src_rect) = portraits.get_portrait_rect(civ) {
                let portrait_size = 96.0;
                let px = (screen_width() - portrait_size) / 2.0;
                let py = 16.0;
                draw_texture_ex(
                    portraits.get_texture(),
                    px,
                    py,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(Vec2::new(portrait_size, portrait_size)),
                        source: Some(src_rect),
                        ..Default::default()
                    },
                );
            }
        }
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

