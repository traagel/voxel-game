use bevy_ecs::prelude::*;
use crate::ecs::{
    components::camera::{Camera, WorldMapCamera},
    resources::{
        renderers::WorldMapRendererRes,
        world_map::WorldMapRes,
        game_view::GameViewRes,
        game_view::GameView,
        worldgen::WorldGenSettingsRes,
    }
};
use crate::renderer::world_map_renderer::MapView;

pub fn draw_world_map(
    query: Query<&Camera, With<WorldMapCamera>>,
    world_map_res: Res<WorldMapRes>,
    renderer: Res<WorldMapRendererRes>,
    game_view: Res<GameViewRes>,
    worldgen_settings: Option<Res<WorldGenSettingsRes>>,
) {
    // Skip rendering if not in a world map view
    if !matches!(game_view.active_view, GameView::WorldMap | GameView::CityInfo) {
        return;
    }

    // Get the world map camera
    if let Ok(camera) = query.single() {
        // Convert the ECS camera to a renderer camera
        let renderer_camera: crate::renderer::camera::Camera = camera.into();
        
        // Use the existing world map renderer with the converted camera
        renderer.0.draw_world_map_with_view(
            &world_map_res.0,
            &renderer_camera,
            MapView::Biome, // You might want to make this configurable
            world_map_res.0.sea_level,
        );
    }
} 