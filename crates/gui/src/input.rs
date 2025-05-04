use macroquad::prelude::{mouse_position, is_mouse_button_pressed, MouseButton, Vec2, KeyCode, is_key_pressed};

pub struct GuiInput {
    pub mouse_pos: Vec2,
    pub mouse_pressed: bool,
    pub mouse_released: bool,
    pub mouse_down: bool,
    pub key_pressed: Option<KeyCode>,
    pub scroll_delta: f32,
    pub text_input: Option<String>,
}

impl GuiInput {
    pub fn from_macroquad() -> Self {
        let (x, y) = mouse_position();
        Self {
            mouse_pos: Vec2::new(x, y),
            mouse_pressed: is_mouse_button_pressed(MouseButton::Left),
            mouse_released: false, // Would need additional state tracking to determine this
            mouse_down: macroquad::prelude::is_mouse_button_down(MouseButton::Left),
            key_pressed: None, // Would need to check for specific keys that are relevant
            scroll_delta: 0.0, // Would need to implement mouse wheel tracking
            text_input: None,  // Would need to implement text input capture
        }
    }
    
    pub fn with_key_pressed(mut self, key: KeyCode) -> Self {
        if is_key_pressed(key) {
            self.key_pressed = Some(key);
        }
        self
    }
    
    pub fn is_key_pressed(&self, key: KeyCode) -> bool {
        match self.key_pressed {
            Some(pressed_key) => pressed_key == key,
            None => false,
        }
    }
    
    pub fn is_mouse_inside_rect(&self, x: f32, y: f32, width: f32, height: f32) -> bool {
        self.mouse_pos.x >= x
            && self.mouse_pos.x <= x + width
            && self.mouse_pos.y >= y
            && self.mouse_pos.y <= y + height
    }
} 