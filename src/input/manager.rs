use crate::input::{
    device::{keyboard::KeyboardInput, mouse::MouseInput as DeviceMouseInput, text::TextInput},
    event::InputEvent,
    state::InputState,
};
use std::collections::HashSet;
use macroquad::prelude::*;

pub struct InputManager {
    keyboard: KeyboardInput,
    mouse: DeviceMouseInput,
    text: TextInput,
    pub state: InputState,
    pub events: Vec<InputEvent>,
}

impl InputManager {
    pub fn new() -> Self {
        Self {
            keyboard: KeyboardInput::new(),
            mouse: DeviceMouseInput::new(),
            text: TextInput,
            state: InputState::default(),
            events: Vec::with_capacity(64),
        }
    }

    pub fn begin_frame(&mut self) {
        self.events.clear();
    }

    pub fn collect(&mut self) {
        self.keyboard.poll(&mut self.state, &mut self.events);
        self.mouse.poll(&mut self.state, &mut self.events);
        self.text.poll(&mut self.events);
    }

    pub fn key(&self) -> KeyInput {
        KeyInput {
            events: &self.events,
            keys_down: &self.state.keys_down,
        }
    }

    pub fn mouse(&self) -> MouseInputView {
        MouseInputView {
            events: &self.events,
            buttons_down: &self.state.mouse_buttons,
            pos: self.state.mouse_position,
        }
    }
}

// -- Submodules

pub struct KeyInput<'a> {
    events: &'a [InputEvent],
    keys_down: &'a HashSet<KeyCode>,
}

impl<'a> KeyInput<'a> {
    pub fn pressed(&self, key: KeyCode) -> bool {
        self.events.iter().any(|e| matches!(e, InputEvent::KeyDown(k) if *k == key))
    }

    pub fn released(&self, key: KeyCode) -> bool {
        self.events.iter().any(|e| matches!(e, InputEvent::KeyUp(k) if *k == key))
    }

    pub fn held(&self, key: KeyCode) -> bool {
        self.keys_down.contains(&key)
    }
}

pub struct MouseInputView<'a> {
    events: &'a [InputEvent],
    buttons_down: &'a HashSet<MouseButton>,
    pos: (f32, f32),
}

impl<'a> MouseInputView<'a> {
    pub fn pressed(&self, button: MouseButton) -> bool {
        self.events.iter().any(|e| matches!(e, InputEvent::MouseDown(b) if *b == button))
    }

    pub fn released(&self, button: MouseButton) -> bool {
        self.events.iter().any(|e| matches!(e, InputEvent::MouseUp(b) if *b == button))
    }

    pub fn held(&self, button: MouseButton) -> bool {
        self.buttons_down.contains(&button)
    }

    pub fn pos(&self) -> (f32, f32) {
        self.pos
    }

    pub fn scroll_delta(&self) -> f32 {
        self.events.iter().find_map(|e| {
            if let InputEvent::MouseScroll { delta } = *e {
                Some(delta)
            } else {
                None
            }
        }).unwrap_or(0.0)
    }
}
