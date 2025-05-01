use macroquad::prelude::*;
use crate::renderer::renderer::Renderer;
use crate::renderer::camera::Camera;
use crate::world::world::World;
use crate::worldgen::generator::WorldGenerator;
use crate::worldgen::builder::WorldGeneratorBuilder;
use crate::worldgen::world_map::{WorldMap, WorldMapGenerator};
use crate::creatures::Creature;
use crate::particle::Particle;
use crate::gui::GuiState;
use crate::player::actions::{paint_dig_target, paint_rock, count_dig_jobs};
use crate::world::terrain_material::TerrainMaterial;
use macroquad::rand::gen_range;
use crate::game::game_state::GameState;

pub enum RenderMode {
    WorldMap,
    LocalMap,
}

pub struct Game {
    world: World,
    renderer: Renderer,
    creatures: Vec<Creature>,
    particles: Vec<Particle>,
    gui: GuiState,
    world_map: WorldMap,
    render_mode: RenderMode,
    world_map_camera: Camera,
}

impl Game {
    pub fn new() -> Self {
        let world_map_gen = WorldMapGenerator::new(42, 128, 128, 0.02);
        let world_map = world_map_gen.generate();
        Self {
            world: World::new(),
            renderer: Renderer::default(),
            creatures: Vec::new(),
            particles: Vec::new(),
            gui: GuiState::new(),
            world_map,
            render_mode: RenderMode::WorldMap, // Start in world map mode
            world_map_camera: Camera::default(),
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
                self.creatures.push(Creature::new(spawn_x, spawn_y, 2.0, RED));
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
                let zoom_speed = 0.1;

                if is_key_down(KeyCode::W) {
                    self.renderer.move_camera_delta(0.0, -move_speed);
                }
                if is_key_down(KeyCode::S) {
                    self.renderer.move_camera_delta(0.0, move_speed);
                }
                if is_key_down(KeyCode::A) {
                    self.renderer.move_camera_delta(-move_speed, 0.0);
                }
                if is_key_down(KeyCode::D) {
                    self.renderer.move_camera_delta(move_speed, 0.0);
                }

                let wheel = mouse_wheel().1;
                if wheel != 0.0 {
                    let old_zoom = self.renderer.get_zoom();
                    let new_zoom = (old_zoom + wheel * zoom_speed).clamp(1.0, 10.0);
                    if (new_zoom - old_zoom).abs() > f32::EPSILON {
                        let mouse = mouse_position();
                        let mouse_x = mouse.0;
                        let mouse_y = mouse.1;
                        let world_x = self.renderer.get_camera_x() + mouse_x / old_zoom;
                        let world_y = self.renderer.get_camera_y() + mouse_y / old_zoom;
                        self.renderer.set_zoom(new_zoom);
                        let new_screen_x = (world_x - self.renderer.get_camera_x()) * new_zoom;
                        let new_screen_y = (world_y - self.renderer.get_camera_y()) * new_zoom;
                        let camera_x = self.renderer.get_camera_x() + (mouse_x - new_screen_x) / new_zoom;
                        let camera_y = self.renderer.get_camera_y() + (mouse_y - new_screen_y) / new_zoom;
                        self.renderer.move_camera_delta(camera_x - self.renderer.get_camera_x(), camera_y - self.renderer.get_camera_y());
                    }
                }

                let mouse_pos = mouse_position();
                let mouse_world_x = self.renderer.get_camera_x() + mouse_pos.0 / self.renderer.get_zoom();
                let mouse_world_y = self.renderer.get_camera_y() + mouse_pos.1 / self.renderer.get_zoom();

                if is_mouse_button_down(MouseButton::Left) {
                    paint_rock(&mut self.world, mouse_world_x as i32, mouse_world_y as i32);
                }
                if is_mouse_button_down(MouseButton::Right) {
                    let mouse_pos = mouse_position();
                    let world_x = self.renderer.get_camera_x() + mouse_pos.0 / self.renderer.get_zoom();
                    let world_y = self.renderer.get_camera_y() + mouse_pos.1 / self.renderer.get_zoom();
                    paint_dig_target(&mut self.world, world_x as i32, world_y as i32);
                }
            }
            RenderMode::WorldMap => {
                let move_speed = 200.0 * get_frame_time();
                let zoom_speed = 0.1;

                if is_key_down(KeyCode::W) {
                    self.world_map_camera.move_delta(0.0, -move_speed);
                }
                if is_key_down(KeyCode::S) {
                    self.world_map_camera.move_delta(0.0, move_speed);
                }
                if is_key_down(KeyCode::A) {
                    self.world_map_camera.move_delta(-move_speed, 0.0);
                }
                if is_key_down(KeyCode::D) {
                    self.world_map_camera.move_delta(move_speed, 0.0);
                }

                let wheel = mouse_wheel().1;
                if wheel != 0.0 {
                    let old_zoom = self.world_map_camera.zoom;
                    let new_zoom = (old_zoom + wheel * zoom_speed).clamp(1.0, 10.0);
                    self.world_map_camera.set_zoom(new_zoom);
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
                creature.draw(self.renderer.get_camera_x(), self.renderer.get_camera_y(), self.renderer.get_zoom());
            }
        }
    }

    fn update(&mut self) {
        self.update_creatures();
        if let RenderMode::LocalMap = self.render_mode {
            self.gui.dig_jobs = count_dig_jobs(&self.world);
            self.gui.update(&self.world);
            for p in &mut self.particles {
                p.x += p.dx;
                p.y += p.dy;
                p.dy += 0.05;
                p.life = p.life.saturating_sub(1);
            }
            self.particles.retain(|p| p.life > 0);
        }
    }

    fn render(&mut self) {
        match self.render_mode {
            RenderMode::WorldMap => {
                self.renderer.draw_world_map(&self.world_map, &self.world_map_camera);
            }
            RenderMode::LocalMap => {
                let state = GameState {
                    camera_x: self.renderer.get_camera_x(),
                    camera_y: self.renderer.get_camera_y(),
                    zoom: self.renderer.get_zoom(),
                    z_levels: &self.world.z_levels,
                };
                self.renderer.draw(&state);
                self.draw_creatures();
                for p in &self.particles {
                    let sx = (p.x - self.renderer.get_camera_x()) * self.renderer.get_zoom();
                    let sy = (p.y - self.renderer.get_camera_y()) * self.renderer.get_zoom();
                    draw_circle(sx, sy, 0.2 * self.renderer.get_camera_y(), YELLOW);
                }
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