// ── ECS resources that wrap the two renderer structs ──
use bevy_ecs::prelude::*;
use crate::renderer::{local_map_renderer::LocalMapRenderer,
                      world_map_renderer::WorldMapRenderer};

#[derive(Resource)]
pub struct LocalMapRendererRes(pub LocalMapRenderer);

#[derive(Resource)]
pub struct WorldMapRendererRes(pub WorldMapRenderer);

impl Default for LocalMapRendererRes {
    fn default() -> Self {
        Self(LocalMapRenderer::default())
    }
}

// WorldMapRenderer doesn't have a simple default implementation since it requires async loading
// The actual initialization will happen in startup systems 