pub mod engine;
use crate::engine::game::Game;

fn main() {
    let game = Game::new();
    game.start();
}
