use bevy_ecs::prelude::*;
use crate::particle::Particle;

#[derive(Resource, Default)]
pub struct ParticlesRes(pub Vec<Particle>); 