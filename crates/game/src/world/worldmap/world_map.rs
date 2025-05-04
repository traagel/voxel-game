use crate::world::worldmap::biome::BiomeId;
use crate::world::worldmap::civilization::{CivilizationInstance, CivilizationRelations, TradeRoute};
use crate::world::worldmap::city::City;

#[derive(Debug, Clone)]
pub struct WorldMap {
    pub width: usize,
    pub height: usize,
    pub biomes: Vec<Vec<BiomeId>>,
    pub elevation: Vec<Vec<f64>>,
    pub moisture: Vec<Vec<f64>>,
    pub rivers: Vec<Vec<bool>>,
    pub temperature: Vec<Vec<f64>>,
    pub precipitation: Vec<Vec<f64>>,
    pub soil_fertility: Vec<Vec<f64>>,
    pub vegetation: Vec<Vec<f64>>,
    pub wind_direction: Vec<Vec<(f64, f64)>>,
    pub resources: Vec<Vec<Option<ResourceType>>>,
    // Category maps for composable biome logic
    pub temperature_map: Vec<Vec<crate::world::worldmap::biome::TemperatureType>>,
    pub vegetation_map: Vec<Vec<crate::world::worldmap::biome::VegetationType>>,
    pub precipitation_map: Vec<Vec<crate::world::worldmap::biome::PrecipitationType>>,
    pub elevation_map: Vec<Vec<crate::world::worldmap::biome::ElevationType>>,
    /// Map of civilizations and their cultures (None = uninhabited)
    pub civilization_map: Vec<Vec<Option<CivilizationInstance>>>,
    /// List of cities in the world
    pub cities: Vec<City>,
    /// Civilization relations (matrix)
    pub civ_relations: CivilizationRelations,
    /// Trade routes between cities/civilizations
    pub trade_routes: Vec<TradeRoute>,
    /// Sea level threshold for this world
    pub sea_level: f64,
    // You can add more fields later: elevation, rainfall, etc.
}

#[derive(Debug, Clone, Copy)]
pub enum ResourceType {
    Iron,
    Gold,
    Coal,
    Gems,
    Oil,
    // Add more as needed
} 