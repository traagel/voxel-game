use crate::gui::windows::worldgen::WorldGenWindowState;
use bevy_ecs::prelude::*;

#[derive(Resource)]
pub struct WorldGenWindowStateRes(pub WorldGenWindowState);

impl Default for WorldGenWindowStateRes {
    fn default() -> Self {
        Self(WorldGenWindowState::new())
    }
} 