use bevy_ecs::prelude::*;

#[derive(Resource, Default)]
pub struct MainMenuStateRes {
    pub show_main: bool,
    pub show_settings: bool,
}

impl MainMenuStateRes {
    pub fn new() -> Self {
        Self { show_main: false, show_settings: false }
    }
    
    pub fn is_visible(&self) -> bool { self.show_main }
    pub fn show(&mut self) { self.show_main = true; }
    pub fn hide(&mut self) { self.show_main = false; }
    pub fn toggle(&mut self) { self.show_main = !self.show_main; }
} 