use macroquad::prelude::*;
use serde::Deserialize;
use crate::world::worldmap::civilization::Civilization;
use std::collections::HashMap;

#[derive(Deserialize, Debug, Clone)]
pub struct CivSpriteTile {
    pub civilization: String,
    pub position: [u32; 2],
    #[serde(default)]
    pub variant: Option<String>,
    #[serde(default)]
    pub r#type: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CivSpriteMap {
    pub tileSize: [u32; 2],
    pub tiles: Vec<CivSpriteTile>,
    pub image: String,
}

pub struct CivPortraits {
    pub sprite_map: CivSpriteMap,
    pub texture: Texture2D,
    pub civ_to_tile: HashMap<Civilization, CivSpriteTile>,
}

impl CivPortraits {
    pub async fn load() -> Self {
        let json_str = load_string("assets/civ_sprite_map.json").await.unwrap();
        println!("[CivPortraits] Loaded JSON string, length: {}", json_str.len());
        let sprite_map: CivSpriteMap = serde_json::from_str(&json_str).unwrap();
        println!("[CivPortraits] Parsed CivSpriteMap: image = {}", sprite_map.image);
        let texture = load_texture(&format!("assets/{}", sprite_map.image)).await.unwrap();
        texture.set_filter(FilterMode::Nearest);
        // Map Civilization enum to CivSpriteTile
        let mut civ_to_tile = HashMap::new();
        for tile in &sprite_map.tiles {
            if let Ok(civ) = match tile.civilization.as_str() {
                "Human" => Ok(Civilization::Human),
                "Elf" => Ok(Civilization::Elf),
                "Dwarf" => Ok(Civilization::Dwarf),
                "GnomeHalfling" => Ok(Civilization::GnomeHalfling),
                "OrcGoblin" => Ok(Civilization::OrcGoblin),
                "Merfolk" => Ok(Civilization::Merfolk),
                "Lizardfolk" => Ok(Civilization::Lizardfolk),
                "FairyFae" => Ok(Civilization::FairyFae),
                "Kobold" => Ok(Civilization::Kobold),
                _ => Err(()),
            } {
                println!("[CivPortraits] Mapping civilization {:?} to tile at position {:?}", civ, tile.position);
                civ_to_tile.insert(civ, tile.clone());
            } else {
                println!("[CivPortraits] WARNING: Unknown civilization string '{}' in JSON", tile.civilization);
            }
        }
        println!("[CivPortraits] civ_to_tile keys: {:?}", civ_to_tile.keys());
        Self { sprite_map, texture, civ_to_tile }
    }

    pub fn get_portrait_rect(&self, civ: Civilization) -> Option<Rect> {
        self.civ_to_tile.get(&civ).map(|tile| {
            let [tw, th] = self.sprite_map.tileSize;
            let [px, py] = tile.position;
            Rect::new(px as f32 * tw as f32, py as f32 * th as f32, tw as f32, th as f32)
        })
    }

    pub fn get_texture(&self) -> &Texture2D {
        &self.texture
    }
} 