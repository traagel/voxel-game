use crate::input::event::InputEvent;
use macroquad::prelude::*;

pub struct TextInput;

impl TextInput {
    pub fn poll(&mut self, events: &mut Vec<InputEvent>) {
        while let Some(c) = get_char_pressed() {
            events.push(InputEvent::TextInput(c));
        }
    }
}
