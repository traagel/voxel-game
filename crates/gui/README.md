# GUI System for 2D Games

This crate provides a simple retained-mode GUI system for 2D games using macroquad.

## Features

- Retained mode GUI with a clean, simple API
- Multiple windows with automatic layout
- Various widgets: Button, Label, Image, Checkbox, Slider
- Window management (show/hide, positioning)
- Themeable UI with customizable colors and textures
- Responsive input handling
- Nine-patch window rendering for flexible UI

## Asset Structure

The GUI system expects assets in the following structure:

```
assets/gui/
   ├─ fonts/Roboto-Regular.ttf  # Default font
   ├─ window_bg.png             # 3×3 nine-patch (corners 16 px)
   └─ button.png                # 3 states stacked vertically (normal/hover/pressed)
```

If these assets are not available, the system will generate default textures.

## Basic Usage

```rust
use gui::{GuiContext, Theme, widgets::*, Rect};

// Initialize the GUI system
async fn setup_gui() -> Result<GuiContext, Box<dyn std::error::Error>> {
    // Load the GUI theme
    let theme = Theme::load().await?;
    
    // Create the GUI context
    let mut gui_context = GuiContext::new(theme);
    
    // Create a window
    let window_id = gui_context.spawn_window("My Window", Rect::new(100.0, 80.0, 300.0, 220.0));
    
    if let Some(win) = gui_context.get_window_mut(window_id) {
        // Add a button
        win.add_widget(Button::new(
            Rect::new(70.0, 80.0, 160.0, 40.0),
            "Click Me"
        ).as_widget());
        
        // Add a label
        win.add_widget(Label::new(
            Rect::new(70.0, 150.0, 0.0, 0.0),
            "Hello, World!",
            16
        ).as_widget());
    }
    
    Ok(gui_context)
}

// In your game loop
fn update_and_render(mut gui_context: &mut GuiContext) {
    // Process input
    let input = gui::GuiInput::from_macroquad();
    gui_context.handle_input(&input);
    
    // Check for button clicks
    if let Some(win) = gui_context.get_window_mut(1) {
        if let Some(Widget::Button(btn)) = win.widgets.get(0) {
            if btn.clicked {
                println!("Button clicked!");
            }
        }
    }
    
    // Render the GUI
    gui_context.draw();
}
```

## Integration with ECS

To integrate with an Entity Component System:

1. Create a resource to hold the GUI context
2. Create systems for input handling, updating, and rendering
3. Add these systems to your ECS schedule

See the example code for a complete ECS integration.

## Customization

The GUI system is designed to be extensible:

- Create custom widgets by extending the `Widget` enum
- Create a custom theme by modifying the `Theme` struct
- Create custom rendering by overriding the draw functions

## License

This code is available under the MIT license. 