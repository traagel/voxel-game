use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui};
use crate::world::worldmap::city::City;
use crate::world::worldmap::civilization::{Civilization, BiomeId};

pub fn city_info_window(city: &City, show: &mut bool) {
    root_ui().window(hash!("city_info_window"), vec2(900.0, 80.0), vec2(420.0, 340.0), |ui| {
        ui.label(None, &format!("City: {}", city.name));
        ui.label(None, &format!("Population: {}", city.population));
        ui.label(None, &format!("Coordinates: ({}, {})", city.x, city.y));
        ui.separator();
        // Civilization info
        let civ = city.civ;
        let color = civ.color();
        ui.label(None, &format!("Civilization: {:?}", civ));
        ui.label(None, &format!("Civ Color: rgba({:.2}, {:.2}, {:.2}, {:.2})", color.r, color.g, color.b, color.a));
        // Preferred biomes
        let biomes = civ.preferred_biomes();
        let biome_names: Vec<String> = biomes.iter().map(|b| format!("{:?}", b)).collect();
        ui.label(None, &format!("Preferred Biomes: {}", biome_names.join(", ")));
        ui.separator();
        // Close button
        if ui.button(None, "Close") {
            *show = false;
        }
    });
} 