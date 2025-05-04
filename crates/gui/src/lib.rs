pub mod context;
pub mod widgets;
pub mod theme;
pub mod input;
pub mod draw;
pub mod example;

pub use context::{GuiContext, Window, WindowId};
pub use widgets::{Widget, Label, Button, Image};
pub use theme::Theme;
pub use input::GuiInput;

// Re-export needed macroquad types
pub use macroquad::prelude::{Rect, Vec2, Color, Texture2D, MouseButton, WHITE};
