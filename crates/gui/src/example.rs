use macroquad::prelude::*;
use crate::{GuiContext, Widget, Theme, GuiInput};
use crate::widgets::{Button, Label, Checkbox, Slider, TextAlign};

// An example using our GUI system
pub async fn gui_example() -> Result<(), Box<dyn std::error::Error>> {
    // Load theme
    let theme = Theme::load().await?;
    let mut gui_ctx = GuiContext::new(theme);
    
    // Create a main window
    let main_window_id = gui_ctx.spawn_window("Main Menu", Rect::new(100.0, 80.0, 300.0, 220.0));
    
    // Add widgets to the main window
    if let Some(win) = gui_ctx.get_window_mut(main_window_id) {
        // Add centered buttons
        win.add_centered_button(70.0, 160.0, 40.0, "Start Game");
        win.add_centered_button(120.0, 160.0, 40.0, "Options");
        
        // Add centered label at the bottom
        win.add_widget(Label::centered(
            Rect::new(win.center().x, 180.0, 0.0, 0.0),
            "v0.1 Demo Game",
            16
        ).as_widget());
    }
    
    // Create options window (initially hidden)
    let options_id = gui_ctx.spawn_window("Options", Rect::new(150.0, 100.0, 350.0, 250.0));
    if let Some(win) = gui_ctx.get_window_mut(options_id) {
        // Add widgets to the options window
        let center_x = win.center_x(300.0); // Get center for wider elements
        
        // Add fullscreen checkbox - horizontally centered
        win.add_centered_checkbox(60.0, "Fullscreen", false);
        
        // Add volume controls with proper alignment
        win.add_widget(Label::new(
            Rect::new(center_x - 130.0, 110.0, 0.0, 0.0),
            "Volume:",
            16
        ).as_widget());
        
        win.add_widget(Slider::new(
            Rect::new(center_x - 60.0, 105.0, 180.0, 20.0),
            0.0,
            100.0,
            75.0
        ).as_widget());
        
        // Center the back button
        win.add_centered_button(180.0, 100.0, 30.0, "Back");
        
        // Hide options window initially
        win.set_visible(false);
    }
    
    // Main game loop
    loop {
        clear_background(Color::new(0.1, 0.1, 0.1, 1.0));
        
        // Process input
        let input = GuiInput::from_macroquad();
        gui_ctx.handle_input(&input);
        
        // Check for button clicks
        let mut show_options = false;
        let mut hide_main = false;
        let mut hide_options = false;
        let mut show_main = false;
        
        if let Some(main_win) = gui_ctx.get_window_mut(main_window_id) {
            if let Some(Widget::Button(btn)) = main_win.widgets.get(0) {
                if btn.clicked {
                    println!("Start Game clicked!");
                    // Here you would transition to the actual game
                }
            }
            
            if let Some(Widget::Button(btn)) = main_win.widgets.get(1) {
                if btn.clicked {
                    // Show options window and hide main window
                    show_options = true;
                    hide_main = true;
                }
            }
        }
        
        // Apply visibility changes based on main window interaction
        if show_options {
            if let Some(options_win) = gui_ctx.get_window_mut(options_id) {
                options_win.set_visible(true);
            }
        }
        
        if hide_main {
            if let Some(main_win) = gui_ctx.get_window_mut(main_window_id) {
                main_win.set_visible(false);
            }
        }
        
        // Check for options window interactions
        if let Some(options_win) = gui_ctx.get_window_mut(options_id) {
            if options_win.visible {
                // Check if Back button was clicked
                if let Some(Widget::Button(btn)) = options_win.widgets.get(3) {
                    if btn.clicked {
                        // Hide options window and show main window
                        hide_options = true;
                        show_main = true;
                    }
                }
                
                // Check if fullscreen checkbox was clicked
                if let Some(Widget::Checkbox(chk)) = options_win.widgets.get_mut(0) {
                    if chk.clicked {
                        chk.checked = !chk.checked;
                        println!("Fullscreen: {}", chk.checked);
                        // Here you would actually toggle fullscreen
                    }
                }
                
                // Check slider value
                if let Some(Widget::Slider(slider)) = options_win.widgets.get(2) {
                    if slider.active {
                        println!("Volume: {}", slider.value);
                        // Here you would update game volume
                    }
                }
            }
        }
        
        // Apply visibility changes based on options window interaction
        if hide_options {
            if let Some(options_win) = gui_ctx.get_window_mut(options_id) {
                options_win.set_visible(false);
            }
        }
        
        if show_main {
            if let Some(main_win) = gui_ctx.get_window_mut(main_window_id) {
                main_win.set_visible(true);
            }
        }
        
        // Draw the GUI
        gui_ctx.draw();
        
        // End the frame
        next_frame().await;
    }
} 