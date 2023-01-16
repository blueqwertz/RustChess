use std::iter::Product;
use crate::engine::bitboard::{BitBoard, BitPos, Color, Kind, Square};
use std::time::Instant;
use crate::engine::bitboard::Kind::Undefined;
use crate::engine::game::PrecomputedBitBoards;

pub fn movegen(board: &mut BitPos, color: u8, precomputed: &PrecomputedBitBoards) -> Vec<Move> {
    let now = Instant::now();

    let mut moves = Vec::new();

    match color {
        0 => {
            for i in (board.white.0.trailing_zeros() as u8)..(64 - board.white.0.leading_zeros() as u8) {
                if board.white.get_bit(i) {
                    if board.wp.get_bit(i) {
                        let pos_moves = pawn_moves(i, color,  board);
                        for pos in pos_moves {
                            moves.push(pos)
                        }
                    }
                        // add trailing zeros
                    else if board.wn.get_bit(i) {
                        let pos_moves = knight_moves(i, color,  board, precomputed.knight_boards);
                        for field in 0u8..64u8 {
                            if pos_moves.get_bit(field) {
                                let (mut captured, mut capture, mut en_passant, mut en_passant_capture, mut promotion, mut promotion_to) = (Kind::Undefined, false, 0, 0, false, Kind::Undefined);
                                if board.black.get_bit(field) {
                                    capture = true;
                                    captured = board.get_piece_type_at(field);
                                }
                                moves.push(Move::new(Color::White, Kind::Knight, i, field, captured, capture, en_passant, en_passant_capture, promotion, promotion_to));
                            }
                        }
                    }
                    else if board.wr.get_bit(i) {
                        let pos_moves = rook_moves(i, color,  board, precomputed.rook_directions, precomputed.king_dir_mask);
                        for field in 0u8..64u8 {
                            if pos_moves.get_bit(field) {
                                let (mut captured, mut capture, mut en_passant, mut en_passant_capture, mut promotion, mut promotion_to) = (Kind::Undefined, false, 0, 0, false, Kind::Undefined);
                                if board.black.get_bit(field) {
                                    capture = true;
                                    captured = board.get_piece_type_at(field);
                                }
                                moves.push(Move::new(Color::White, Kind::Rook, i, field, captured, capture, en_passant, en_passant_capture, promotion, promotion_to));
                            }
                        }
                    }
                    else if board.wb.get_bit(i) {
                        let pos_moves = bishop_moves(i, color,  board, precomputed.bishop_directions);
                        for field in 0u8..64u8 {
                            if pos_moves.get_bit(field) {
                                let (mut captured, mut capture, mut en_passant, mut en_passant_capture, mut promotion, mut promotion_to) = (Kind::Undefined, false, 0, 0, false, Kind::Undefined);
                                if board.black.get_bit(field) {
                                    capture = true;
                                    captured = board.get_piece_type_at(field);
                                }
                                moves.push(Move::new(Color::White, Kind::Rook, i, field, captured, capture, en_passant, en_passant_capture, promotion, promotion_to));
                            }
                        }
                    }
                    else if board.wq.get_bit(i) {
                        let pos_moves = queen_moves(i, color,  board, precomputed.rook_directions,precomputed.bishop_directions, precomputed.king_dir_mask);
                        for field in 0u8..64u8 {
                            if pos_moves.get_bit(field) {
                                let (mut captured, mut capture, mut en_passant, mut en_passant_capture, mut promotion, mut promotion_to) = (Kind::Undefined, false, 0, 0, false, Kind::Undefined);
                                if board.black.get_bit(field) {
                                    capture = true;
                                    captured = board.get_piece_type_at(field);
                                }
                                moves.push(Move::new(Color::White, Kind::Queen, i, field, captured, capture, en_passant, en_passant_capture, promotion, promotion_to));
                            }
                        }
                    }
                }
            }
        },
        1 => {
            for i in (board.black.0.trailing_zeros() as u8)..(64 - board.black.0.leading_zeros() as u8) {
                if board.black.get_bit(i) {
                    if board.bp.get_bit(i) {
                        let pos_moves = pawn_moves(i, color, board);
                        for pos in pos_moves {
                            moves.push(pos)
                        }
                    }
                    else if board.bn.get_bit(i) {
                        let pos_moves = knight_moves(i, color, board, precomputed.knight_boards);
                        for field in 0u8..64u8 {
                            if pos_moves.get_bit(field) {
                                let (mut captured, mut capture, mut en_passant, mut en_passant_capture, mut promotion, mut promotion_to) = (Kind::Undefined, false, 0, 0, false, Kind::Undefined);
                                if board.white.get_bit(field) {
                                    capture = true;
                                    captured = board.get_piece_type_at(field);
                                }
                                moves.push(Move::new(Color::Black, Kind::Knight, i, field, captured, capture, en_passant, en_passant_capture, promotion, promotion_to));
                            }
                        }
                    }
                    else if board.br.get_bit(i) {
                        let pos_moves = rook_moves(i, color,  board, precomputed.rook_directions, precomputed.king_dir_mask);
                        for field in 0u8..64u8 {
                            if pos_moves.get_bit(field) {
                                let (mut captured, mut capture, mut en_passant, mut en_passant_capture, mut promotion, mut promotion_to) = (Kind::Undefined, false, 0, 0, false, Kind::Undefined);
                                if board.white.get_bit(field) {
                                    capture = true;
                                    captured = board.get_piece_type_at(field);
                                }
                                moves.push(Move::new(Color::Black, Kind::Rook, i, field, captured, capture, en_passant, en_passant_capture, promotion, promotion_to));
                            }
                        }
                    }
                    else if board.bb.get_bit(i) {
                        let pos_moves = bishop_moves(i, color,  board, precomputed.bishop_directions);
                        for field in 0u8..64u8 {
                            if pos_moves.get_bit(field) {
                                let (mut captured, mut capture, mut en_passant, mut en_passant_capture, mut promotion, mut promotion_to) = (Kind::Undefined, false, 0, 0, false, Kind::Undefined);
                                if board.white.get_bit(field) {
                                    capture = true;
                                    captured = board.get_piece_type_at(field);
                                }
                                moves.push(Move::new(Color::Black, Kind::Bishop, i, field, captured, capture, en_passant, en_passant_capture, promotion, promotion_to));
                            }
                        }
                    }
                    else if board.bq.get_bit(i) {
                        let pos_moves = queen_moves(i, color,  board, precomputed.rook_directions,precomputed.bishop_directions, precomputed.king_dir_mask);
                        for field in 0u8..64u8 {
                            if pos_moves.get_bit(field) {
                                let (mut captured, mut capture, mut en_passant, mut en_passant_capture, mut promotion, mut promotion_to) = (Kind::Undefined, false, 0, 0, false, Kind::Undefined);
                                if board.white.get_bit(field) {
                                    capture = true;
                                    captured = board.get_piece_type_at(field);
                                }
                                moves.push(Move::new(Color::Black, Kind::Queen, i, field, captured, capture, en_passant, en_passant_capture, promotion, promotion_to));
                            }
                        }
                    }
                }
            }
        },
        _ => {}
    }

    let time_elapsed = now.elapsed().as_nanos();

    println!("Total: {} ns", time_elapsed);
    println!("Total moves: {}", moves.len());

    moves
}

