pub struct GuiState {
    pub worldgen_width: usize,
    pub worldgen_height: usize,
    pub selected_city: Option<crate::world::worldmap::city::City>,
    pub show_city_info: bool,
}

impl GuiState {
    pub fn new() -> Self {
        Self {
            worldgen_width: 128,
            worldgen_height: 128,
            selected_city: None,
            show_city_info: false,
        }
    }
} 