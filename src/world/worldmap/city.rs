//! City struct for world generation
use super::civilization::Civilization;

#[derive(Debug, Clone)]
pub struct City {
    pub name: String,
    pub civ: Civilization,
    pub x: usize,
    pub y: usize,
    pub population: u32,
    // Add more fields as needed (e.g., is_capital, trade_hub, etc.)
} 