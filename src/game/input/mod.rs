#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RenderMode {
    WorldMap,
    LocalMap,
}

pub mod handler;
pub mod local_map;
pub mod world_map;

pub use handler::InputHandler; 