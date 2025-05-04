use crate::world::localmap::world::World;
use bevy_ecs::prelude::*;

#[derive(Resource, Default)]
pub struct WorldRes(pub World); 