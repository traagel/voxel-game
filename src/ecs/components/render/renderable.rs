use bevy_ecs::prelude::*;

/// Marker component indicating an entity can be rendered
#[derive(Component, Clone, Debug, Default)]
pub struct Renderable {
    // Add fields as needed for rendering (sprite, color, etc.)
    pub render_type: RenderType,
}

#[derive(Clone, Debug, Default)]
pub enum RenderType {
    #[default]
    Default,
    Creature,
    Particle,
}

impl Renderable {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn creature() -> Self {
        Self {
            render_type: RenderType::Creature,
        }
    }
    
    pub fn particle() -> Self {
        Self {
            render_type: RenderType::Particle,
        }
    }
} 