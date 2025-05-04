use bevy_ecs::prelude::*;
use crate::ecs::systems::render::{draw_world_map, draw_local_map};
use crate::ecs::systems::input::handle_camera_input;
use crate::ecs::systems::update::{update_creatures, update_particles};
use crate::ecs::systems::register_gui_systems;

// Define the stages as system labels
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum Stage { 
    Startup, 
    Input, 
    Update, 
    Render 
}

pub fn build_schedule() -> Schedule {
    let mut schedule = Schedule::default();
    
    // Configure system sets to run in sequence
    schedule.configure_sets(
        (
            Stage::Startup,
            Stage::Input,
            Stage::Update,
            Stage::Render
        ).chain()
    );
    
    // Add startup system with run_once condition
    schedule.add_systems(
        crate::app::startup::init
            .in_set(Stage::Startup)
            .run_if(|| {
                static DONE: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false); 
                let is_first = !DONE.load(std::sync::atomic::Ordering::Relaxed);
                DONE.store(true, std::sync::atomic::Ordering::Relaxed);
                is_first
            })
    );
    
    // Add input systems
    schedule.add_systems(
        handle_camera_input.in_set(Stage::Input)
    );
    
    // Add update systems
    schedule.add_systems(
        (update_creatures, update_particles).in_set(Stage::Update)
    );
    
    // Add render systems
    schedule.add_systems(
        (draw_world_map, draw_local_map).in_set(Stage::Render)
    );
    
    // Register GUI systems (using our convenience function)
    // This will add GUI input, update, and render systems to their respective sets
    register_gui_systems(&mut schedule);
    
    schedule
} 