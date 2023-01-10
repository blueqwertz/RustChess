pub mod engine;
use engine::bitboard::*;
use engine::movegen::*;

fn main() {
    let mut board = engine::bitboard::BitPos::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    board.white.print();
    board.black.print();
    movegen(board);
}
