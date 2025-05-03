use crate::world::worldmap::biome::BiomeId;
use crate::world::worldmap::city::City;
use crate::world::worldmap::{Civilization, CivilizationInstance, Culture, Alignment, SocietalTrait};
use rand::Rng;
use rand::SeedableRng;
use rand::seq::SliceRandom;

/// Returns (civ_map, cities, civ_seeds)
pub fn generate_civilizations_and_cities(
    builder: &crate::worldgen::worldmap::builder::WorldMapBuilder,
    elevation: &[Vec<f64>],
    sea_level: f64,
    biomes: &[Vec<BiomeId>],
    _river_mask: &[Vec<bool>],
) -> (
    Vec<Vec<Option<CivilizationInstance>>>,
    Vec<City>,
    Vec<(usize, usize, CivilizationInstance, usize)>,
) {
    let width = builder.width;
    let height = builder.height;
    let seed = builder.seed;
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed as u64 + 1000);
    let civ_types = [
        Civilization::Human,
        Civilization::Elf,
        Civilization::Dwarf,
        Civilization::GnomeHalfling,
        Civilization::OrcGoblin,
        Civilization::Merfolk,
        Civilization::Lizardfolk,
        Civilization::FairyFae,
        Civilization::Kobold,
    ];
    let alignments = [
        Alignment::LawfulGood, Alignment::NeutralGood, Alignment::ChaoticGood,
        Alignment::LawfulNeutral, Alignment::TrueNeutral, Alignment::ChaoticNeutral,
        Alignment::LawfulEvil, Alignment::NeutralEvil, Alignment::ChaoticEvil,
    ];
    let traits = [
        SocietalTrait::Isolationist, SocietalTrait::Expansionist, SocietalTrait::Nomadic,
        SocietalTrait::Mercantile, SocietalTrait::Militaristic, SocietalTrait::Scholarly, SocietalTrait::Spiritual,
    ];
    let traditions = [
        "Ancestor Worship", "Arcane Scholarship", "Nature Reverence", "Warrior Code", "Trade Guilds", "Seafaring", "Stonecraft", "Dreamwalking", "Shadow Pact"
    ];
    let religions = [
        "Sun God", "Nature Spirits", "Ancestor Ghosts", "The Great Machine", "The Deep", "The Dragon", "The Moon Court", "The Old Ones", "The Flame"
    ];
    // Place multiple civilization seeds per type
    let mut civ_seeds = Vec::new();
    for &civ in &civ_types {
        let num_seeds = rng.gen_range(2..=4); // 2-4 per civ type
        let preferred_biomes = civ.preferred_biomes();
        for _ in 0..num_seeds {
            let mut found = false;
            // Try to find a preferred biome tile for the civ seed
            for _ in 0..100 {
                let x = rng.gen_range(0..width);
                let y = rng.gen_range(0..height);
                if elevation[x][y] > sea_level
                    && preferred_biomes.contains(&biomes[x][y])
                    && !civ_seeds.iter().any(|(cx,cy,_,_)| (*cx as isize - x as isize).abs() < 10 && (*cy as isize - y as isize).abs() < 10)
                {
                    let culture = Culture {
                        alignment: *alignments.choose(&mut rng).unwrap(),
                        tradition: traditions.choose(&mut rng).unwrap().to_string(),
                        religion: religions.choose(&mut rng).unwrap().to_string(),
                        trait_: *traits.choose(&mut rng).unwrap(),
                    };
                    let instance_id = civ_seeds.len();
                    civ_seeds.push((x, y, CivilizationInstance { civ_type: civ, culture }, instance_id));
                    found = true;
                    break;
                }
            }
            // Fallback: any land tile if no preferred biome found
            if !found {
                for _ in 0..100 {
                    let x = rng.gen_range(0..width);
                    let y = rng.gen_range(0..height);
                    if elevation[x][y] > sea_level
                        && !civ_seeds.iter().any(|(cx,cy,_,_)| (*cx as isize - x as isize).abs() < 10 && (*cy as isize - y as isize).abs() < 10)
                    {
                        let culture = Culture {
                            alignment: *alignments.choose(&mut rng).unwrap(),
                            tradition: traditions.choose(&mut rng).unwrap().to_string(),
                            religion: religions.choose(&mut rng).unwrap().to_string(),
                            trait_: *traits.choose(&mut rng).unwrap(),
                        };
                        let instance_id = civ_seeds.len();
                        civ_seeds.push((x, y, CivilizationInstance { civ_type: civ, culture }, instance_id));
                        break;
                    }
                }
            }
        }
    }
    // Assign each tile to the nearest civ seed within a radius (clustered influence)
    let influence_radius = 20.0; // adjust as desired
    let mut civilization_map = vec![vec![None; height]; width];
    for x in 0..width {
        for y in 0..height {
            if elevation[x][y] > sea_level {
                let mut min_dist = f64::MAX;
                let mut nearest = None;
                for (cx, cy, civ, _instance_id) in &civ_seeds {
                    let dist = (((*cx as isize - x as isize).pow(2) + (*cy as isize - y as isize).pow(2)) as f64).sqrt();
                    if dist < min_dist {
                        min_dist = dist;
                        nearest = Some(civ.clone());
                    }
                }
                if min_dist <= influence_radius {
                    civilization_map[x][y] = nearest;
                } else {
                    civilization_map[x][y] = None; // wilderness
                }
            }
        }
    }
    // Place cities for each civilization instance
    let mut cities = Vec::new();
    let city_names = [
        "Aldoria", "Brighthaven", "Stonehelm", "Rivermouth", "Frostford", "Sunspire", "Shadowfen", "Goldport", "Ironhold", "Starfall", "Mistwood", "Deepmere", "Windrest", "Moonwatch", "Thundertop"
    ];
    for (seed_x, seed_y, civ_inst, instance_id) in &civ_seeds {
        let mut placed = 0;
        let max_cities = 3;
        for _ in 0..200 {
            let x = rng.gen_range(0..width);
            let y = rng.gen_range(0..height);
            if civilization_map[x][y].as_ref().map(|c| c.civ_type) == Some(civ_inst.civ_type)
                && elevation[x][y] > sea_level
                && (x != *seed_x || y != *seed_y)
                && !cities.iter().any(|c: &City| (c.x as isize - x as isize).abs() < 5 && (c.y as isize - y as isize).abs() < 5)
            {
                let name = city_names.choose(&mut rng).unwrap_or(&"Unnamed").to_string();
                let population = rng.gen_range(1000..100_000);
                cities.push(City {
                    name,
                    civ: civ_inst.civ_type,
                    x,
                    y,
                    population,
                });
                placed += 1;
                if placed >= max_cities { break; }
            }
        }
        // Always place a capital at the seed
        let name = format!("{} Capital {}", match civ_inst.civ_type {
            Civilization::Human => "Human",
            Civilization::Elf => "Elf",
            Civilization::Dwarf => "Dwarf",
            Civilization::GnomeHalfling => "Gnome/Halfling",
            Civilization::OrcGoblin => "Orc/Goblin",
            Civilization::Merfolk => "Merfolk",
            Civilization::Lizardfolk => "Lizardfolk",
            Civilization::FairyFae => "Fairy/Fae",
            Civilization::Kobold => "Kobold",
        }, instance_id);
        let population = rng.gen_range(50_000..500_000);
        cities.push(City {
            name,
            civ: civ_inst.civ_type,
            x: *seed_x,
            y: *seed_y,
            population,
        });
    }
    (civilization_map, cities, civ_seeds)
}
