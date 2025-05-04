use bevy_ecs::prelude::*;
use crate::ecs::resources::GuiContextRes;
use gui::GuiInput;

/// System to handle input for our GUI system
pub fn handle_gui_system_input(
    mut gui_context: ResMut<GuiContextRes>,
) {
    // Create the input object from macroquad
    let input = GuiInput::from_macroquad();
    
    // Pass it to our GUI context to handle
    gui_context.context_mut().handle_input(&input);
} 