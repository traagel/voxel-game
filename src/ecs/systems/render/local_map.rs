use bevy_ecs::prelude::*;
use crate::ecs::{
    components::{
        camera::{Camera, LocalMapCamera},
        render::Renderable,
        creature::Creature,
    },
    resources::{
        renderers::LocalMapRendererRes,
        world::WorldRes,
        game_view::GameViewRes,
        game_view::GameView,
        particle::ParticlesRes,
    }
};
use crate::game::game_state::GameState;
use macroquad::prelude::*;

pub fn draw_local_map(
    query: Query<&Camera, With<LocalMapCamera>>,
    creatures: Query<(&Creature, Option<&Renderable>)>,
    local_map_renderer: Res<LocalMapRendererRes>,
    world: Res<WorldRes>,
    game_view: Res<GameViewRes>,
    particles: Res<ParticlesRes>,
) {
    // Skip rendering if not in local map view
    if !matches!(game_view.active_view, GameView::LocalMap) {
        return;
    }

    // Get the local map camera
    if let Ok(camera) = query.single() {
        // Create game state from camera and world
        let state = GameState {
            camera_x: camera.x,
            camera_y: camera.y,
            zoom: camera.zoom,
            z_levels: &world.0.z_levels,
        };

        // Draw the local map using the existing renderer
        local_map_renderer.0.draw(&state);

        // Draw creatures
        for (creature, _) in &creatures {
            // We don't have direct access to the creature's draw method from the ECS component
            // Draw the creature as a simple circle for now
            let sx = (creature.dig_target.map_or(0.0, |(x, _)| x as f32) - camera.x) * camera.zoom;
            let sy = (creature.dig_target.map_or(0.0, |(_, y)| y as f32) - camera.y) * camera.zoom;
            draw_circle(sx, sy, 2.0 * camera.zoom, RED);
        }

        // Draw particles using the same approach as in your existing code
        for p in &particles.0 {
            let sx = (p.x - camera.x) * camera.zoom;
            let sy = (p.y - camera.y) * camera.zoom;
            draw_circle(sx, sy, 0.2 * camera.zoom, YELLOW);
        }
    }
} 