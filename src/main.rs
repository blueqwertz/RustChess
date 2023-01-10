pub mod engine;
use engine::bitboard::BitPos;
use engine::movegen::movegen;

fn main() {
    let board = BitPos::from_fen("rnb1kbnr/pppqp1pp/8/1B1p1p2/4P3/2N2N2/PPPP1PPP/R1BQK2R w KQkq - 0 1");
    board.print();
    movegen(board);
}
