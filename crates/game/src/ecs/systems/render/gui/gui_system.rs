use bevy_ecs::prelude::*;
use bevy_ecs::system::SystemState;
use crate::ecs::resources::{
    gui_state::GuiStateRes,
    game_view::GameViewRes,
    window_manager::{
        CityInfoStateRes,
        MainMenuStateRes,
        WorldGenWindowStateRes,
        WorkerInfoStateRes,
    },
    world_map::WorldMapRes,
    portraits::CivPortraitsRes,
};
use macroquad::prelude::*;

/// System that handles GUI rendering
pub fn gui_system(world: &mut World) {
    // Create a SystemState for accessing all our resources
    // This properly handles borrowing from World
    let mut system_state: SystemState<(
        // Immutable resources
        Res<GuiStateRes>,
        Res<WorldMapRes>,
        Option<Res<CivPortraitsRes>>,
        // Mutable resources
        ResMut<GameViewRes>,
        ResMut<CityInfoStateRes>,
        ResMut<MainMenuStateRes>,
        ResMut<WorldGenWindowStateRes>,
        ResMut<WorkerInfoStateRes>,
    )> = SystemState::new(world);
    
    // Get all resources from the world through the SystemState
    let (
        gui_state, 
        world_map, 
        portraits, 
        mut game_view, 
        mut city_info,
        mut main_menu, 
        mut worldgen, 
        mut worker_info
    ) = system_state.get_mut(world);
    
    // Execute the GUI drawing code directly
    if let Err(err) = execute_gui_draw(
        &gui_state,
        &mut game_view,
        &mut city_info,
        &mut main_menu,
        &mut worldgen,
        &mut worker_info,
        &world_map,
        portraits.as_deref(),
    ) {
        eprintln!("Error drawing GUI: {}", err);
    }
    
    // Apply any changes back to the world
    system_state.apply(world);
}

// Helper function to execute GUI drawing
fn execute_gui_draw(
    gui_state: &GuiStateRes,
    game_view: &mut GameViewRes,
    city_info: &mut CityInfoStateRes,
    main_menu: &mut MainMenuStateRes,
    worldgen: &mut WorldGenWindowStateRes,
    worker_info: &mut WorkerInfoStateRes,
    world_map: &WorldMapRes,
    portraits: Option<&CivPortraitsRes>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Save camera state by pushing a new camera stack context
    push_camera_state();
    
    // Reset to default camera for UI drawing
    set_default_camera();
    
    // Draw debug information
    super::debug::draw_debug_info(gui_state, game_view, main_menu, worldgen);
    
    // Handle key inputs for view switching
    super::input::handle_key_inputs(main_menu, worldgen, game_view);
    
    // Skip drawing other GUI elements if GUI is disabled
    if !gui_state.show_ui {
        // Restore previous camera before returning
        pop_camera_state();
        return Ok(());
    }

    // Draw the main menu if it's visible or we're in MainMenu view
    if main_menu.is_visible() || matches!(game_view.active_view, crate::ecs::resources::game_view::GameView::MainMenu) {
        // Call our new synchronous version of the main menu drawing function
        super::draw_main_menu_sync(main_menu);
    }
    
    // Call other drawing functions that aren't async
    match game_view.active_view {
        crate::ecs::resources::game_view::GameView::WorldGen => {
            super::draw_worldgen_window(worldgen);
        },
        crate::ecs::resources::game_view::GameView::WorldMap => {
            super::draw_worldgen_window(worldgen);
            
            // Draw city info if needed
            let city_info_selected = city_info.selected_city.clone();
            if let (Some(city), Some(portraits_res)) = (city_info_selected.as_ref(), portraits) {
                super::draw_city_info_window(city_info, city, &portraits_res.0, &world_map.0);
            }
        },
        crate::ecs::resources::game_view::GameView::LocalMap => {
            super::draw_worker_info_window(worker_info);
        },
        _ => {}
    }
    
    // Restore the previous camera state
    pop_camera_state();
    
    Ok(())
} 