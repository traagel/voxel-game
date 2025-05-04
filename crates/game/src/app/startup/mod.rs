// startup placeholder

pub mod main;

pub use main::*;

use bevy_ecs::prelude::*;

// Synchronous parts of initialization
pub fn init(_world: &mut World) {
    // This is the default startup system registered with the schedule
    // It's called once when the app starts
}

// Main async initialization function that's called from App::init()
pub async fn init_async(world: &mut World) {
    // Call the async init function
    main::init(world).await;
}
