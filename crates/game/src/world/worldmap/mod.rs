//! Global world state (biome map, world metadata, etc)
pub mod biome;
pub mod world_map;
pub mod civilization;
pub mod city;

pub use biome::BiomeId;
pub use world_map::WorldMap;
pub use civilization::{Civilization, CivilizationInstance, Alignment, SocietalTrait, Culture, Relation, CivilizationRelations, TradeRoute};
pub use city::City;
