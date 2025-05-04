use bevy_ecs::prelude::*;
use gui::GuiContext;

/// Resource that holds the GUI context from our GUI crate
#[derive(Resource)]
pub struct GuiContextRes(pub GuiContext);

impl GuiContextRes {
    pub fn new(context: GuiContext) -> Self {
        Self(context)
    }
    
    /// Get a mutable reference to the inner GuiContext
    pub fn context_mut(&mut self) -> &mut GuiContext {
        &mut self.0
    }
    
    /// Get a reference to the inner GuiContext
    pub fn context(&self) -> &GuiContext {
        &self.0
    }
} 