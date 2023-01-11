use std::time::Instant;
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
    //Langsamer
    /*
    let now = Instant::now();
    println!("{}", now.elapsed().as_nanos());


    let now = Instant::now();

    let mut possiblemoves: Vec<Move> = Vec::new();
    let options: [i16; 8] = [-9,-8,-7,-1,1,7,8,9];


    for option in options{
        let new_position = position as i16 + option;
        if new_position > BOARD_MAX as i16 { continue}
        if new_position < u8::MIN as i16 {continue}
        if position % 8 == 0 && (option == -9 || option == -1 || option == 7) { continue }
        if position % 8 == 7 && (option == -7 || option == 1 || option == 9) {continue}
        let new_position = new_position as u8;

        possiblemoves.push(Move::new(Color::Black,Kind::King,position,new_position))
    };
    println!("{}", now.elapsed().as_nanos());
    println!("{} possible moves", possiblemoves.len());
    //possiblemoves
     */

    let now = Instant::now();

    let mut possiblemoves: Vec<Move> = Vec::new();
    let has_space_left =  position % 8 != 0;
    let has_space_right = position % 8 != 7;
    let has_space_up = position > 7;
    let has_space_down = position < 56;
    /*
    println!("{:?}", has_space_left);
    println!("{:?}", has_space_right);
    println!("{:?}", has_space_up);
    println!("{:?}", has_space_down);
    */
    if has_space_up{
        let one_row_up = position - 8;
        possiblemoves.push(Move::new(Color::Black,Kind::King,position, one_row_up));

        if has_space_left{
            let one_up_left = position - 9;
            possiblemoves.push(Move::new(Color::Black,Kind::King,position, one_up_left))
        }
        if has_space_right{
            let one_up_right = position - 7;
            possiblemoves.push(Move::new(Color::Black,Kind::King,position, one_up_right))
        }
    }
    if has_space_down{
        let one_row_down = position + 8;
        possiblemoves.push(Move::new(Color::Black,Kind::King,position, one_row_down));

        if has_space_left{
            let one_down_left = position + 7;
            possiblemoves.push(Move::new(Color::Black,Kind::King,position, one_down_left))
        }
        if has_space_right{
            let one_down_right = position + 9;
            possiblemoves.push(Move::new(Color::Black,Kind::King,position, one_down_right))
        }
    }
    if has_space_left{
        let one_row_left = position - 1;
        possiblemoves.push(Move::new(Color::Black,Kind::King,position, one_row_left));
    }
    if has_space_right{
        let one_row_right = position + 1;
        possiblemoves.push(Move::new(Color::Black,Kind::King,position, one_row_right))
    }
    println!("{}", now.elapsed().as_nanos());

    // -9,-8, -7
    // -1, X, +1
    // +7,+8, +9

    println!("{} possible moves", possiblemoves.len());
    possiblemoves
}