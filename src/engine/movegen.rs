use crate::engine::bitboard::{BitPos, Color, Kind, Square};

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
    // -9* -8 -7*
    //      X
    //  7*  8   9*
    //      16*
    let mut pos_moves: Vec<Move> = Vec::new();

    let (x, y) = (position % 8, ((position / 8) as f32).floor() as u8);

    // generate moves
    match color {
        0 => {
            if !boards.black.get_bit(position - 8) {
                pos_moves.push(Move::new(Color::White, Kind::Pawn, position, position - 8));
                if y == 7 {
                    if !boards.black.get_bit(position - 16) {
                        pos_moves.push(Move::new(Color::White, Kind::Pawn, position, position - 16));
                    }
                }
            }
            if boards.black.get_bit(position - 9) {
                pos_moves.push(Move::new(Color::White, Kind::Pawn, position, position - 9))
            }
            if boards.black.get_bit(position - 7) {
                pos_moves.push(Move::new(Color::White, Kind::Pawn, position, position - 7))
            }
        },
        1 => {
            if !boards.white.get_bit(position + 8) {
                pos_moves.push(Move::new(Color::Black, Kind::Pawn, position, position + 8));
                if y == 7 {
                    if !boards.white.get_bit(position + 16) {
                        pos_moves.push(Move::new(Color::Black, Kind::Pawn, position, position + 16));
                    }
                }
            }
            if boards.white.get_bit(position + 9) {
                pos_moves.push(Move::new(Color::White, Kind::Pawn, position, position + 9))
            }
            if boards.white.get_bit(position + 7) {
                pos_moves.push(Move::new(Color::White, Kind::Pawn, position, position + 7))
            }
        },
        _ => {}
    }

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