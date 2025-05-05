use crate::input::{event::InputEvent, state::InputState};
use macroquad::prelude::*;
use std::collections::HashSet;

pub struct MouseInput {
    previous_buttons: HashSet<MouseButton>,
    previous_pos: (f32, f32),
}

impl MouseInput {
    pub fn new() -> Self {
        Self {
            previous_buttons: HashSet::new(),
            previous_pos: mouse_position(),
        }
    }

    pub fn poll(&mut self, state: &mut InputState, events: &mut Vec<InputEvent>) {
        let current_pos = mouse_position();
        if current_pos != self.previous_pos {
            events.push(InputEvent::MouseMove {
                x: current_pos.0,
                y: current_pos.1,
            });
        }
        state.mouse_position = current_pos;
        self.previous_pos = current_pos;

        for button in [MouseButton::Left, MouseButton::Right, MouseButton::Middle] {
            if is_mouse_button_down(button) {
                state.mouse_buttons.insert(button);
                if !self.previous_buttons.contains(&button) {
                    events.push(InputEvent::MouseDown(button));
                }
            } else if self.previous_buttons.contains(&button) {
                events.push(InputEvent::MouseUp(button));
                state.mouse_buttons.remove(&button);
            }
        }

        self.previous_buttons = state.mouse_buttons.clone();

        let scroll = mouse_wheel().1;
        if scroll.abs() > 0.0 {
            state.mouse_scroll = scroll;
            events.push(InputEvent::MouseScroll { delta: scroll });
        } else {
            state.mouse_scroll = 0.0;
        }
    }
}
