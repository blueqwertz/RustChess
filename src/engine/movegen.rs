use crate::engine::bitboard::{BitBoard, BitPos, Color, Kind, Square};
use std::time::Instant;

pub fn movegen(board: &mut BitPos, color: u8, knight_boards: [BitBoard; 64]) -> Vec<Move> {
    let now = Instant::now();

    let mut moves = Vec::new();

    match color {
        0 => {
            for i in 0u8..64u8 {
                if board.white.get_bit(i) {
                    if board.wp.get_bit(i) {
                        let pos_moves = pawn_moves(i, color,  board);
                        for pos in pos_moves {
                            moves.push(pos)
                        }
                    } else if board.wn.get_bit(i) {
                        let pos_moves = knight_moves(i, color,  board, knight_boards);
                        for pos in pos_moves {
                            moves.push(pos)
                        }
                    }
                }
            }
        },
        1 => {
            for i in 0u8..64u8 {
                if board.black.get_bit(i) {
                    if board.bp.get_bit(i) {
                        let pos_moves = pawn_moves(i, color, board);
                        for pos in pos_moves {
                            moves.push(pos)
                        }
                    } else if board.bn.get_bit(i) {
                        let pos_moves = knight_moves(i, color, board, knight_boards);
                        for pos in pos_moves {
                            moves.push(pos)
                        }
                    }
                }
            }
        },
        _ => {}
    }

    println!("Total: {} ns", now.elapsed().as_nanos());
    println!("Total moves: {}", moves.len());

    moves
}

#[derive(Debug)]
pub struct Move{
    color: Color,
    kind: Kind,

    from: u8,
    to: u8,

    // captured: Kind,
    // capture: bool,
    // en_passant: u8,
    // en_passant_capture: u8,
    //
    // promotion: bool,
    // promotion_to: Kind,
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

fn pawn_moves(position: u8, color: u8, mut boards: &mut BitPos) -> Vec<Move> {
    //     -16*
    // -9* -8 -7*
    //      X
    //  7*  8   9*
    //      16*

    let mut pos_moves: Vec<Move> = Vec::new();
    if boards.pinned.get_bit(position) {
        return pos_moves
    }

    let side_pawn_left: bool = Vec::from([0u8, 8u8, 16u8, 24u8, 32u8, 40u8, 48u8, 56u8]).contains(&position);
    let side_pawn_right: bool = Vec::from([7u8, 15u8, 23u8, 31u8, 39u8, 47u8, 55u8, 63u8]).contains(&position);

    // generate moves
    match color {
        0 => {
            if !boards.black.get_bit(position - 8) {
                pos_moves.push(Move::new(Color::White, Kind::Pawn, position, position - 8));
                if (48..56).contains(&position) {
                    if !boards.black.get_bit(position - 16) {
                        pos_moves.push(Move::new(Color::White, Kind::Pawn, position, position - 16));
                    }
                }
            }
            if !side_pawn_left {
                if boards.black.get_bit(position - 9) {
                    pos_moves.push(Move::new(Color::White, Kind::Pawn, position, position - 9));
                }
                boards.attack_white.set_bit(position - 9);
            }
            if !side_pawn_right {
                if boards.black.get_bit(position - 7) {
                    pos_moves.push(Move::new(Color::White, Kind::Pawn, position, position - 7));
                }
                boards.attack_white.set_bit(position - 7);
            }
        },
        1 => {
            if !boards.white.get_bit(position + 8) {
                pos_moves.push(Move::new(Color::Black, Kind::Pawn, position, position + 8));
                if (8..16).contains(&position) {
                    if !boards.white.get_bit(position + 16) {
                        pos_moves.push(Move::new(Color::Black, Kind::Pawn, position, position + 16));
                    }
                }
            }
            if boards.white.get_bit(position + 9) {
                pos_moves.push(Move::new(Color::White, Kind::Pawn, position, position + 9));
                boards.attack_black.set_bit(position + 16);
            }
            if boards.white.get_bit(position + 7) {
                pos_moves.push(Move::new(Color::White, Kind::Pawn, position, position + 7));
                boards.attack_black.set_bit(position + 16);
            }
        },
        _ => {}
    }

    pos_moves
}

fn knight_moves(position: u8, color: u8, boards: &mut BitPos, knight_boards: [BitBoard; 64]) -> Vec<Move> {
    let mut pos_moves: Vec<Move> = Vec::new();

    if boards.pinned.get_bit(position) {
        return pos_moves
    }

    // generate moves
    let cur_bit_board = knight_boards[position as usize];

    match color {
        0 => {
            for field in 0u8..64u8 {
                if cur_bit_board.get_bit(field) {
                    if !boards.white.get_bit(field) {
                        pos_moves.push(Move::new(Color::White, Kind::Knight, position, field));
                        boards.attack_white.set_bit(field);
                    }
                }
            }
        },
        1 => {
            for field in 0u8..64u8 {
                if cur_bit_board.get_bit(field) {
                    if !boards.black.get_bit(field) {
                        pos_moves.push(Move::new(Color::Black, Kind::Knight, position, field));
                        boards.attack_black.set_bit(field);
                    }
                }
            }
        },
        _ => {}
    }

    pos_moves
}

fn rook_moves(position: u8, color: u8, boards: &mut BitPos) -> Vec<Move> {
    let pos_moves: Vec<Move> = Vec::new();

    // generate moves



    pos_moves
}