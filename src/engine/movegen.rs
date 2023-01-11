use crate::engine::bitboard::{BitPos, Color, Kind, Square};

pub fn movegen(board: BitPos, color: u8) {
    let mut moves = Vec::new();

    // pawns
    match color {
        0 => {
            for i in 0u8..64u8 {
                if board.white.get_bit(i) {
                    if board.wp.get_bit(i) {
                        let pos_moves = pawn_moves(i, color, board);
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
                    }
                }
            }
        },
        _ => {}
    }

    for pos_move in moves {
        pos_move.print();
    }
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

fn pawn_moves(position: u8, color: u8, mut boards: BitPos) -> Vec<Move> {
    //     -16*
    // -9* -8 -7*
    //      X
    //  7*  8   9*
    //      16*

    let mut pos_moves: Vec<Move> = Vec::new();
    if boards.pinned.get_bit(position) {
        return pos_moves
    }
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
            if boards.black.get_bit(position - 9) {
                pos_moves.push(Move::new(Color::White, Kind::Pawn, position, position - 9));
                boards.attack_white.set_bit(position - 9);
            }
            if boards.black.get_bit(position - 7) {
                pos_moves.push(Move::new(Color::White, Kind::Pawn, position, position - 7));
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

fn knight_moves(position: u8, color: u8, kind: u8, mut boards: BitPos) -> Vec<Move> {
    let pos_moves: Vec<Move> = Vec::new();

    // generate moves


    pos_moves
}

fn rook_moves(position: u8, color: u8, kind: u8, mut boards: BitPos) -> Vec<Move> {
    let pos_moves: Vec<Move> = Vec::new();

    // generate moves



    pos_moves
}