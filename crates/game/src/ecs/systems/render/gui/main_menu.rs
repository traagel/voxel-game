use macroquad::prelude::*;
use crate::ecs::resources::window_manager::MainMenuStateRes;
use gui::{GuiContext, Theme, GuiInput, WindowId, Widget};
use gui::widgets::{Button, Label, Checkbox, Slider, TextAlign};
use std::sync::Mutex;
use once_cell::sync::Lazy;

// Store GUI context and window IDs across frames
pub struct MainMenuGui {
    gui_ctx: GuiContext,
    main_menu_id: WindowId,
    settings_id: WindowId,
    initialized: bool,
}

impl MainMenuGui {
    pub fn new() -> Self {
        Self {
            gui_ctx: GuiContext::new(Theme::default()),  // Will be properly initialized later
            main_menu_id: 0,
            settings_id: 0,
            initialized: false,
        }
    }

    pub async fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.initialized {
            return Ok(());
        }

        println!("Initializing MainMenuGui asynchronously...");

        // Load the theme
        let theme = Theme::load().await?;
        self.gui_ctx = GuiContext::new(theme);
        
        // Call the common initialization code
        self.init_windows();

        self.initialized = true;
        println!("MainMenuGui async initialization complete");
        Ok(())
    }

    // Synchronous version that uses a default theme
    pub fn initialize_sync(&mut self) {
        if self.initialized {
            return;
        }

        println!("Initializing MainMenuGui synchronously with default theme...");
        
        // Create with default theme
        self.gui_ctx = GuiContext::new(Theme::default());
        
        // Call the common initialization code
        self.init_windows();

        self.initialized = true;
        println!("MainMenuGui sync initialization complete");
    }

    // Common code for both initialization methods
    fn init_windows(&mut self) {
        // Create main menu window
        let win_size = vec2(400.0, 300.0);
        let win_pos_x = (screen_width() - win_size.x) / 2.0;
        let win_pos_y = (screen_height() - win_size.y) / 2.0;
        
        self.main_menu_id = self.gui_ctx.spawn_window(
            "Main Menu", 
            Rect::new(win_pos_x, win_pos_y, win_size.x, win_size.y)
        );

        // Create settings window
        let settings_size = vec2(350.0, 250.0);
        let settings_pos_x = (screen_width() - settings_size.x) / 2.0;
        let settings_pos_y = (screen_height() - settings_size.y) / 2.0;
        
        self.settings_id = self.gui_ctx.spawn_window(
            "Settings", 
            Rect::new(settings_pos_x, settings_pos_y, settings_size.x, settings_size.y)
        );

        // Add widgets to main menu
        if let Some(win) = self.gui_ctx.get_window_mut(self.main_menu_id) {
            // Set window to be visible by default
            win.set_visible(true);
            
            // Add centered buttons with consistent spacing
            win.add_centered_button(100.0, 160.0, 40.0, "Resume (Esc)");
            win.add_centered_button(150.0, 160.0, 40.0, "Settings");
            
            // Add debug section
            let debug_y = 210.0;
            win.add_widget(Label::new(
                Rect::new(win.rect.x + 20.0, debug_y, 0.0, 0.0),
                "Debug Info",
                16
            ).as_widget());
            
            win.add_widget(Label::new(
                Rect::new(win.rect.x + 20.0, debug_y + 25.0, 0.0, 0.0),
                &format!("Screen: {}x{}", screen_width() as i32, screen_height() as i32),
                14
            ).as_widget());
            
            // Add test button
            win.add_centered_button(debug_y + 60.0, 180.0, 30.0, "DEBUG: Test Button");
            
            println!("Main menu window created with ID: {} and visibility: {}", self.main_menu_id, win.visible);
        } else {
            println!("Failed to get main menu window after creation");
        }

        // Add widgets to settings window
        if let Some(win) = self.gui_ctx.get_window_mut(self.settings_id) {
            // Add fullscreen checkbox
            win.add_centered_checkbox(80.0, "Fullscreen", false);
            
            // Add volume slider
            let center_x = win.center_x(280.0);
            
            win.add_widget(Label::new(
                Rect::new(center_x - 120.0, 120.0, 0.0, 0.0),
                "Volume:",
                16
            ).as_widget());
            
            win.add_widget(Slider::new(
                Rect::new(center_x - 60.0, 115.0, 160.0, 20.0),
                0.0,
                100.0,
                75.0
            ).as_widget());
            
            // Add back button
            win.add_centered_button(180.0, 100.0, 30.0, "Close Settings");
            
            // Hide settings window initially
            win.set_visible(false);
        }
    }
}

