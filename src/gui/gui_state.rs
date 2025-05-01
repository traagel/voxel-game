pub struct GuiState {
    pub worldgen_width: usize,
    pub worldgen_height: usize,
}

impl GuiState {
    pub fn new() -> Self {
        Self {
            worldgen_width: 128,
            worldgen_height: 128,
        }
    }
} 