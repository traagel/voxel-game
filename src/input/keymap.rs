use macroquad::prelude::KeyCode;

pub const TRACKED_KEYS: &[KeyCode] = &[
    // Letters
    KeyCode::A, KeyCode::B, KeyCode::C, KeyCode::D, KeyCode::E,
    KeyCode::F, KeyCode::G, KeyCode::H, KeyCode::I, KeyCode::J,
    KeyCode::K, KeyCode::L, KeyCode::M, KeyCode::N, KeyCode::O,
    KeyCode::P, KeyCode::Q, KeyCode::R, KeyCode::S, KeyCode::T,
    KeyCode::U, KeyCode::V, KeyCode::W, KeyCode::X, KeyCode::Y, KeyCode::Z,

    // Numbers
    KeyCode::Key0, KeyCode::Key1, KeyCode::Key2, KeyCode::Key3, KeyCode::Key4,
    KeyCode::Key5, KeyCode::Key6, KeyCode::Key7, KeyCode::Key8, KeyCode::Key9,

    // Navigation
    KeyCode::Up, KeyCode::Down, KeyCode::Left, KeyCode::Right,
    KeyCode::PageUp, KeyCode::PageDown, KeyCode::Home, KeyCode::End,

    // Function
    KeyCode::F1, KeyCode::F2, KeyCode::F3, KeyCode::F4,
    KeyCode::F5, KeyCode::F6, KeyCode::F7, KeyCode::F8,
    KeyCode::F9, KeyCode::F10, KeyCode::F11, KeyCode::F12,

    // Control
    KeyCode::Enter, KeyCode::Escape, KeyCode::Tab, KeyCode::Backspace,
    KeyCode::LeftShift, KeyCode::RightShift,
    KeyCode::LeftControl, KeyCode::RightControl,
    KeyCode::LeftAlt, KeyCode::RightAlt,

    // Symbols
    KeyCode::Space, KeyCode::Minus, KeyCode::Equal,
    KeyCode::LeftBracket, KeyCode::RightBracket,
    KeyCode::Backslash, KeyCode::Semicolon, KeyCode::Apostrophe,
    KeyCode::Comma, KeyCode::Period, KeyCode::Slash, KeyCode::GraveAccent,
];
