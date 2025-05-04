use bevy_ecs::prelude::*;
use crate::world::worldmap::city::City;
use crate::world::worldmap::civilization::Civilization;

#[derive(Resource, Default)]
pub struct CityInfoStateRes {
    pub show: bool,
    pub selected_city: Option<City>,
    pub selected_civ: Option<Civilization>,
}

impl CityInfoStateRes {
    pub fn new() -> Self {
        Self {
            show: false,
            selected_city: None,
            selected_civ: None,
        }
    }
    
    pub fn is_visible(&self) -> bool { self.show }
    pub fn show(&mut self) { self.show = true; }
    pub fn hide(&mut self) { self.show = false; }
    pub fn toggle(&mut self) { self.show = !self.show; }
} 