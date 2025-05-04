use std::collections::HashMap;
use macroquad::prelude::{Rect, Vec2};
use crate::{Widget, Theme, GuiInput};

pub type WindowId = u32;

pub struct GuiContext {
    pub windows: HashMap<WindowId, Window>,
    pub theme: Theme,
    next_id: WindowId,
}

impl GuiContext {
    pub fn new(theme: Theme) -> Self {
        Self { windows: HashMap::new(), theme, next_id: 1 }
    }

    pub fn spawn_window(&mut self, title: impl Into<String>, rect: Rect) -> WindowId {
        let id = self.next_id;
        self.next_id += 1;
        self.windows.insert(id, Window {
            id,
            title: title.into(),
            rect,
            widgets: Vec::new(),
            visible: true,
            dragging: false,
            drag_offset: Vec2::new(0.0, 0.0),
            last_position: Vec2::new(rect.x, rect.y),
        });
        id
    }

    pub fn handle_input(&mut self, input: &GuiInput) {
        // Store the currently dragged window id to bring it to front
        let mut dragged_window_id = None;
        
        // Process dragging first for active windows
        for (&id, window) in self.windows.iter_mut().filter(|(_, w)| w.visible) {
            // Store the previous position before any potential updates
            let prev_pos = Vec2::new(window.rect.x, window.rect.y);
            
            // Continue dragging if already started
            if window.dragging {
                if input.mouse_down {
                    // Update window position based on mouse movement
                    window.rect.x = input.mouse_pos.x - window.drag_offset.x;
                    window.rect.y = input.mouse_pos.y - window.drag_offset.y;
                    
                    // Keep track of the dragged window
                    dragged_window_id = Some(id);
                } else {
                    // Stop dragging when mouse is released
                    window.dragging = false;
                }
            }
            // Check if starting a new drag in title bar area (assume title bar height is 30px)
            else if input.mouse_pressed &&
                   input.mouse_pos.x >= window.rect.x && 
                   input.mouse_pos.x <= window.rect.x + window.rect.w &&
                   input.mouse_pos.y >= window.rect.y && 
                   input.mouse_pos.y <= window.rect.y + 30.0 {
                
                // Start dragging this window
                window.dragging = true;
                window.drag_offset.x = input.mouse_pos.x - window.rect.x;
                window.drag_offset.y = input.mouse_pos.y - window.rect.y;
                
                // Keep track of the dragged window
                dragged_window_id = Some(id);
            }
            
            // If the window position changed, update all widget positions
            if window.rect.x != prev_pos.x || window.rect.y != prev_pos.y {
                let dx = window.rect.x - window.last_position.x;
                let dy = window.rect.y - window.last_position.y;
                
                // Only update widgets if there's actual movement
                if dx != 0.0 || dy != 0.0 {
                    // Update all widget positions
                    for widget in &mut window.widgets {
                        match widget {
                            Widget::Button(button) => {
                                button.rect.x += dx;
                                button.rect.y += dy;
                            },
                            Widget::Label(label) => {
                                label.rect.x += dx;
                                label.rect.y += dy;
                            },
                            Widget::Checkbox(checkbox) => {
                                checkbox.rect.x += dx;
                                checkbox.rect.y += dy;
                            },
                            Widget::Slider(slider) => {
                                slider.rect.x += dx;
                                slider.rect.y += dy;
                            },
                            Widget::Image(image) => {
                                image.rect.x += dx;
                                image.rect.y += dy;
                            },
                        }
                    }
                    
                    // Store the new position for next frame's comparison
                    window.last_position.x = window.rect.x;
                    window.last_position.y = window.rect.y;
                }
            }
        }
        
        // Bring dragged window to front (will implement later if needed)
        
        // Now handle widget interaction in all windows
        for window in self.windows.values_mut().filter(|w| w.visible) {
            // Skip widget interaction if dragging
            if window.dragging {
                continue;
            }
            
            // Check if mouse is within window bounds
            if input.mouse_pos.x >= window.rect.x 
                && input.mouse_pos.x <= window.rect.x + window.rect.w
                && input.mouse_pos.y >= window.rect.y 
                && input.mouse_pos.y <= window.rect.y + window.rect.h {
                
                // Update widget states
                for widget in &mut window.widgets {
                    match widget {
                        Widget::Button(button) => {
                            // Check if mouse is over button
                            let hovered = input.mouse_pos.x >= button.rect.x 
                                && input.mouse_pos.x <= button.rect.x + button.rect.w
                                && input.mouse_pos.y >= button.rect.y 
                                && input.mouse_pos.y <= button.rect.y + button.rect.h;
                            
                            button.hovered = hovered;
                            button.clicked = hovered && input.mouse_pressed;
                        }
                        Widget::Checkbox(checkbox) => {
                            // Check if mouse is over checkbox
                            let checkbox_size = Vec2::new(20.0, 20.0); // Match the size used in draw.rs
                            let hovered = input.mouse_pos.x >= checkbox.rect.x 
                                && input.mouse_pos.x <= checkbox.rect.x + checkbox_size.x
                                && input.mouse_pos.y >= checkbox.rect.y 
                                && input.mouse_pos.y <= checkbox.rect.y + checkbox_size.y;
                            
                            checkbox.hovered = hovered;
                            
                            // Set clicked state and toggle checked state on click
                            checkbox.clicked = hovered && input.mouse_pressed;
                            
                            // Toggle checkbox on click
                            if checkbox.clicked {
                                checkbox.checked = !checkbox.checked;
                            }
                        }
                        Widget::Slider(slider) => {
                            // Check if mouse is over slider handle
                            let handle_size = Vec2::new(20.0, 20.0); // Approximate handle size
                            let handle_pos = slider.rect.x + (slider.value - slider.min) / (slider.max - slider.min) * slider.rect.w;
                            
                            let hovered = input.mouse_pos.x >= handle_pos - handle_size.x/2.0 
                                && input.mouse_pos.x <= handle_pos + handle_size.x/2.0
                                && input.mouse_pos.y >= slider.rect.y 
                                && input.mouse_pos.y <= slider.rect.y + handle_size.y;
                            
                            slider.hovered = hovered;
                            
                            // Update slider value on drag
                            if (hovered && input.mouse_pressed) || slider.dragging {
                                slider.dragging = input.mouse_down;
                                slider.active = slider.dragging; // Set active state when dragging
                                
                                if input.mouse_down {
                                    // Calculate new value based on mouse position
                                    let ratio = ((input.mouse_pos.x - slider.rect.x) / slider.rect.w).max(0.0).min(1.0);
                                    slider.value = slider.min + ratio * (slider.max - slider.min);
                                }
                            } else if !input.mouse_down {
                                // Reset active state when not dragging
                                slider.active = false;
                            }
                        }
                        // Handle other widget types as needed
                        _ => {}
                    }
                }
            } else {
                // Reset hover/click states when mouse leaves window
                for widget in &mut window.widgets {
                    if let Widget::Button(button) = widget {
                        button.hovered = false;
                        button.clicked = false;
                    }
                    if let Widget::Slider(slider) = widget {
                        if !input.mouse_down {
                            slider.dragging = false;
                        }
                    }
                }
            }
        }
    }

