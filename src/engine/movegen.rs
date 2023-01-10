
use crate::engine::bitboard::{BitBoard, BitPos, Color, Kind, Square};
const BOARD_MAX:u8 = 63;


pub fn movegen(board: BitPos) {
    let mut moves: Vec<Move> = Vec::new();
    moves.push(Move::new(Color::Black, Kind::Bishop, 0, 1));
    println!("{moves:?}");
    println!("{:?}", KingMove(28,board))
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


fn KingMove(position: u8, boards: BitPos) -> Vec<Move>{
    // -9,-8, -7
    // -1, X, +1
    // +7,+8,+9
    let options: [i16; 8] = [-9,-8,-7,-1,1,7,8,9];
    let mut possiblemoves: Vec<Move> = Vec::new();

    for option in options{
        let new_position = position as i16 + option;
        if new_position > BOARD_MAX as i16 { continue}
        if new_position < u8::MIN as i16 {continue}
        if position % 8 == 0 && (option == -9 || option == -1 || option == 7) { continue }
        if position % 8 == 7 && (option == -7 || option == 1 || option == 9) {continue}
        let new_position = new_position as u8;

        possiblemoves.push(Move::new(Color::Black,Kind::King,position,new_position))
    };
    println!("{} possible moves", possiblemoves.len());
    possiblemoves

}