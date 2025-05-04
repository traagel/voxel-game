use bevy_ecs::prelude::*;
use crate::ecs::resources::{
    GuiContextRes,
    window_manager::MainMenuStateRes,
    game_view::{GameViewRes, GameView},
};
use gui::widgets::{Widget, Button};

/// System to update the GUI based on game state
pub fn update_gui_system(
    mut gui_context: ResMut<GuiContextRes>,
    mut main_menu: ResMut<MainMenuStateRes>,
    mut game_view: ResMut<GameViewRes>,
) {
    // Variables to track requested actions that require window state changes
    let mut show_options = false;
    let mut hide_main = false;
    let mut hide_options = false;
    let mut show_main = false;
    let mut start_game = false;

    // Process GUI interactions
    // First, get main menu window which should be ID 1
    if let Some(main_win) = gui_context.context_mut().windows.get_mut(&1) {
        // Make the window visible based on main menu state
        main_win.set_visible(main_menu.show_main);
        
        // Check Start Game button (index 1 since we have a title label at index 0)
        if let Some(Widget::Button(btn)) = main_win.widgets.get(1) {
            if btn.clicked {
                // Flag to start the game
                start_game = true;
            }
        }
        
        // Check Options button
        if let Some(Widget::Button(btn)) = main_win.widgets.get(2) {
            if btn.clicked {
                // Flag to show options and hide main menu
                show_options = true;
                hide_main = true;
            }
        }
        
        // Check Quit button
        if let Some(Widget::Button(btn)) = main_win.widgets.get(3) {
            if btn.clicked {
                // Exit the game
                std::process::exit(0);
            }
        }
    }
    
    // Apply changes from main menu interaction
    if show_options {
        if let Some(options_win) = gui_context.context_mut().windows.get_mut(&2) {
            options_win.set_visible(true);
        }
    }
    
    if hide_main {
        if let Some(main_win) = gui_context.context_mut().windows.get_mut(&1) {
            main_win.set_visible(false);
        }
    }
    
    // Process options window interactions (should be ID 2)
    if let Some(options_win) = gui_context.context_mut().windows.get_mut(&2) {
        // Check Back button (index 3)
        if let Some(Widget::Button(btn)) = options_win.widgets.get(3) {
            if btn.clicked {
                // Flag to hide options and show main menu
                hide_options = true;
                show_main = true;
            }
        }
        
        // Handle other options here, e.g. checkbox, slider, etc.
    }
    
    // Apply changes from options window interaction
    if hide_options {
        if let Some(options_win) = gui_context.context_mut().windows.get_mut(&2) {
            options_win.set_visible(false);
        }
    }
    
    if show_main {
        if let Some(main_win) = gui_context.context_mut().windows.get_mut(&1) {
            main_win.set_visible(true);
        }
    }
    
    // Handle game state changes
    if start_game {
        main_menu.hide();
        game_view.active_view = GameView::WorldMap;
    }
} 