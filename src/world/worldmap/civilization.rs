//! Civilization types, cultures, and instances for world generation

use macroquad::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Civilization {
    Human,
    Elf,
    Dwarf,
    GnomeHalfling,
    OrcGoblin,
    Merfolk,
    Lizardfolk,
    FairyFae,
    Kobold,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alignment {
    LawfulGood,
    NeutralGood,
    ChaoticGood,
    LawfulNeutral,
    TrueNeutral,
    ChaoticNeutral,
    LawfulEvil,
    NeutralEvil,
    ChaoticEvil,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocietalTrait {
    Isolationist,
    Expansionist,
    Nomadic,
    Mercantile,
    Militaristic,
    Scholarly,
    Spiritual,
    // Add more as needed
}

#[derive(Debug, Clone)]
pub struct Culture {
    pub alignment: Alignment,
    pub tradition: String, // e.g. "Ancestor Worship", "Arcane Scholarship"
    pub religion: String,  // e.g. "Sun God", "Nature Spirits"
    pub trait_: SocietalTrait,
}

#[derive(Debug, Clone)]
pub struct CivilizationInstance {
    pub civ_type: Civilization,
    pub culture: Culture,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Relation {
    Peace,
    Neutral,
    War,
}

#[derive(Debug, Clone)]
pub struct CivilizationRelations {
    pub relations: std::collections::HashMap<(Civilization, Civilization), Relation>,
}

#[derive(Debug, Clone)]
pub struct TradeRoute {
    pub from: (usize, usize), // city coordinates
    pub to: (usize, usize),
    pub civ_a: Civilization,
    pub civ_b: Civilization,
    pub path: Vec<(usize, usize)>, // The full route as a list of points
    // Optionally: trade volume, type, etc.
}

impl Civilization {
    pub fn color(&self) -> Color {
        match self {
            Civilization::Human => RED,
            Civilization::Elf => GREEN,
            Civilization::Dwarf => GRAY,
            Civilization::GnomeHalfling => YELLOW,
            Civilization::OrcGoblin => DARKPURPLE,
            Civilization::Merfolk => SKYBLUE,
            Civilization::Lizardfolk => LIME,
            Civilization::FairyFae => PINK,
            Civilization::Kobold => ORANGE,
        }
    }
} 