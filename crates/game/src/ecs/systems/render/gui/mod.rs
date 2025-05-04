// Module declarations
mod main_menu;
mod city_info;
mod worker_info;
mod worldgen;
mod debug;
mod input;
mod core;
mod gui_system;

// Re-export public API
pub use main_menu::{draw_main_menu, draw_main_menu_sync};
pub use city_info::draw_city_info_window;
pub use worker_info::draw_worker_info_window;
pub use worldgen::draw_worldgen_window;
pub use debug::draw_debug_info;
pub use input::handle_key_inputs;
pub use core::draw_gui;
pub use gui_system::*;
