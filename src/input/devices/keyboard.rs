use macroquad::prelude::{is_key_down, is_key_pressed, KeyCode};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyboardEvent {
    pub key: KeyCode,
    pub pressed: bool,
}

pub fn poll_keyboard_events() -> Vec<KeyboardEvent> {
    let mut events = Vec::new();
    // WASD (continuous)
    for &key in &[KeyCode::W, KeyCode::A, KeyCode::S, KeyCode::D] {
        if is_key_down(key) {
            events.push(KeyboardEvent { key, pressed: true });
        }
    }
    // Tab, Escape, C (discrete)
    for &key in &[KeyCode::Tab, KeyCode::Escape, KeyCode::C] {
        if is_key_pressed(key) {
            events.push(KeyboardEvent { key, pressed: true });
        }
    }
    events
} 