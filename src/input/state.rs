use macroquad::prelude::*;
use std::collections::HashSet;

#[derive(Default)]
pub struct InputState {
    pub keys_down: HashSet<KeyCode>,
    pub mouse_buttons: HashSet<MouseButton>,
    pub mouse_position: (f32, f32),
    pub mouse_scroll: f32,
}
