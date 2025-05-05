use macroquad::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum InputEvent {
    KeyDown(KeyCode),
    KeyUp(KeyCode),
    MouseDown(MouseButton),
    MouseUp(MouseButton),
    MouseMove { x: f32, y: f32 },
    MouseScroll { delta: f32 },
    TextInput(char),
}
