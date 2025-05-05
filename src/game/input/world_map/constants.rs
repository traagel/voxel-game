/// The size of a tile in pixels
pub const TILE_PX: f32 = 8.0;

/// The minimum zoom level
pub const MIN_ZOOM: f32 = 1.0;

/// The maximum zoom level
pub const MAX_ZOOM: f32 = 10.0;

/// The speed of zooming
pub const ZOOM_SPEED: f32 = 0.2;

/// The base movement speed (multiplied by frame time)
pub const BASE_MOVE_SPEED: f32 = 200.0;

/// The margin (in world units) that determines how far the camera can go beyond the map edges
/// A value of 0.0 means the camera can't show anything beyond the map edges
pub const CAMERA_MARGIN: f32 = 10.0;

/// The minimum allowed distance (in world units) from camera center to map edge
/// This ensures the camera doesn't zoom out too far from the map
pub const MIN_CAMERA_DISTANCE: f32 = -CAMERA_MARGIN; 