    pub fn draw(&self) { 
        crate::draw::draw_gui(self); 
    }
    
    pub fn get_window_mut(&mut self, id: WindowId) -> Option<&mut Window> {
        self.windows.get_mut(&id)
    }
}

pub struct Window {
    pub id: WindowId,
    pub title: String,
    pub rect: Rect,
    pub widgets: Vec<Widget>,
    pub visible: bool,
    pub dragging: bool,
    pub drag_offset: Vec2,
    pub last_position: Vec2,
}

impl Window {
    pub fn add_widget(&mut self, widget: Widget) {
        self.widgets.push(widget);
    }
    
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
    
    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = title.into();
    }
    
    /// Calculates a centered horizontal position for a widget of the given width
    pub fn center_x(&self, width: f32) -> f32 {
        self.rect.x + (self.rect.w - width) / 2.0
    }
    
    /// Adds a button that is horizontally centered in the window
    pub fn add_centered_button(&mut self, 
                              y: f32, 
                              width: f32, 
                              height: f32, 
                              text: impl Into<String>) {
        let x = self.center_x(width);
        self.add_widget(crate::widgets::Button::new(
            Rect::new(x, y, width, height), 
            text
        ).as_widget());
    }
    
    /// Adds a label that is horizontally centered in the window
    pub fn add_centered_label(&mut self, 
                             y: f32, 
                             text: impl Into<String>, 
                             font_size: u16) {
        let text_str = text.into();
        let x = self.rect.x + (self.rect.w / 2.0); // Center position
        self.add_widget(crate::widgets::Label::new(
            Rect::new(x, y, 0.0, 0.0), 
            text_str,
            font_size
        ).as_widget());
    }
    
    /// Adds a checkbox that is horizontally centered in the window
    pub fn add_centered_checkbox(&mut self, 
                               y: f32, 
                               label: impl Into<String>, 
                               checked: bool) {
        let label_str = label.into();
        let checkbox_size = 20.0; // Standard checkbox size
        let x = self.center_x(checkbox_size + 10.0); // Adjust as needed for your design
        self.add_widget(crate::widgets::Checkbox::new(
            Rect::new(x, y, checkbox_size, checkbox_size), 
            label_str, 
            checked
        ).as_widget());
    }
    
    /// Adds a slider that is horizontally centered in the window
    pub fn add_centered_slider(&mut self, 
                             y: f32, 
                             width: f32, 
                             height: f32, 
                             min: f32, 
                             max: f32, 
                             value: f32) {
        let x = self.center_x(width);
        self.add_widget(crate::widgets::Slider::new(
            Rect::new(x, y, width, height), 
            min, 
            max, 
            value
        ).as_widget());
    }
    
    /// Calculates the position that would center a widget on both x and y axes
    pub fn center_rect(&self, width: f32, height: f32) -> Rect {
        Rect::new(
            self.rect.x + (self.rect.w - width) / 2.0,
            self.rect.y + (self.rect.h - height) / 2.0,
            width,
            height
        )
    }
    
    /// Gets the center position of the window
    pub fn center(&self) -> macroquad::prelude::Vec2 {
        macroquad::prelude::Vec2::new(
            self.rect.x + self.rect.w / 2.0,
            self.rect.y + self.rect.h / 2.0
        )
    }

    /// Add a widget at the center of the window
    pub fn add_centered_widget<F>(&mut self, mut create_widget: F) 
    where
        F: FnMut(macroquad::prelude::Vec2) -> Widget
    {
        let center_pos = self.center();
        self.add_widget(create_widget(center_pos));
    }
} 