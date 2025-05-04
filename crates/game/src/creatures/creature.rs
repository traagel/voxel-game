use macroquad::prelude::*;

pub struct Creature {
    pub x: f32,
    pub y: f32,
    pub size: f32,
    pub color: Color,
    pub target: Option<(i32, i32)>,
    pub dig_cooldown: u32,
}

impl Creature {
    pub fn new(x: f32, y: f32, size: f32, color: Color) -> Self {
        Self {
            x,
            y,
            size,
            color,
            target: None,
            dig_cooldown: 0,
        }
    }
} 