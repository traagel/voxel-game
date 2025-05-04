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

/// Compatibility enum for older code that uses RenderMode
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RenderMode {
    WorldMap,
    LocalMap,
}

/// Conversion between GameView and RenderMode for compatibility
impl From<GameView> for RenderMode {
    fn from(view: GameView) -> Self {
        match view {
            GameView::WorldMap | GameView::CityInfo | GameView::RegionMap | GameView::WorldGen => RenderMode::WorldMap,
            GameView::LocalMap => RenderMode::LocalMap,
            GameView::MainMenu => RenderMode::WorldMap, // Default to world map for main menu
        }
    }
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