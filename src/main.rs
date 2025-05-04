mod creatures;
mod player;
mod renderer;
mod world;
mod worldgen;
mod gui;
mod particle;
mod input;
mod ecs;

use game::game::Game;

mod game;

mod app;  // holds App struct that owns World & Schedule

#[macroquad::main("Voxel Engine")]
async fn main() {
    let mut app = app::App::new();
    app.init().await;   // Initialize resources that require async operations
    app.run().await;    // Run the ECS game loop
}
