use crate::gui::windows::window_manager::WindowManager;
use bevy_ecs::prelude::*;

#[derive(Resource)]
pub struct WindowManagerRes(pub WindowManager); 