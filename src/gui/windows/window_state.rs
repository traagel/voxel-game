pub trait WindowState {
    fn is_visible(&self) -> bool;
    fn show(&mut self);
    fn hide(&mut self);
    fn toggle(&mut self);
} 