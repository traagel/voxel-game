use crate::gui::windows::window_state::WindowState;
use crate::world::worldmap::city::City;
use crate::world::worldmap::civilization::Civilization;

#[derive(Default)]
pub struct CityInfoState {
    pub show: bool,
    pub selected_city: Option<City>,
    pub selected_civ: Option<Civilization>,
}

impl CityInfoState {
    pub fn new() -> Self {
        Self {
            show: false,
            selected_city: None,
            selected_civ: None,
        }
    }
}

impl WindowState for CityInfoState {
    fn is_visible(&self) -> bool { self.show }
    fn show(&mut self) { self.show = true; }
    fn hide(&mut self) { self.show = false; }
    fn toggle(&mut self) { self.show = !self.show; }
} 