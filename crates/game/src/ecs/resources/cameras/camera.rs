use bevy_ecs::prelude::*;
use crate::ecs::components::camera::Camera;

/// Resource that keeps track of which camera is currently active
#[derive(Resource, Default)]
pub struct ActiveCamera {
    /// Entity ID of the currently active camera
    pub entity: Option<Entity>,
}

/// Global camera settings that apply to all cameras
#[derive(Resource, Default)]
pub struct CameraSettings {
    /// Default zoom level for new cameras
    pub default_zoom: f32,
    /// Camera movement speed modifier
    pub movement_speed: f32,
    /// Camera zoom speed modifier
    pub zoom_speed: f32,
} 