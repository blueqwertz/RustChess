pub mod engine;
use crate::engine::game::Game;

fn main() {
    let mut game = Game::new();
    // game.start("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    game.start("rnbqkb1r/ppppppn1/4p1R1/8/2Q4P/5PN1/PPPPP1pP/RNB1K2B w Qkq - 0 1");
}
