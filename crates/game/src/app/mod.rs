// app placeholder

pub mod schedule;
pub mod startup;

use bevy_ecs::prelude::*;
use macroquad::prelude::*;

pub struct App {
    pub world: World,
    pub schedule: Schedule,
}

impl App {
    pub fn new() -> Self {
        let world = World::default();
        let schedule = schedule::build_schedule();
        Self { world, schedule }
    }

    pub async fn init(&mut self) {
        // Initialize resources that require async operations
        startup::init_async(&mut self.world).await;
    }

    pub async fn run(&mut self) {
        let mut frame_count = 0;
        
        loop {
            // Clear the screen at the beginning of each frame
            macroquad::prelude::clear_background(macroquad::prelude::BLACK);
            
            // Run one iteration of the schedule
            self.schedule.run(&mut self.world);
            
            // Wait for the next frame
            next_frame().await;
            
            frame_count += 1;
            
            // Prevent flooding console with too many debug messages
            if frame_count > 10 {
                // Continue running without debug prints
                loop {
                    // Clear screen at the beginning of each frame
                    macroquad::prelude::clear_background(macroquad::prelude::BLACK);
                    
                    self.schedule.run(&mut self.world);
                    next_frame().await;
                }
            }
        }
    }
}
