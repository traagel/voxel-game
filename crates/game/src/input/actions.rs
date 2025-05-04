#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    PanCamera { dx: f32, dy: f32 },
    Zoom { delta: f32, x: f32, y: f32 },
    CenterCamera,
    OpenMenu,
    SwitchView,
    PaintTile { x: i32, y: i32 },
    DigTile { x: i32, y: i32 },
    StartDrag { x: f32, y: f32 },
    Drag { x: f32, y: f32, dx: f32, dy: f32 },
    EndDrag,
    CityClick { x: f32, y: f32 },
    // Add more as needed
} 