use bevy_ecs::prelude::*;
use crate::ecs::resources::portraits::*;

/// System for loading assets during startup
pub async fn load_assets(world: &mut World) {
    println!("Starting asset loading...");
    
    // Load portrait assets
    let portraits = CivPortraits::load().await;
    world.insert_resource(CivPortraitsRes(portraits));
    
    println!("Asset loading complete!");
} 