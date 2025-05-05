use crate::input::{event::InputEvent, state::InputState};
use crate::input::keymap::TRACKED_KEYS;
use std::collections::HashSet;
use macroquad::prelude::*;

pub struct KeyboardInput {
    previous: HashSet<KeyCode>,
}

impl KeyboardInput {
    pub fn new() -> Self {
        Self { previous: HashSet::new() }
    }

    pub fn poll(&mut self, state: &mut InputState, events: &mut Vec<InputEvent>) {
        let mut current = HashSet::new();

        for &key in TRACKED_KEYS {
            if is_key_down(key) {
                current.insert(key);
                state.keys_down.insert(key);
            }
        }

        for key in current.iter() {
            if !self.previous.contains(key) {
                events.push(InputEvent::KeyDown(*key));
            }
        }

        for key in self.previous.iter() {
            if !current.contains(key) {
                events.push(InputEvent::KeyUp(*key));
                state.keys_down.remove(key);
            }
        }

        self.previous = current;
    }
}
