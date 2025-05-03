use crate::world::worldmap::{CivilizationInstance, CivilizationRelations, Civilization, Relation};
use std::collections::HashMap;
use rand::Rng;
use rand::SeedableRng;

pub fn generate_relations(
    civ_seeds: &Vec<(usize, usize, CivilizationInstance, usize)>,
    seed: u32,
) -> CivilizationRelations {
    let mut relations = HashMap::new();
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed as u64 + 2000);
    // Collect all civ types present
    let mut civ_types = Vec::new();
    for (_, _, civ_inst, _) in civ_seeds {
        if !civ_types.contains(&civ_inst.civ_type) {
            civ_types.push(civ_inst.civ_type);
        }
    }
    // Assign random relations between all pairs (symmetric, no self-relations)
    for i in 0..civ_types.len() {
        for j in (i+1)..civ_types.len() {
            let rel = match rng.gen_range(0..10) {
                0 => Relation::War,
                1..=3 => Relation::Peace,
                _ => Relation::Neutral,
            };
            relations.insert((civ_types[i], civ_types[j]), rel);
            relations.insert((civ_types[j], civ_types[i]), rel);
        }
    }
    CivilizationRelations { relations }
}
