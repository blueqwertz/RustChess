pub mod engine;
use crate::engine::game::Game;

fn main() {
    let mut game = Game::new();
    // game.start("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    game.start("rnbqkb1r/p1ppp1n1/6Rp/1Qp5/7P/5PN1/PPPPP1pP/1NB1K2B w kq - 0 1");
}
