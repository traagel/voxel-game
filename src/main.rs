mod creature;
pub mod particle;
mod renderer;
mod world;
mod worldgen;

use macroquad::prelude::*;
use renderer::renderer::Renderer;
use world::world::World;
use worldgen::builder::WorldGeneratorBuilder;
use worldgen::generator::WorldGenerator;

use creature::creature::Creature;

use crate::world::terrain_material::TerrainMaterial;
use macroquad::rand::gen_range;

mod gui;
use gui::GuiState;

fn find_spawn_point(world: &World) -> Option<(f32, f32)> {
    let tries = 100;

    for _ in 0..tries {
        // Pick random world coordinates
        let x = gen_range(0, 256) as i32; // adjust depending on your world size
        let y = gen_range(0, 256) as i32;

        let material = world.get_material_at(x, y);

        if material == TerrainMaterial::Dirt {
            return Some((x as f32, y as f32));
        }
    }

    None
}

#[macroquad::main("Voxel Engine")]
async fn main() {
    let mut world = World::new();
    let mut renderer = Renderer::default(); // Make mutable so we can move/zoom

    let generator = WorldGenerator::new(42);
    let area = (-1..=1)
        .flat_map(|x| (-1..=1).map(move |y| (x, y)))
        .collect::<Vec<_>>();
    generator.generate_into_world(&mut world, &area);

    let mut creatures = Vec::new();
    let mut particles = Vec::new();

    let mut gui = GuiState::new();

    for _ in 0..10 {
        if let Some((spawn_x, spawn_y)) = find_spawn_point(&world) {
            creatures.push(Creature::new(spawn_x, spawn_y, 2.0, RED));
        }
    }

    fn paint_dig_target(world: &mut World, x: i32, y: i32) {
        let radius = 2; // Adjust if you want bigger painting area

        for dx in -radius..=radius {
            for dy in -radius..=radius {
                let px = x + dx;
                let py = y + dy;

                let (chunk_x, chunk_y) = (px.div_euclid(32 * 8), py.div_euclid(32 * 8));
                let local_x = px.rem_euclid(32 * 8);
                let local_y = py.rem_euclid(32 * 8);
                let tile_x = local_x.div_euclid(8);
                let tile_y = local_y.div_euclid(8);
                let sub_x = local_x.rem_euclid(8);
                let sub_y = local_y.rem_euclid(8);

                if let Some(zlevel) = world.z_levels.get_mut(0) {
                    if let Some(chunk) = zlevel.chunks.get_mut(&(chunk_x, chunk_y)) {
                        if let Some(tile) = chunk
                            .tiles
                            .get_mut(tile_x as usize)
                            .and_then(|r| r.get_mut(tile_y as usize))
                        {
                            tile.subgrid[sub_x as usize][sub_y as usize].dig_target = true;
                            chunk.dirty = true;
                        }
                    }
                }
            }
        }
    }

    fn count_dig_jobs(world: &World) -> usize {
        let mut count = 0;

        for z in &world.z_levels {
            for chunk in z.chunks.values() {
                for row in &chunk.tiles {
                    for tile in row {
                        for subrow in &tile.subgrid {
                            for sub in subrow {
                                if sub.dig_target {
                                    count += 1;
                                }
                            }
                        }
                    }
                }
            }
        }

        count
    }

    loop {
        // ======== CAMERA INPUT ========

        let move_speed = 200.0 * get_frame_time(); // Frame-rate independent
        let zoom_speed = 0.1;

        if is_key_down(KeyCode::W) {
            renderer.move_camera_delta(0.0, -move_speed);
        }
        if is_key_down(KeyCode::S) {
            renderer.move_camera_delta(0.0, move_speed);
        }
        if is_key_down(KeyCode::A) {
            renderer.move_camera_delta(-move_speed, 0.0);
        }
        if is_key_down(KeyCode::D) {
            renderer.move_camera_delta(move_speed, 0.0);
        }

        let wheel = mouse_wheel().1;
        if wheel != 0.0 {
            let old_zoom = renderer.zoom;
            let new_zoom = (old_zoom + wheel * zoom_speed).clamp(1.0, 10.0);

            // Only adjust if zoom actually changed
            if (new_zoom - old_zoom).abs() > f32::EPSILON {
                // Get mouse position on screen
                let mouse = mouse_position();
                let mouse_x = mouse.0;
                let mouse_y = mouse.1;

                // Convert to world coordinates before zoom
                let world_x = renderer.camera_x + mouse_x / old_zoom;
                let world_y = renderer.camera_y + mouse_y / old_zoom;

                // Set zoom
                renderer.zoom = new_zoom;

                // Convert mouse back to screen space with new zoom
                let new_screen_x = (world_x - renderer.camera_x) * new_zoom;
                let new_screen_y = (world_y - renderer.camera_y) * new_zoom;

                // Offset camera so mouse stays locked
                renderer.camera_x += (mouse_x - new_screen_x) / new_zoom;
                renderer.camera_y += (mouse_y - new_screen_y) / new_zoom;
            }
        }

        // Convert mouse position to world coordinates
        let mouse_pos = mouse_position();
        let mouse_world_x = renderer.camera_x + mouse_pos.0 / renderer.zoom;
        let mouse_world_y = renderer.camera_y + mouse_pos.1 / renderer.zoom;

        if is_mouse_button_down(MouseButton::Left) {
            paint_rock(&mut world, mouse_world_x as i32, mouse_world_y as i32);
        }

        if is_mouse_button_down(MouseButton::Right) {
            let mouse_pos = mouse_position();
            let world_x = renderer.camera_x + mouse_pos.0 / renderer.zoom;
            let world_y = renderer.camera_y + mouse_pos.1 / renderer.zoom;

            paint_dig_target(&mut world, world_x as i32, world_y as i32);
        }

        fn paint_rock(world: &mut World, x: i32, y: i32) {
            let radius = 2; // Paint a small 5x5 blob around the cursor

            for dx in -radius..=radius {
                for dy in -radius..=radius {
                    let px = x + dx;
                    let py = y + dy;
                    world.set_material_at(px, py, TerrainMaterial::Rock);
                }
            }
        }

        // ======== RENDERING ========

        clear_background(BLACK);
        renderer.render(&mut world);

        for creature in &mut creatures {
            if creature.target.is_none() {
                creature.find_nearest_dig_target(&world);
            }
            creature.move_toward_target(&mut world);
            creature.dig_if_close(&mut world, &mut particles);
            creature.draw(renderer.camera_x, renderer.camera_y, renderer.zoom);
        }

        gui.dig_jobs = count_dig_jobs(&world);
        gui.update();

        for p in &mut particles {
            p.x += p.dx;
            p.y += p.dy;
            p.dy += 0.05; // gravity
            p.life = p.life.saturating_sub(1);
        }
        particles.retain(|p| p.life > 0);

        for p in &particles {
            let sx = (p.x - renderer.camera_x) * renderer.zoom;
            let sy = (p.y - renderer.camera_y) * renderer.zoom;
            draw_circle(sx, sy, 0.2 * renderer.zoom, YELLOW);
        }

        next_frame().await;
    }
}
