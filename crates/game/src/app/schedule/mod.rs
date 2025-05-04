// schedule placeholder

pub mod main;

pub use main::*;

use bevy_ecs::prelude::*;

pub fn build_schedule() -> Schedule {
    // Call the actual implementation in main.rs
    main::build_schedule()
}
