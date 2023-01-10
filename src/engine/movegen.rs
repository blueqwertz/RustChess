use std::collections::HashMap;
use crate::engine::bitboard::{BitBoard, BitPos, Color, Kind, Square};

pub fn movegen(board: BitPos) {
    let mut moves: Vec<Move> = Vec::new();



    for pos_move in moves {
        pos_move.print();
    }
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

    pub fn print(&self) {
        let color = &self.color;
        let kind = &self.kind;
        let from = Square::from(*&self.from);
        let to = Square::from(*&self.to);
        println!("{color:?}, {kind:?}, {from:?} -> {to:?}");
    }
}

fn pawn_moves(position: u8, color: u8, kind: u8, boards: BitPos) -> Vec<Move> {
    //     -16*
    // -9* - 8 -7*

    let pos_moves: Vec<Move> = Vec::new();

    // generate moves

    pos_moves
}

fn knight_moves(position: u8, boards: BitPos) -> Vec<Move> {
    let pos_moves: Vec<Move> = Vec::new();

    // generate moves

    pos_moves
}

fn rook_moves(position: u8, boards: BitPos) -> Vec<Move> {
    let pos_moves: Vec<Move> = Vec::new();

    // generate moves

    pos_moves
}