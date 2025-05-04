use bevy_ecs::prelude::*;

#[derive(Resource, Default)]
pub struct WindowManagerRes {
    // Any common window state can go here if needed
    pub active_windows: Vec<String>,
}

impl WindowManagerRes {
    pub fn new() -> Self {
        Self {
            active_windows: Vec::new(),
        }
    }
} 