pub mod engine;
use crate::engine::game::Game;

fn main() {
    let mut game = Game::new();
    // game.start("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    game.start("rnbqkbnr/pppp1ppp/8/8/2R1p3/8/PPPPPPPP/1NBQKBNR w Kkq - 0 1");
}