#[derive(Debug, Copy, Clone)]
pub struct Move{
    pub color: Color,
    pub kind: Kind,

    pub from: u8,
    pub to: u8,

    pub captured: Kind,
    pub capture: bool,
    pub en_passant: u8,
    pub en_passant_capture: u8,

    pub promotion: bool,
    pub promotion_to: Kind,
}

impl Move {
    pub fn new(color: Color, kind: Kind, from: u8, to: u8, captured: Kind, capture: bool, en_passant: u8, en_passant_capture: u8, promotion: bool, promotion_to: Kind) -> Self {
        Self{color, kind, from, to, captured, capture, en_passant, en_passant_capture, promotion, promotion_to}
    }

    pub fn print(&self) {
        let color = &self.color;
        let kind = &self.kind;
        let from = Square::from(*&self.from);
        let to = Square::from(*&self.to);
        let capture = &self.capture;
        let captured = &self.captured;
        // println!("{color:?}, {kind:?}, {from:?} -> {to:?}");
        println!("{from:?}{to:?}: {color:?}, {kind:?}, {capture:?}, {captured:?}");
    }
}

fn pawn_moves(position: u8, color: u8, mut boards: &mut BitPos) -> Vec<Move> {
    //     -16*
    // -9* -8 -7*
    //      X
    //  7*  8   9*
    //      16*

    let mut pos_moves: Vec<Move> = Vec::new();


    // generate moves
    match color {
        0 => {
            if !boards.pinned[0].get_bit(position) {
                if !boards.all.get_bit(position - 8) {
                    let (mut captured, mut capture, mut en_passant, mut en_passant_capture, mut promotion, mut promotion_to) = (Kind::Undefined, false, 0, 0, false, Kind::Undefined);
                    pos_moves.push(Move::new(Color::White, Kind::Pawn, position, position - 8, captured, capture, en_passant, en_passant_capture, promotion, promotion_to));
                    if (48..56).contains(&position) {
                        if !boards.all.get_bit(position - 16) {
                            let (mut captured, mut capture, mut en_passant, mut en_passant_capture, mut promotion, mut promotion_to) = (Kind::Undefined, false, 0, 0, false, Kind::Undefined);
                            pos_moves.push(Move::new(Color::White, Kind::Pawn, position, position - 16, captured, capture, en_passant, en_passant_capture, promotion, promotion_to));
                        }
                    }
                }
            }

            let side_pawn_left: bool = Vec::from([0u8, 8u8, 16u8, 24u8, 32u8, 40u8, 48u8, 56u8]).contains(&position);
            let side_pawn_right: bool = Vec::from([7u8, 15u8, 23u8, 31u8, 39u8, 47u8, 55u8, 63u8]).contains(&position);

            if !boards.pinned[7].get_bit(position) {
                if !side_pawn_left {
                    if boards.black.get_bit(position - 9) {
                        let (mut captured, mut capture, mut en_passant, mut en_passant_capture, mut promotion, mut promotion_to) = (boards.get_piece_type_at(position - 9), true, 0, 0, false, Kind::Undefined);
                        pos_moves.push(Move::new(Color::White, Kind::Pawn, position, position - 9, captured, capture, en_passant, en_passant_capture, promotion, promotion_to));
                    }
                    boards.attack_white.set_bit(position - 9);
                }
            }

            if !boards.pinned[1].get_bit(position) {
                if !side_pawn_right {
                    if boards.black.get_bit(position - 7) {
                        let (mut captured, mut capture, mut en_passant, mut en_passant_capture, mut promotion, mut promotion_to) = (boards.get_piece_type_at(position - 7), true, 0, 0, false, Kind::Undefined);
                        pos_moves.push(Move::new(Color::Black, Kind::Pawn, position, position - 7, captured, capture, en_passant, en_passant_capture, promotion, promotion_to));
                    }
                    boards.attack_white.set_bit(position - 7);
                }
            }
        },
        1 => {
            if !boards.pinned[4].get_bit(position) {
                if !boards.all.get_bit(position + 8) {
                    let (mut captured, mut capture, mut en_passant, mut en_passant_capture, mut promotion, mut promotion_to) = (Kind::Undefined, false, 0, 0, false, Kind::Undefined);
                    pos_moves.push(Move::new(Color::Black, Kind::Pawn, position, position + 8, captured, capture, en_passant, en_passant_capture, promotion, promotion_to));
                    if (8..16).contains(&position) {
                        if !boards.all.get_bit(position + 16) {
                            let (mut captured, mut capture, mut en_passant, mut en_passant_capture, mut promotion, mut promotion_to) = (Kind::Undefined, false, 0, 0, false, Kind::Undefined);
                            pos_moves.push(Move::new(Color::Black, Kind::Pawn, position, position + 16, captured, capture, en_passant, en_passant_capture, promotion, promotion_to));
                        }
                    }
                }
            }

            let side_pawn_left: bool = Vec::from([0u8, 8u8, 16u8, 24u8, 32u8, 40u8, 48u8, 56u8]).contains(&position);
            let side_pawn_right: bool = Vec::from([7u8, 15u8, 23u8, 31u8, 39u8, 47u8, 55u8, 63u8]).contains(&position);

            if !boards.pinned[3].get_bit(position) {
                if boards.white.get_bit(position + 9) {
                    let (mut captured, mut capture, mut en_passant, mut en_passant_capture, mut promotion, mut promotion_to) = (boards.get_piece_type_at(position + 9), true, 0, 0, false, Kind::Undefined);
                    pos_moves.push(Move::new(Color::Black, Kind::Pawn, position, position + 9, captured, capture, en_passant, en_passant_capture, promotion, promotion_to));
                }
            }

            if boards.pinned[6].get_bit(position) {
                if boards.white.get_bit(position + 7) {
                    let (mut captured, mut capture, mut en_passant, mut en_passant_capture, mut promotion, mut promotion_to) = (boards.get_piece_type_at(position + 7), true, 0, 0, false, Kind::Undefined);
                    pos_moves.push(Move::new(Color::Black, Kind::Pawn, position, position + 7, captured, capture, en_passant, en_passant_capture, promotion, promotion_to));
                }
            }
        },
        _ => {}
    }

    pos_moves
}

