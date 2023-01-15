pub mod engine;
use crate::engine::game::Game;

fn main() {
    let mut game = Game::new();
    // game.start("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    game.start("rnbq1b1r/p2pp1n1/3p2Rp/pkp1Q3/7P/5PN1/1PPPP1pP/1NB1K2B w ha - 0 1");
}
