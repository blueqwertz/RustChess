use std::iter::Product;
use crate::engine::bitboard::{BitBoard, BitPos, Color, Kind, Square};
use std::time::Instant;
use crate::engine::game::PrecomputedBitBoards;

pub fn movegen(board: &mut BitPos, color: u8, precomputed: &PrecomputedBitBoards) -> Vec<Move> {
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
                        let pos_moves = knight_moves(i, color,  board, precomputed.knight_boards);
                        for field in 0u8..64u8 {
                            if pos_moves.get_bit(field) {
                                moves.push(Move::new(Color::White, Kind::Knight, i, field))
                            }
                        }
                    } else if board.wr.get_bit(i) {
                        let pos_moves = sliding_pieces(i, color,  board, precomputed.rook_directions);
                        for field in 0u8..64u8 {
                            if pos_moves.get_bit(field) {
                                moves.push(Move::new(Color::White, Kind::Rook, i, field))
                            }
                        }
                    } else if board.wb.get_bit(i) {
                        let pos_moves = sliding_pieces(i, color,  board, precomputed.bishop_directions);
                        for field in 0u8..64u8 {
                            if pos_moves.get_bit(field) {
                                moves.push(Move::new(Color::White, Kind::Rook, i, field))
                            }
                        }
                    } else if board.wq.get_bit(i) {
                        let pos_moves = queen_moves(i, color,  board, precomputed.rook_directions,precomputed.bishop_directions);
                        for field in 0u8..64u8 {
                            if pos_moves.get_bit(field) {
                                moves.push(Move::new(Color::White, Kind::Rook, i, field))
                            }
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
                        let pos_moves = knight_moves(i, color, board, precomputed.knight_boards);
                        for field in 0u8..64u8 {
                            if pos_moves.get_bit(field) {
                                moves.push(Move::new(Color::Black, Kind::Knight, i, field))
                            }
                        }
                    } else if board.br.get_bit(i) {
                        let pos_moves = sliding_pieces(i, color,  board, precomputed.rook_directions);
                        for field in 0u8..64u8 {
                            if pos_moves.get_bit(field) {
                                moves.push(Move::new(Color::Black, Kind::Rook, i, field))
                            }
                        }
                    } else if board.bb.get_bit(i) {
                        let pos_moves = sliding_pieces(i, color,  board, precomputed.bishop_directions);
                        for field in 0u8..64u8 {
                            if pos_moves.get_bit(field) {
                                moves.push(Move::new(Color::Black, Kind::Rook, i, field))
                            }
                        }
                    } else if board.bq.get_bit(i) {
                        let pos_moves = queen_moves(i, color,  board, precomputed.rook_directions,precomputed.bishop_directions);
                        for field in 0u8..64u8 {
                            if pos_moves.get_bit(field) {
                                moves.push(Move::new(Color::Black, Kind::Rook, i, field))
                            }
                        }
                    }
                }
            }
        },
        _ => {}
    }

    println!("Total: {} ns", now.elapsed().as_nanos());
    // println!("Total moves: {}", moves.len());

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
        // println!("{color:?}, {kind:?}, {from:?} -> {to:?}");
        println!("{from:?} -> {to:?}");
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
            if !boards.all.get_bit(position - 8) {
                pos_moves.push(Move::new(Color::White, Kind::Pawn, position, position - 8));
                if (48..56).contains(&position) {
                    if !boards.all.get_bit(position - 16) {
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
            if !boards.all.get_bit(position + 8) {
                pos_moves.push(Move::new(Color::Black, Kind::Pawn, position, position + 8));
                if (8..16).contains(&position) {
                    if !boards.all.get_bit(position + 16) {
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

fn knight_moves(position: u8, color: u8, boards: &mut BitPos, knight_boards: [BitBoard; 64]) -> BitBoard {

    if boards.pinned.get_bit(position) {
        return BitBoard::empty();
    }

    // generate moves
    let cur_bit_board = knight_boards[position as usize];

    match color {
        0 => {
            BitBoard::from(cur_bit_board.0 & (!boards.white.0))
        },
        1 => {
            BitBoard::from(cur_bit_board.0 & (!boards.black.0))
        },
        _ => {
            BitBoard::empty()
        }
    }
}

fn sliding_pieces(position: u8, color: u8, boards: &mut BitPos, given_rays: [[BitBoard; 4]; 64]) -> BitBoard {

    // generate moves

    let mut blockers = boards.all.0;

    let mut rays = given_rays[position as usize];
    let mut attack_board = BitBoard::empty();


    match color {
        0 => {
            for direction in 0..4 {
                let masked_blockers = BitBoard::from(rays[direction].0 & blockers);

                for sq in 0u8..64u8 {
                    if masked_blockers.get_bit(sq) {
                        rays[direction].0 &= rays[direction].0 & (!given_rays[sq as usize][direction].0);
                    }
                }
                attack_board.0 |= rays[direction].0;
            }

            attack_board.0 &= !boards.white.0;
            boards.attack_white.0 |= attack_board.0;
        },
        1 => {
            for direction in 0..4 {
                let masked_blockers = BitBoard::from(rays[direction].0 & blockers);

                for sq in 0u8..64u8 {
                    if masked_blockers.get_bit(sq) {
                        rays[direction].0 &= rays[direction].0 & (!given_rays[sq as usize][direction].0);
                    }
                }
                attack_board.0 |= rays[direction].0;
            }
            attack_board.0 &= !boards.black.0;
            boards.attack_black.0 |= attack_board.0;
        },
        _ => {}
    }

    attack_board
}

fn queen_moves (position: u8, color: u8, boards: &mut BitPos, rook_rays: [[BitBoard; 4]; 64], bishop_rays: [[BitBoard; 4]; 64]) -> BitBoard {
    let rook_type_moves: BitBoard = sliding_pieces(position, color, boards, rook_rays);
    let bishop_type_moves: BitBoard = sliding_pieces(position, color, boards, bishop_rays);

    BitBoard::from(rook_type_moves.0 | bishop_type_moves.0)

}