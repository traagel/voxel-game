pub mod roads;
pub mod seed;
pub mod relations;
pub mod trade;

use crate::world::worldmap::biome::BiomeId;
use crate::world::worldmap::city::City;
use crate::world::worldmap::{CivilizationRelations, TradeRoute};

/// Generate all civilization-related data: civ map, cities, relations, and trade routes.
pub fn generate_all(
    builder: &crate::worldgen::worldmap::builder::WorldMapBuilder,
    elevation: &[Vec<f64>],
    sea_level: f64,
    biomes: &[Vec<BiomeId>],
    river_mask: &[Vec<bool>],
) -> (
    Vec<Vec<Option<crate::world::worldmap::CivilizationInstance>>>,
    Vec<City>,
    CivilizationRelations,
    Vec<TradeRoute>,
) {
    let (civ_map, cities, civ_seeds) = seed::generate_civilizations_and_cities(
        builder, elevation, sea_level, biomes, river_mask,
    );
    let relations = relations::generate_relations(&civ_seeds, builder.seed);
    let trade = trade::generate_trade_routes(
        &civ_seeds, &cities, &elevation.to_vec(), &river_mask.to_vec(), sea_level, &relations
    );
    (civ_map, cities, relations, trade)
}
