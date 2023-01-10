
use crate::engine::bitboard::{BitBoard, BitPos, Color, Kind, Square};

pub fn movegen(board: BitPos) {
    let mut moves: Vec<Move> = Vec::new();
    moves.push(Move::new(Color::Black, Kind::Bishop, 0, 1));
    println!("{moves:?}");
}

#[derive(Debug)]
pub struct Move{
    color: Color,
    kind: Kind,
    from: u8,
    to: u8
}

impl Move {
    pub fn new(color: Color, kind: Kind, from: u8, to: u8) -> Self {
        Self{color, kind, from, to}
    }

}