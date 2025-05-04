use crate::world::worldmap::world_map::WorldMap;
use bevy_ecs::prelude::*;

#[derive(Resource)]
pub struct WorldMapRes(pub WorldMap); 