// Replace unsafe static with thread-safe Lazy<Mutex<>>
static MAIN_MENU_GUI: Lazy<Mutex<MainMenuGui>> = Lazy::new(|| {
    Mutex::new(MainMenuGui::new())
});

/// Draw the main menu window
pub async fn draw_main_menu(state: &mut MainMenuStateRes) -> Result<(), Box<dyn std::error::Error>> {
    // Get a mutex lock on the GUI
    let mut gui = MAIN_MENU_GUI.lock().unwrap();
    
    // Make sure GUI is initialized
    if !gui.initialized {
        println!("Initializing GUI in draw_main_menu");
        gui.initialize().await?;
    }
    
    // Draw a debug indicator to show this method was called
    draw_rectangle(200.0, 10.0, 200.0, 30.0, Color::new(0.2, 0.7, 0.3, 0.7));
    draw_text("MainMenu.draw() called", 210.0, 30.0, 16.0, WHITE);
    
    // Draw visibility status for debugging
    draw_text(&format!("Menu visible: {}", state.show_main), 210.0, 50.0, 16.0, WHITE);
    
    // Store window IDs in local variables to avoid borrow checker issues
    let main_menu_id = gui.main_menu_id;
    let settings_id = gui.settings_id;
    
    // Force show for debugging if needed
    state.show_main = true; // Force show menu
    
    // Update window visibility based on state
    let mut main_window_visible = false;
    if let Some(main_win) = gui.gui_ctx.get_window_mut(main_menu_id) {
        main_win.set_visible(state.show_main);
        main_window_visible = main_win.visible;
        
        // Update dynamic debug labels
        if state.show_main && main_win.widgets.len() >= 5 {
            if let Widget::Label(label) = &mut main_win.widgets[4] {
                label.text = format!("show_main: {}", state.show_main);
            }
            
            if main_win.widgets.len() >= 6 {
                if let Widget::Label(label) = &mut main_win.widgets[5] {
                    label.text = format!("show_settings: {}", state.show_settings);
                }
            }
        }
    } else {
        println!("Warning: Main menu window not found with ID: {}", main_menu_id);
    }
    
    if let Some(settings_win) = gui.gui_ctx.get_window_mut(settings_id) {
        settings_win.set_visible(state.show_settings);
    }
    
    // Process input
    let input = GuiInput::from_macroquad();
    gui.gui_ctx.handle_input(&input);
    
    // Check for button clicks in main menu
    if let Some(main_win) = gui.gui_ctx.get_window_mut(main_menu_id) {
        if main_win.visible {
            // Resume button
            if let Some(Widget::Button(btn)) = main_win.widgets.get(0) {
                if btn.clicked {
                    state.toggle();
                }
            }
            
            // Settings button
            if let Some(Widget::Button(btn)) = main_win.widgets.get(1) {
                if btn.clicked {
                    state.show_settings = true;
                }
            }
            
            // Test button
            if let Some(Widget::Button(btn)) = main_win.widgets.get_mut(6) {
                if btn.clicked {
                    println!("Test button clicked!");
                }
            }
        }
    }
    
    // Check for button clicks in settings menu
    if let Some(settings_win) = gui.gui_ctx.get_window_mut(settings_id) {
        if settings_win.visible {
            // Close settings button
            if let Some(Widget::Button(btn)) = settings_win.widgets.get(3) {
                if btn.clicked {
                    state.show_settings = false;
                }
            }
            
            // Fullscreen checkbox
            if let Some(Widget::Checkbox(chk)) = settings_win.widgets.get(0) {
                if chk.clicked {
                    // Would toggle fullscreen here
                    println!("Fullscreen toggled: {}", chk.checked);
                }
            }
            
            // Volume slider
            if let Some(Widget::Slider(slider)) = settings_win.widgets.get(2) {
                if slider.active {
                    // Would adjust volume here
                    println!("Volume changed: {:.1}", slider.value);
                }
            }
        }
    }
    
    // Draw the GUI
    gui.gui_ctx.draw();
    
    // Show debug info after drawing
    draw_text(&format!("Main window drawn and visible: {}", main_window_visible), 
              210.0, 70.0, 16.0, YELLOW);
    
    Ok(())
}

