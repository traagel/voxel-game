// Update systems will be implemented here
// Examples: creature movement, world updates, etc. 

pub mod creatures;
pub mod particles;
pub mod window_interactions;
pub mod gui_system;

pub use creatures::*;
pub use particles::*;
pub use window_interactions::*;
pub use gui_system::*; 