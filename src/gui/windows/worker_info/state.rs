pub struct WorkerInfoState {
    pub show: bool,
    pub selected_worker: Option<usize>, // Placeholder for worker ID or struct
}

impl WorkerInfoState {
    pub fn new() -> Self {
        Self {
            show: false,
            selected_worker: None,
        }
    }
    pub fn is_visible(&self) -> bool { self.show }
    pub fn show(&mut self) { self.show = true; }
    pub fn hide(&mut self) { self.show = false; }
    pub fn toggle(&mut self) { self.show = !self.show; }
} 