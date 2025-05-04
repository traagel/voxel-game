use bevy_ecs::prelude::*;
use bevy_ecs::schedule::*;
use crate::app::App;
use crate::app::schedule::Stage;
use crate::ecs::resources::window_manager::*;
use crate::ecs::resources::gui_state::GuiStateRes;
use crate::ecs::systems::input::handle_gui_input;
use crate::ecs::systems::render::draw_gui;
use crate::ecs::systems::update::update_window_interactions;

// Define a custom plugin trait since we're not using the full Bevy engine
pub trait Plugin {
    fn build(&self, app: &mut App);
}

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        // This function can't be directly implemented since our App struct doesn't have these methods.
        // Instead, we need to modify our approach.
        
        // In a real plugin system, we would do something like this:
        // Register all GUI-related resources
        // app.init_resource::<GuiStateRes>()
        //     .init_resource::<WindowManagerRes>()
        //     .init_resource::<CityInfoStateRes>()
        //     .init_resource::<MainMenuStateRes>()
        //     .init_resource::<WorldGenWindowStateRes>()
        //     .init_resource::<WorkerInfoStateRes>();
            
        // Add GUI-related systems
        // app.add_systems(PreUpdate, handle_gui_input)
        //     .add_systems(Update, update_window_interactions)
        //     .add_systems(PostUpdate, draw_gui);
    }
}

// A convenience function to register the GUI systems with our custom App
pub fn register_gui_systems(schedule: &mut Schedule) {
    // This more directly matches our app structure
    
    // Resources were already added in startup/main.rs
    
    // Add GUI systems to the schedule
    schedule.add_systems(handle_gui_input.in_set(Stage::Input));
    schedule.add_systems(update_window_interactions.in_set(Stage::Update));
    
    // Configure draw_gui to run after all other rendering systems using an explicit ordering constraint
    // This ensures the world rendering happens first, then GUI is drawn on top
    schedule.add_systems(
        draw_gui
            .in_set(Stage::Render)
            .after(crate::ecs::systems::render::draw_world_map)
            .after(crate::ecs::systems::render::draw_local_map)
    );
} 