/// Non-async version of draw_main_menu that can be called from synchronous code
pub fn draw_main_menu_sync(state: &mut MainMenuStateRes) {
    // Get a mutex lock on the GUI
    let mut gui_lock = MAIN_MENU_GUI.lock().unwrap();
    let gui = &mut *gui_lock;
    
    // Make sure GUI is initialized synchronously
    if !gui.initialized {
        println!("Initializing GUI synchronously in draw_main_menu_sync");
        gui.initialize_sync();
    }
    
    // Draw a debug indicator to show this method was called
    draw_rectangle(200.0, 10.0, 200.0, 30.0, Color::new(0.2, 0.7, 0.3, 0.7));
    draw_text("MainMenu.draw_sync() called", 210.0, 30.0, 16.0, WHITE);
    
    // Draw visibility status for debugging
    draw_text(&format!("Menu visible: {}", state.show_main), 210.0, 50.0, 16.0, WHITE);
    
    // Store window IDs in local variables to avoid borrow checker issues
    let main_menu_id = gui.main_menu_id;
    let settings_id = gui.settings_id;
    
    // Force show for debugging if needed
    state.show_main = true; // Force show menu
    
    // Update window visibility based on state
    let mut main_window_visible = false;
    if let Some(main_win) = gui.gui_ctx.get_window_mut(main_menu_id) {
        main_win.set_visible(state.show_main);
        main_window_visible = main_win.visible;
        
        // Update dynamic debug labels
        if state.show_main && main_win.widgets.len() >= 5 {
            if let Widget::Label(label) = &mut main_win.widgets[4] {
                label.text = format!("show_main: {}", state.show_main);
            }
            
            if main_win.widgets.len() >= 6 {
                if let Widget::Label(label) = &mut main_win.widgets[5] {
                    label.text = format!("show_settings: {}", state.show_settings);
                }
            }
        }
    } else {
        println!("Warning: Main menu window not found with ID: {}", main_menu_id);
    }
    
    if let Some(settings_win) = gui.gui_ctx.get_window_mut(settings_id) {
        settings_win.set_visible(state.show_settings);
    }
    
    // Process input
    let input = GuiInput::from_macroquad();
    gui.gui_ctx.handle_input(&input);
    
    // Process button clicks (same as in async version)
    if let Some(main_win) = gui.gui_ctx.get_window_mut(main_menu_id) {
        if main_win.visible {
            // Resume button
            if let Some(Widget::Button(btn)) = main_win.widgets.get(0) {
                if btn.clicked {
                    state.toggle();
                }
            }
            
            // Settings button
            if let Some(Widget::Button(btn)) = main_win.widgets.get(1) {
                if btn.clicked {
                    state.show_settings = true;
                }
            }
            
            // Test button
            if let Some(Widget::Button(btn)) = main_win.widgets.get_mut(6) {
                if btn.clicked {
                    println!("Test button clicked!");
                }
            }
        }
    }
    
    // Check for button clicks in settings menu
    if let Some(settings_win) = gui.gui_ctx.get_window_mut(settings_id) {
        if settings_win.visible {
            // Close settings button
            if let Some(Widget::Button(btn)) = settings_win.widgets.get(3) {
                if btn.clicked {
                    state.show_settings = false;
                }
            }
            
            // Fullscreen checkbox
            if let Some(Widget::Checkbox(chk)) = settings_win.widgets.get(0) {
                if chk.clicked {
                    // Would toggle fullscreen here
                    println!("Fullscreen toggled: {}", chk.checked);
                }
            }
            
            // Volume slider
            if let Some(Widget::Slider(slider)) = settings_win.widgets.get(2) {
                if slider.active {
                    // Would adjust volume here
                    println!("Volume changed: {:.1}", slider.value);
                }
            }
        }
    }
    
    // Draw the GUI
    gui.gui_ctx.draw();
    
    // Show debug info after drawing
    draw_text(&format!("Main window drawn and visible: {}", main_window_visible), 
              210.0, 70.0, 16.0, YELLOW);
} 