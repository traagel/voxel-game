use crate::input::actions::Action;
use crate::input::devices::keyboard::KeyboardEvent;
use crate::input::devices::mouse::{MouseEvent, MouseButton};
use macroquad::prelude::KeyCode;

static mut PREV_DRAG_POS: Option<(f32, f32)> = None;

pub fn map_keyboard_event(event: &KeyboardEvent) -> Option<Action> {
    match event.key {
        KeyCode::W if event.pressed => Some(Action::PanCamera { dx: 0.0, dy: -1.0 }),
        KeyCode::S if event.pressed => Some(Action::PanCamera { dx: 0.0, dy: 1.0 }),
        KeyCode::A if event.pressed => Some(Action::PanCamera { dx: -1.0, dy: 0.0 }),
        KeyCode::D if event.pressed => Some(Action::PanCamera { dx: 1.0, dy: 0.0 }),
        KeyCode::Tab if event.pressed => Some(Action::SwitchView),
        KeyCode::Escape if event.pressed => Some(Action::OpenMenu),
        KeyCode::C if event.pressed => Some(Action::CenterCamera),
        _ => None,
    }
}

pub fn map_mouse_event(event: &MouseEvent) -> Option<Action> {
    match event.button {
        MouseButton::Left if event.pressed => {
            // Determine the appropriate action based on the context
            // For now, let's default to CityClick since that's what we're fixing
            Some(Action::CityClick { x: event.x, y: event.y })
            
            // Later, this could be context-aware:
            // if game_view is WorldMap -> CityClick
            // if game_view is LocalMap -> PaintTile
            // etc.
        },
        MouseButton::Right if event.pressed => Some(Action::DigTile { x: event.x as i32, y: event.y as i32 }),
        MouseButton::Wheel if event.pressed && event.wheel_delta != 0.0 => Some(Action::Zoom { delta: event.wheel_delta, x: event.x, y: event.y }),
        MouseButton::Middle if event.pressed => {
            // Start drag
            unsafe { PREV_DRAG_POS = Some((event.x, event.y)); }
            Some(Action::StartDrag { x: event.x, y: event.y })
        },
        MouseButton::Middle if !event.pressed => {
            // End drag
            unsafe { PREV_DRAG_POS = None; }
            Some(Action::EndDrag)
        },
        // Drag (continuous)
        MouseButton::Middle => {
            unsafe {
                if let Some((prev_x, prev_y)) = PREV_DRAG_POS {
                    let dx = event.x - prev_x;
                    let dy = event.y - prev_y;
                    PREV_DRAG_POS = Some((event.x, event.y));
                    if dx != 0.0 || dy != 0.0 {
                        return Some(Action::Drag { x: event.x, y: event.y, dx, dy });
                    }
                }
            }
            None
        },
        _ => None,
    }
} 