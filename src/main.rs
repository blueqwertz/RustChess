pub mod engine;
use engine::bitboard::BitPos;
use engine::movegen::movegen;
use crate::engine::bitboard::Color;

fn main() {
    let board = BitPos::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    // let board = BitPos::from_fen("rnbqkbnr/pppp2pp/8/8/8/4p1p1/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    board.print();
    movegen(board, Color::White as u8);
}
