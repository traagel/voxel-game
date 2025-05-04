use crate::gui::windows::window_state::WindowState;

#[derive(Default)]
pub struct MainMenuState {
    pub show_main: bool,
    pub show_settings: bool,
}

impl MainMenuState {
    pub fn new() -> Self {
        Self { show_main: false, show_settings: false }
    }
}

impl WindowState for MainMenuState {
    fn is_visible(&self) -> bool { self.show_main }
    fn show(&mut self) { self.show_main = true; }
    fn hide(&mut self) { self.show_main = false; }
    fn toggle(&mut self) { self.show_main = !self.show_main; }
} 