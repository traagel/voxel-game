//! Global world‑map generation: continents, climate, biomes, civilizations.

pub mod params;
pub mod utils;
pub mod terrain;
pub mod climate;
pub mod hydrology;
pub mod biome;
pub mod civ;
pub mod builder;          // thin orchestrator

// Only keep the builder-based alias for backward compatibility
pub use builder::WorldMapBuilder as WorldMapGenerator;
