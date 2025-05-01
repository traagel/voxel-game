use crate::world::zlevel::ZLevel;

/// An immutable snapshot of all data needed for rendering.
pub struct GameState<'a> {
    pub camera_x: f32,
    pub camera_y: f32,
    pub zoom: f32,
    pub z_levels: &'a [ZLevel],
    // In the future, you can add creatures, particles, gui, etc.
} 