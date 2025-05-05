mod creatures;
mod player;
mod renderer;
mod world;
mod worldgen;
mod gui;
mod particle;
mod input;

use game::Game;

mod game;

#[macroquad::main("Voxel Engine")]
async fn main() {
    let mut game = Game::new().await;
    game.init();
    game.run().await;
}
