pub mod engine;
use crate::engine::game::Game;

fn main() {
	// let mut game = Game::from_fen("rnbqkbnr/pppppppp/8/8/8/1P6/P1PPPPPP/RNBQKBNR w KQkq - 0 1");
	let mut game = Game::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
	game.start();
}