fn knight_moves(position: u8, color: u8, boards: &mut BitPos, knight_boards: [BitBoard; 64]) -> BitBoard {

    if BitBoard::from(boards.pinned[0].0 | boards.pinned[1].0 | boards.pinned[2].0 | boards.pinned[3].0 | boards.pinned[4].0 | boards.pinned[5].0 | boards.pinned[6].0 | boards.pinned[7].0).get_bit(position) {
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

fn rook_moves (position: u8, color: u8, boards: &mut BitPos, given_rays: [[BitBoard; 4]; 64], king_masks: [[[BitBoard; 64]; 4]; 64]) -> BitBoard {

    // generate moves

    let mut blockers = boards.all.0;

    let mut rays = given_rays[position as usize];
    let mut attack_board = BitBoard::empty();

    match color {
        0 => {
            for direction in 0..4 {
                if boards.pinned[direction * 2].get_bit(position) {
                    continue
                }
                let masked_blockers = BitBoard::from(rays[direction].0 & blockers);
                if masked_blockers.0 == 0u64 {
                    attack_board.0 |= rays[direction].0;
                    continue
                }
                if masked_blockers.0 & boards.bk.0 != 0u64 {
                    // king in attack line
                    let mut blockers_no_king = masked_blockers.0 & (!boards.bk.0);
                    blockers_no_king &= king_masks[position as usize][direction][boards.bk.0.trailing_zeros() as usize].0;
                    if blockers_no_king.count_ones() == 1 {
                        let index: usize = match direction {
                            0 => 4,
                            2 => 6,
                            4 => 0,
                            6 => 2,
                            _ => direction,
                        };
                        boards.pinned[index].set_bit(blockers_no_king.trailing_zeros() as u8);
                    }
                }
                for sq in (masked_blockers.0.trailing_zeros() as u8)..(64 - masked_blockers.0.leading_zeros() as u8) {
                    if masked_blockers.get_bit(sq) {
                        rays[direction].0 &= rays[direction].0 ^ (given_rays[sq as usize][direction].0);
                    }
                }
                attack_board.0 |= rays[direction].0;
            }

            attack_board.0 &= !boards.white.0;
            boards.attack_white.0 |= attack_board.0;
        },
        1 => {
            for direction in 0usize..4usize {
                if boards.pinned[direction * 2].get_bit(position) {
                    continue
                }
                let masked_blockers = BitBoard::from(rays[direction].0 & blockers);
                if masked_blockers.0 == 0u64 {
                    attack_board.0 |= rays[direction].0;
                    continue
                }
                if masked_blockers.0 & boards.bk.0 != 0u64 {
                    // king in attack line
                    let blockers_no_king = masked_blockers.0 & (!boards.wk.0);

                    if blockers_no_king.count_ones() == 1 {
                        let index: usize = match direction {
                            0 => 4,
                            2 => 6,
                            4 => 0,
                            6 => 2,
                            _ => direction,
                        };
                        boards.pinned[index].set_bit(blockers_no_king.trailing_zeros() as u8);
                    }
                }
                for sq in (masked_blockers.0.trailing_zeros() as u8)..(64 - masked_blockers.0.leading_zeros() as u8) {
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

fn bishop_moves (position: u8, color: u8, boards: &mut BitPos, given_rays: [[BitBoard; 4]; 64]) -> BitBoard {

    // generate moves

    let mut blockers = boards.all.0;

    let mut rays = given_rays[position as usize];
    let mut attack_board = BitBoard::empty();


    match color {
        0 => {
            for direction in 0usize..4usize {
                if boards.pinned[direction * 2 + 1].get_bit(position) {
                    continue
                }
                let masked_blockers = BitBoard::from(rays[direction].0 & blockers);
                if masked_blockers.0 == 0u64 {
                    attack_board.0 |= rays[direction].0;
                    continue
                }
                if masked_blockers.0 & boards.bk.0 != 0u64 {
                    // king in attack line
                    let blockers_no_king = masked_blockers.0 & (!boards.bk.0);
                    if blockers_no_king.count_ones() == 1 {
                        let index: usize = match direction {
                            1 => 5,
                            3 => 7,
                            5 => 1,
                            7 => 3,
                            _ => direction,
                        };
                        boards.pinned[index].set_bit(blockers_no_king.trailing_zeros() as u8);
                    }
                }
                for sq in (masked_blockers.0.trailing_zeros() as u8)..(64 - masked_blockers.0.leading_zeros() as u8) {
                    if masked_blockers.get_bit(sq) {
                        rays[direction].0 &= rays[direction].0 ^ (given_rays[sq as usize][direction].0);
                    }
                }
                attack_board.0 |= rays[direction].0;
            }

            attack_board.0 &= !boards.white.0;
            boards.attack_white.0 |= attack_board.0;
        },
        1 => {
            for direction in 0..4 {
                if boards.pinned[direction * 2 + 1].get_bit(position) {
                    continue
                }
                let masked_blockers = BitBoard::from(rays[direction].0 & blockers);
                if masked_blockers.0 == 0u64 {
                    attack_board.0 |= rays[direction].0;
                    continue
                }
                if masked_blockers.0 & boards.bk.0 != 0u64 {
                    // king in attack line
                    let blockers_no_king = masked_blockers.0 & (!boards.wk.0);
                    if blockers_no_king.count_ones() == 1 {
                        let index: usize = match direction {
                            1 => 5,
                            3 => 7,
                            5 => 1,
                            7 => 3,
                            _ => direction,
                        };
                        boards.pinned[index].set_bit(blockers_no_king.trailing_zeros() as u8);
                    }
                }
                for sq in (masked_blockers.0.trailing_zeros() as u8)..(64 - masked_blockers.0.leading_zeros() as u8) {
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

fn queen_moves (position: u8, color: u8, boards: &mut BitPos, rook_rays: [[BitBoard; 4]; 64], bishop_rays: [[BitBoard; 4]; 64], king_masks: [[[BitBoard; 64]; 4]; 64]) -> BitBoard {
    let rook_type_moves: BitBoard = rook_moves(position, color, boards, rook_rays, king_masks);
    let bishop_type_moves: BitBoard = bishop_moves(position, color, boards, bishop_rays);

    BitBoard::from(rook_type_moves.0 | bishop_type_moves.0)

}