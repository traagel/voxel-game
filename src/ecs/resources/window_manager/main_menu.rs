use crate::gui::windows::main_menu::MainMenuState;
use bevy_ecs::prelude::*;

#[derive(Resource)]
pub struct MainMenuStateRes(pub MainMenuState);

impl Default for MainMenuStateRes {
    fn default() -> Self {
        Self(MainMenuState::new())
    }
} 