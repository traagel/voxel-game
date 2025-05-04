pub mod devices;
pub mod mapping;
pub mod actions;

use actions::Action;
use devices::keyboard::poll_keyboard_events;
use devices::mouse::poll_mouse_events;
use mapping::bindings::{map_keyboard_event, map_mouse_event};

pub fn poll_actions() -> Vec<Action> {
    let mut actions = Vec::new();
    for event in poll_keyboard_events() {
        if let Some(action) = map_keyboard_event(&event) {
            actions.push(action);
        }
    }
    for event in poll_mouse_events() {
        if let Some(action) = map_mouse_event(&event) {
            actions.push(action);
        }
    }
    actions
} 