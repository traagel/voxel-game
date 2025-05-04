use macroquad::prelude::{is_mouse_button_down, is_mouse_button_pressed, is_mouse_button_released, mouse_position, mouse_wheel, MouseButton as MQMouseButton};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Wheel,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MouseEvent {
    pub button: MouseButton,
    pub pressed: bool,
    pub x: f32,
    pub y: f32,
    pub wheel_delta: f32, // Only used for wheel events
}

pub fn poll_mouse_events() -> Vec<MouseEvent> {
    let mut events = Vec::new();
    let (x, y) = mouse_position();
    // Continuous (down)
    for (btn, mq_btn) in [
        (MouseButton::Left, MQMouseButton::Left),
        (MouseButton::Right, MQMouseButton::Right),
        (MouseButton::Middle, MQMouseButton::Middle),
    ] {
        if is_mouse_button_down(mq_btn) {
            events.push(MouseEvent { button: btn, pressed: true, x, y, wheel_delta: 0.0 });
        }
    }
    // Discrete (pressed)
    for (btn, mq_btn) in [
        (MouseButton::Left, MQMouseButton::Left),
        (MouseButton::Right, MQMouseButton::Right),
        (MouseButton::Middle, MQMouseButton::Middle),
    ] {
        if is_mouse_button_pressed(mq_btn) {
            events.push(MouseEvent { button: btn, pressed: true, x, y, wheel_delta: 0.0 });
        }
    }
    // Discrete (released)
    for (btn, mq_btn) in [
        (MouseButton::Middle, MQMouseButton::Middle),
    ] {
        if is_mouse_button_released(mq_btn) {
            events.push(MouseEvent { button: btn, pressed: false, x, y, wheel_delta: 0.0 });
        }
    }
    // Mouse wheel
    let wheel = mouse_wheel().1;
    if wheel != 0.0 {
        events.push(MouseEvent { button: MouseButton::Wheel, pressed: true, x, y, wheel_delta: wheel });
    }
    events
} 