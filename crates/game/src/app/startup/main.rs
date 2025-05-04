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
            CityInfoStateRes,
            MainMenuStateRes,
            WorldGenWindowStateRes,
            WorkerInfoStateRes,
        },
        game_view::{GameViewRes, GameView},
        particle::ParticlesRes,
        GuiContextRes,
    },
    systems::startup::{load_assets, init_gui_system},
};
use gui::{GuiContext, Theme};

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
    let gui_state = GuiStateRes::new();
    world.insert_resource(gui_state);
    
    // Insert individual window state resources
    world.insert_resource(CityInfoStateRes::default());
    
    // Set up main menu
    let mut main_menu = MainMenuStateRes::new();
    main_menu.show(); // Make main menu visible by default
    world.insert_resource(main_menu);
    
    // Set up worldgen window
    let mut worldgen_state = WorldGenWindowStateRes::new();
    worldgen_state.show(); // Make worldgen window visible by default
    world.insert_resource(worldgen_state);
    
    world.insert_resource(WorkerInfoStateRes::default());
    
    // Set default game view to WorldMap instead of MainMenu
    world.insert_resource(GameViewRes {
        active_view: GameView::WorldMap, // Start with WorldMap view for testing
    });
    
    world.insert_resource(ParticlesRes::default());
    
    // Load all game assets
    load_assets(world).await;
    
    // Initialize our GUI system
    match init_gui_system(world).await {
        Ok(_) => println!("GUI system initialized successfully"),
        Err(err) => {
            eprintln!("Failed to initialize GUI system: {}", err);
            
            // Create a fallback GUI context if initialization failed
            if world.get_resource::<GuiContextRes>().is_none() {
                // Try to load a simple theme with fallback mechanisms
                match Theme::load().await {
                    Ok(theme) => {
                        let gui_context = GuiContext::new(theme);
                        world.insert_resource(GuiContextRes::new(gui_context));
                        println!("Created fallback GUI context");
                    },
                    Err(e) => {
                        eprintln!("Cannot create even a fallback GUI: {}", e);
                        // Game will likely crash, but at least we tried
                    }
                }
            }
        }
    }

    // ── spawn cameras ──
    world.spawn((Camera::default(), WorldMapCamera));
    world.spawn((Camera::default(), LocalMapCamera));

    // Example: spawn 10 creatures at (0,0)
    for _ in 0..10 {
        world.spawn(Creature::at(0.0, 0.0));
    }
} 