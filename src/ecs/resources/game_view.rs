use bevy_ecs::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameView {
    MainMenu,
    WorldGen,
    WorldMap,
    LocalMap,
    CityInfo,
    RegionMap,
}

#[derive(Resource)]
pub struct GameViewRes {
    pub active_view: GameView,
}

impl Default for GameViewRes {
    fn default() -> Self {
        Self {
            active_view: GameView::WorldMap, // Default view
        }
    }
} 