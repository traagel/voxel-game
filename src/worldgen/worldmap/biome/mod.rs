pub mod thresholds;
pub mod models;
pub mod classifiers;
pub mod map;

pub use classifiers::{temperature, vegetation, precipitation, elevation};
pub use map::classify_world; 