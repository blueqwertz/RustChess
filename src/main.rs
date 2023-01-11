pub mod engine;
use engine::bitboard::BitPos;
use engine::movegen::movegen;

fn main() {
    let board = BitPos::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    // let board = BitPos::from_fen("r1bqkb1r/pppp1ppp/2n2n2/4p3/4P3/3P1N2/PPP2PPP/RNBQKB1R w KQkq - 0 1");
    board.print();
    movegen(board);
}
