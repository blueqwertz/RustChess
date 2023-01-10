pub mod engine;
use engine::bitboard::*;

fn main() {
    let mut board = engine::bitboard::BitPos::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    board.white.print();
    board.black.print();
}
