pub mod engine;
use crate::engine::game::Game;

fn main() {
    let game = Game::new();
    // game.start("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    game.start("rnbqkbnr/ppppppp1/8/8/7p/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
}
