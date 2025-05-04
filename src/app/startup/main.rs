use bevy_ecs::prelude::*;
use crate::ecs::{
    components::creature::Creature,
    components::camera::{Camera, WorldMapCamera, LocalMapCamera},
    resources::{
        renderers::{LocalMapRendererRes, WorldMapRendererRes},
        world_map::WorldMapRes, 
        world::WorldRes,
        gui_state::GuiStateRes, 
        window_manager::{
            WindowManagerRes,
            CityInfoStateRes,
            MainMenuStateRes,
            WorldGenWindowStateRes,
            WorkerInfoStateRes,
        },
        game_view::{GameViewRes, GameView},
        particle::ParticlesRes,
    },
};
use crate::gui::windows::window_state::WindowState;

pub async fn init(world: &mut World) {
    // ── Insert renderers as resources ──
    world.insert_resource(LocalMapRendererRes(crate::renderer::local_map_renderer::LocalMapRenderer::default()));
    world.insert_resource(WorldMapRendererRes(
        crate::renderer::world_map_renderer::WorldMapRenderer::new().await
    ));

    // ── Insert other shared resources ──
    world.insert_resource(WorldRes::default());
    
    // Create and insert world map
    let worldgen = crate::worldgen::worldmap::WorldMapGenerator::new(
        42, // seed
        256, // width
        256, // height
        0.02, // feature scale
        None, // default params
    );
    let world_map = worldgen.generate();
    world.insert_resource(WorldMapRes(world_map));
    
    // Insert GUI resources
    let mut gui_state = crate::gui::GuiState::new();
    gui_state.show_ui = true; // Make sure GUI is visible by default
    world.insert_resource(GuiStateRes(gui_state));
    
    world.insert_resource(WindowManagerRes(crate::gui::windows::window_manager::WindowManager::new()));
    
    // Insert individual window state resources
    world.insert_resource(CityInfoStateRes::default());
    
    // Set up main menu
    let mut main_menu = crate::gui::windows::main_menu::MainMenuState::new();
    main_menu.show(); // Make main menu visible by default
    world.insert_resource(MainMenuStateRes(main_menu));
    
    // Set up worldgen window
    let mut worldgen_state = crate::gui::windows::worldgen::WorldGenWindowState::new();
    worldgen_state.show(); // Make worldgen window visible by default
    world.insert_resource(WorldGenWindowStateRes(worldgen_state));
    
    world.insert_resource(WorkerInfoStateRes::default());
    
    // Set default game view to WorldMap instead of MainMenu
    world.insert_resource(GameViewRes {
        active_view: GameView::WorldMap, // Start with WorldMap view for testing
    });
    
    world.insert_resource(ParticlesRes::default());
    
    // Load portraits if needed
    let portraits = crate::gui::windows::city_info::portraits::CivPortraits::load().await;
    world.insert_resource(crate::ecs::resources::portraits::CivPortraitsRes(portraits));

    // ── spawn cameras ──
    world.spawn((Camera::default(), WorldMapCamera));
    world.spawn((Camera::default(), LocalMapCamera));

    // Example: spawn 10 creatures at (0,0)
    for _ in 0..10 {
        world.spawn(Creature::at(0.0, 0.0));
    }
} 