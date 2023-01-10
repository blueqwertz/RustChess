pub mod engine;
use engine::bitboard::*;

fn main() {
    let mut board = engine::bitboard::BitPos::empty();
    board.bp.set_bit(Square::A2);
    board.bp.print();
}
