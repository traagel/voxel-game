use crate::input::{
    event::InputEvent,
    state::InputState,
    device::{keyboard::KeyboardInput, mouse::MouseInput, text::TextInput},
};

pub struct InputManager {
    keyboard: KeyboardInput,
    mouse: MouseInput,
    text: TextInput,
    state: InputState,
    events: Vec<InputEvent>,
}

impl InputManager {
    pub fn new() -> Self {
        Self {
            keyboard: KeyboardInput::new(),
            mouse: MouseInput::new(),
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

    pub fn events(&self) -> &[InputEvent] {
        &self.events
    }

    pub fn state(&self) -> &InputState {
        &self.state
    }
}
