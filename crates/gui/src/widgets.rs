use macroquad::prelude::{Rect, Texture2D};

#[derive(Clone)]
pub enum Widget {
    Label(Label),
    Button(Button),
    Image(Image),
    Checkbox(Checkbox),
    Slider(Slider),
}

#[derive(Clone, Copy, PartialEq)]
pub enum TextAlign {
    Left,
    Center,
    Right,
}

#[derive(Clone)]
pub struct Label {
    pub rect: Rect,
    pub text: String,
    pub font_size: u16,
    pub alignment: TextAlign,
}

impl Label {
    pub fn new(rect: Rect, text: impl Into<String>, font_size: u16) -> Self {
        Self {
            rect,
            text: text.into(),
            font_size,
            alignment: TextAlign::Left, // Default alignment
        }
    }
    
    pub fn centered(rect: Rect, text: impl Into<String>, font_size: u16) -> Self {
        Self {
            rect,
            text: text.into(),
            font_size,
            alignment: TextAlign::Center,
        }
    }
    
    pub fn with_alignment(mut self, alignment: TextAlign) -> Self {
        self.alignment = alignment;
        self
    }
    
    pub fn as_widget(self) -> Widget {
        Widget::Label(self)
    }
}

#[derive(Clone)]
pub struct Button {
    pub rect: Rect,
    pub text: String,
    pub clicked: bool,
    pub hovered: bool,
}

impl Button {
    pub fn new(rect: Rect, text: impl Into<String>) -> Self {
        Self {
            rect,
            text: text.into(),
            clicked: false,
            hovered: false,
        }
    }
    
    /// Create a centered button at position (x, y) with the given width and height
    pub fn centered(x: f32, y: f32, width: f32, height: f32, text: impl Into<String>) -> Self {
        Self {
            rect: Rect::new(x - width/2.0, y - height/2.0, width, height),
            text: text.into(),
            clicked: false,
            hovered: false,
        }
    }
    
    pub fn as_widget(self) -> Widget {
        Widget::Button(self)
    }
    
    pub fn was_clicked(&self) -> bool {
        self.clicked
    }
}

#[derive(Clone)]
pub struct Image {
    pub rect: Rect,
    pub tex: Texture2D,
}

impl Image {
    pub fn new(rect: Rect, tex: Texture2D) -> Self {
        Self { rect, tex }
    }
    
    pub fn as_widget(self) -> Widget {
        Widget::Image(self)
    }
}

#[derive(Clone)]
pub struct Checkbox {
    pub rect: Rect,
    pub label: String,
    pub checked: bool,
    pub hovered: bool,
    pub clicked: bool,
}

impl Checkbox {
    pub fn new(rect: Rect, label: impl Into<String>, checked: bool) -> Self {
        Self {
            rect,
            label: label.into(),
            checked,
            hovered: false,
            clicked: false,
        }
    }
    
    pub fn as_widget(self) -> Widget {
        Widget::Checkbox(self)
    }
    
    pub fn is_checked(&self) -> bool {
        self.checked
    }
}

#[derive(Clone)]
pub struct Slider {
    pub rect: Rect,
    pub value: f32,
    pub min: f32,
    pub max: f32,
    pub active: bool,
    pub hovered: bool,
    pub dragging: bool,
}

impl Slider {
    pub fn new(rect: Rect, min: f32, max: f32, value: f32) -> Self {
        Self {
            rect,
            value: value.clamp(min, max),
            min,
            max,
            active: false,
            hovered: false,
            dragging: false,
        }
    }
    
    pub fn as_widget(self) -> Widget {
        Widget::Slider(self)
    }
    
    pub fn get_value(&self) -> f32 {
        self.value
    }
    
    pub fn set_value(&mut self, value: f32) {
        self.value = value.clamp(self.min, self.max);
    }
} 