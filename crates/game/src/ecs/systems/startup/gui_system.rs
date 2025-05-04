use bevy_ecs::prelude::*;
use gui::{GuiContext, Theme, widgets::*, Rect};
use crate::ecs::resources::{GuiContextRes, window_manager::MainMenuStateRes};

/// Initialize our GUI system
pub async fn init_gui_system(world: &mut World) -> anyhow::Result<()> {
    // Load the GUI theme
    let theme = Theme::load().await?;
    
    // Create the GUI context
    let mut gui_context = GuiContext::new(theme);
    
    // Create a main menu window
    let main_window_id = gui_context.spawn_window("Main Menu", Rect::new(100.0, 80.0, 300.0, 220.0));
    
    if let Some(win) = gui_context.get_window_mut(main_window_id) {
        // Add a title label
        win.add_widget(Label::new(
            Rect::new(70.0, 40.0, 0.0, 0.0),
            "Voxel Game",
            24
        ).as_widget());
        
        // Start Game button
        win.add_widget(Button::new(
            Rect::new(70.0, 80.0, 160.0, 40.0),
            "Start Game"
        ).as_widget());
        
        // Options button
        win.add_widget(Button::new(
            Rect::new(70.0, 130.0, 160.0, 40.0),
            "Options"
        ).as_widget());
        
        // Quit button
        win.add_widget(Button::new(
            Rect::new(70.0, 180.0, 160.0, 40.0),
            "Quit"
        ).as_widget());
    }
    
    // Create an options window (initially hidden)
    let options_id = gui_context.spawn_window("Options", Rect::new(150.0, 100.0, 350.0, 250.0));
    if let Some(win) = gui_context.get_window_mut(options_id) {
        // Fullscreen checkbox
        win.add_widget(Checkbox::new(
            Rect::new(50.0, 60.0, 20.0, 20.0),
            "Fullscreen",
            false
        ).as_widget());
        
        // Volume label
        win.add_widget(Label::new(
            Rect::new(50.0, 100.0, 0.0, 0.0),
            "Volume:",
            16
        ).as_widget());
        
        // Volume slider
        win.add_widget(Slider::new(
            Rect::new(120.0, 100.0, 180.0, 20.0),
            0.0,
            100.0,
            75.0
        ).as_widget());
        
        // Back button
        win.add_widget(Button::new(
            Rect::new(125.0, 180.0, 100.0, 30.0),
            "Back"
        ).as_widget());
        
        // Hide options window initially
        win.set_visible(false);
    }
    
    // Insert the GUI context as a resource
    world.insert_resource(GuiContextRes::new(gui_context));
    
    // Make sure the main menu state is properly initialized
    if let Some(mut main_menu_state) = world.get_resource_mut::<MainMenuStateRes>() {
        main_menu_state.show();
    }
    
    Ok(())
} 