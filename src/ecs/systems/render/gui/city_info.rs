use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui};
use crate::ecs::resources::window_manager::CityInfoStateRes;
use crate::world::worldmap::city::City;
use crate::ecs::resources::portraits::CivPortraits;
use crate::world::worldmap::world_map::WorldMap;

/// Draw the city information window
pub fn draw_city_info_window(
    state: &mut CityInfoStateRes, 
    city: &City, 
    portraits: &CivPortraits, 
    world_map: &WorldMap
) {
    if !state.show { return; }
    
    let win_pos = vec2(900.0, 80.0);
    let win_size = vec2(420.0, 340.0);
    
    // Calculate portrait position
    let portrait_size = 96.0;
    let px = win_pos.x + win_size.x - portrait_size - 16.0;
    let py = win_pos.y + 16.0;

    // Draw the portrait BEFORE the window, so the window doesn't cover it
    if let Some(src_rect) = portraits.get_portrait_rect(city.civ) {
        draw_texture_ex(
            portraits.get_texture(),
            px,
            py,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(portrait_size, portrait_size)),
                source: Some(src_rect),
                ..Default::default()
            },
        );
    }

    // Now draw the UI window (it will be drawn over the portrait, so leave space)
    root_ui().window(hash!("city_info_window"), win_pos, win_size, |ui| {
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
        // --- Civilization extended info ---
        if let Some(civ_instance) = &world_map.civilization_map[city.x][city.y] {
            ui.separator();
            ui.label(None, &format!("Alignment: {:?}", civ_instance.culture.alignment));
            ui.label(None, &format!("Tradition: {}", civ_instance.culture.tradition));
            ui.label(None, &format!("Religion: {}", civ_instance.culture.religion));
            ui.label(None, &format!("Trait: {:?}", civ_instance.culture.trait_));
        }
        // Show relations to other civs
        ui.separator();
        ui.label(None, "Relations:");
        use crate::world::worldmap::civilization::Civilization as CivEnum;
        for other in [
            CivEnum::Human,
            CivEnum::Elf,
            CivEnum::Dwarf,
            CivEnum::GnomeHalfling,
            CivEnum::OrcGoblin,
            CivEnum::Merfolk,
            CivEnum::Lizardfolk,
            CivEnum::FairyFae,
            CivEnum::Kobold,
        ] {
            if other != civ {
                if let Some(rel) = world_map.civ_relations.relations.get(&(civ, other)) {
                    ui.label(None, &format!("  {:?} ↔ {:?}: {:?}", civ, other, rel));
                }
            }
        }
        ui.separator();
        // Close button
        if ui.button(None, "Close") {
            state.show = false;
        }
    });
} 