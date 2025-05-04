use bevy_ecs::prelude::*;
use crate::ecs::resources::window_manager::WorldGenSettings;

#[derive(Resource)]
pub struct WorldGenSettingsRes(pub WorldGenSettings);

impl Default for WorldGenSettingsRes {
    fn default() -> Self {
        Self(WorldGenSettings::default())
    }
} 