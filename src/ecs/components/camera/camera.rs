use bevy_ecs::prelude::*;

/// Camera component that works with your renderer
#[derive(Component, Debug, Clone)]
pub struct Camera {
    pub x: f32,
    pub y: f32,
    pub zoom: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, zoom: 4.0 }
    }
}

// Allow your renderer's Camera to be converted to an ECS Camera
impl From<&crate::renderer::camera::Camera> for Camera {
    fn from(cam: &crate::renderer::camera::Camera) -> Self {
        Self {
            x: cam.x,
            y: cam.y,
            zoom: cam.zoom,
        }
    }
}

// Allow ECS Camera to be used where a renderer Camera is expected
impl From<&Camera> for crate::renderer::camera::Camera {
    fn from(cam: &Camera) -> Self {
        Self {
            x: cam.x,
            y: cam.y,
            zoom: cam.zoom,
        }
    }
} 