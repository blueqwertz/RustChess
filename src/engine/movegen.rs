use std::iter::Product;
use crate::engine::bitboard::{BitBoard, BitPos, Color, Kind, Square};
use std::time::Instant;
use crate::engine::bitboard::Kind::Undefined;
use crate::engine::game::PrecomputedBitBoards;
use crate::engine::moves::{Move};

pub fn movegen(board: &mut BitPos, color: bool, precomputed: &PrecomputedBitBoards) -> Vec<Move> {
	let now = Instant::now();

	let mut moves = Vec::new();

	println!("Starting movegen...");

	match color {
		true => {
			for i in (board.white.0.trailing_zeros() as u8)..(64 - board.white.0.leading_zeros() as u8) {
				if board.white.get_bit(i) {
					if board.wp.get_bit(i) {
						let pos_moves = pawn_moves(i, color,  board);
						for pos in pos_moves {
							moves.push(pos)
						}
					}
					else if board.wn.get_bit(i) {
						let pos_moves = knight_moves(i, color,  board, precomputed.knight_boards);
						for field in (pos_moves.0.trailing_zeros() as u8)..(64 - pos_moves.0.leading_zeros() as u8) {
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
						let pos_moves = rook_moves(i, color,  board, precomputed.rook_directions, precomputed.king_dir_masks);
						for field in (pos_moves.0.trailing_zeros() as u8)..(64 - pos_moves.0.leading_zeros() as u8) {
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
						let pos_moves = bishop_moves(i, color,  board, precomputed.bishop_directions, precomputed.king_dir_masks);
						for field in (pos_moves.0.trailing_zeros() as u8)..(64 - pos_moves.0.leading_zeros() as u8) {
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
						let pos_moves = queen_moves(i, color,  board, precomputed.rook_directions,precomputed.bishop_directions, precomputed.king_dir_masks);
						for field in (pos_moves.0.trailing_zeros() as u8)..(64 - pos_moves.0.leading_zeros() as u8) {
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
		false => {
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
						for field in (pos_moves.0.trailing_zeros() as u8)..(64 - pos_moves.0.leading_zeros() as u8) {
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
						let pos_moves = rook_moves(i, color,  board, precomputed.rook_directions, precomputed.king_dir_masks);
						for field in (pos_moves.0.trailing_zeros() as u8)..(64 - pos_moves.0.leading_zeros() as u8) {
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
						let pos_moves = bishop_moves(i, color,  board, precomputed.bishop_directions, precomputed.king_dir_masks);
						for field in (pos_moves.0.trailing_zeros() as u8)..(64 - pos_moves.0.leading_zeros() as u8) {
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
						let pos_moves = queen_moves(i, color,  board, precomputed.rook_directions,precomputed.bishop_directions, precomputed.king_dir_masks);
						for field in (pos_moves.0.trailing_zeros() as u8)..(64 - pos_moves.0.leading_zeros() as u8) {
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

	// println!("Total: {} ns", time_elapsed);
	// println!("Total moves: {}", moves.len());

	moves
}

fn pawn_moves(position: u8, color: bool, mut boards: &mut BitPos) -> Vec<Move> {
	//     -16*
	// -9* -8 -7*
	//      X
	//  7*  8   9*
	//      16*

	let mut pos_moves: Vec<Move> = Vec::new();


	// generate moves
	match color {
		true => {
			if boards.can_move_direction(position, 0, Color::White) {
				if !boards.all.get_bit(position - 8) {
					let (mut captured, mut capture, mut en_passant, mut en_passant_capture, mut promotion, mut promotion_to) = (Kind::Undefined, false, 0, 0, false, Kind::Undefined);
					pos_moves.push(Move::new(Color::White, Kind::Pawn, position, position - 8, captured, capture, en_passant, en_passant_capture, promotion, promotion_to));
					if (48..56).contains(&position) {
						if !boards.all.get_bit(position - 16) {
							let (mut captured, mut capture, mut en_passant, mut en_passant_capture, mut promotion, mut promotion_to) = (Kind::Undefined, false, position - 8, 0, false, Kind::Undefined);
							pos_moves.push(Move::new(Color::White, Kind::Pawn, position, position - 16, captured, capture, en_passant, en_passant_capture, promotion, promotion_to));
						}
					}
				}
			}

			let side_pawn_left: bool = Vec::from([0u8, 8u8, 16u8, 24u8, 32u8, 40u8, 48u8, 56u8]).contains(&position);
			let side_pawn_right: bool = Vec::from([7u8, 15u8, 23u8, 31u8, 39u8, 47u8, 55u8, 63u8]).contains(&position);

			if boards.can_move_direction(position, 7, Color::White) {
				if !side_pawn_left {
					if boards.black.get_bit(position - 9) {
						let (mut captured, mut capture, mut en_passant, mut en_passant_capture, mut promotion, mut promotion_to) = (boards.get_piece_type_at(position - 9), true, 0, 0, false, Kind::Undefined);
						pos_moves.push(Move::new(Color::White, Kind::Pawn, position, position - 9, captured, capture, en_passant, en_passant_capture, promotion, promotion_to));
					}
					boards.attack_white.set_bit(position - 9);
				}
			}

			if boards.can_move_direction(position, 1, Color::White) {
				if !side_pawn_right {
					if boards.black.get_bit(position - 7) {
						let (mut captured, mut capture, mut en_passant, mut en_passant_capture, mut promotion, mut promotion_to) = (boards.get_piece_type_at(position - 7), true, 0, 0, false, Kind::Undefined);
						pos_moves.push(Move::new(Color::Black, Kind::Pawn, position, position - 7, captured, capture, en_passant, en_passant_capture, promotion, promotion_to));
					}
					boards.attack_white.set_bit(position - 7);
				}
			}
		},
		false => {
			if boards.can_move_direction(position, 4, Color::Black) {
				if !boards.all.get_bit(position + 8) {
					let (mut captured, mut capture, mut en_passant, mut en_passant_capture, mut promotion, mut promotion_to) = (Kind::Undefined, false, 0, 0, false, Kind::Undefined);
					pos_moves.push(Move::new(Color::Black, Kind::Pawn, position, position + 8, captured, capture, en_passant, en_passant_capture, promotion, promotion_to));
					if (8..16).contains(&position) {
						if !boards.all.get_bit(position + 16) {
							let (mut captured, mut capture, mut en_passant, mut en_passant_capture, mut promotion, mut promotion_to) = (Kind::Undefined, false, position + 8, 0, false, Kind::Undefined);
							pos_moves.push(Move::new(Color::Black, Kind::Pawn, position, position + 16, captured, capture, en_passant, en_passant_capture, promotion, promotion_to));
						}
					}
				}
			}

			let side_pawn_left: bool = Vec::from([0u8, 8u8, 16u8, 24u8, 32u8, 40u8, 48u8, 56u8]).contains(&position);
			let side_pawn_right: bool = Vec::from([7u8, 15u8, 23u8, 31u8, 39u8, 47u8, 55u8, 63u8]).contains(&position);

			if boards.can_move_direction(position, 3, Color::Black) {
				if boards.white.get_bit(position + 9) {
					let (mut captured, mut capture, mut en_passant, mut en_passant_capture, mut promotion, mut promotion_to) = (boards.get_piece_type_at(position + 9), true, 0, 0, false, Kind::Undefined);
					pos_moves.push(Move::new(Color::Black, Kind::Pawn, position, position + 9, captured, capture, en_passant, en_passant_capture, promotion, promotion_to));
				}
			}

			if boards.can_move_direction(position, 5, Color::Black) {
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

fn knight_moves(position: u8, color: bool, boards: &mut BitPos, knight_boards: [BitBoard; 64]) -> BitBoard {

	// generate moves
	let cur_bit_board = knight_boards[position as usize];

	match color {
		true => {
			if BitBoard::from(boards.pinned_white[0].0 | boards.pinned_white[1].0 | boards.pinned_white[2].0 | boards.pinned_white[3].0 | boards.pinned_white[4].0 | boards.pinned_white[5].0 | boards.pinned_white[6].0 | boards.pinned_white[7].0).get_bit(position) {
				return BitBoard::empty();
			}
			BitBoard::from(cur_bit_board.0 & (!boards.white.0))
		},
		false => {
			if BitBoard::from(boards.pinned_black[0].0 | boards.pinned_black[1].0 | boards.pinned_black[2].0 | boards.pinned_black[3].0 | boards.pinned_black[4].0 | boards.pinned_black[5].0 | boards.pinned_black[6].0 | boards.pinned_black[7].0).get_bit(position) {
				return BitBoard::empty();
			}
			BitBoard::from(cur_bit_board.0 & (!boards.black.0))
		},
		_ => {
			BitBoard::empty()
		}
	}
}

fn rook_moves (position: u8, color: bool, boards: &mut BitPos, given_rays: [[BitBoard; 4]; 64], king_masks: [[[BitBoard; 64]; 4]; 64]) -> BitBoard {

	// generate moves

	let mut blockers = boards.all.0;

	let mut rays = given_rays[position as usize];
	let mut attack_board = BitBoard::empty();

	match color {
		true => {
			for direction in 0usize..4 {
				if !boards.can_move_direction(position, direction * 2, Color::White) {
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
						boards.pinned_black[index].set_bit(blockers_no_king.trailing_zeros() as u8);
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
		false => {
			for direction in 0usize..4usize {
				if !boards.can_move_direction(position, direction * 2, Color::Black) {
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
						boards.pinned_white[index].set_bit(blockers_no_king.trailing_zeros() as u8);
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

fn bishop_moves (position: u8, color: bool, boards: &mut BitPos, given_rays: [[BitBoard; 4]; 64], king_masks: [[[BitBoard; 64]; 4]; 64]) -> BitBoard {

	// generate moves

	let mut blockers = boards.all.0;

	let mut rays = given_rays[position as usize];
	let mut attack_board = BitBoard::empty();


	match color {
		true => {
			for direction in 0usize..4usize {
				if !boards.can_move_direction(position, direction * 2, Color::White) {
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
							1 => 5,
							3 => 7,
							5 => 1,
							7 => 3,
							_ => direction,
						};
						boards.pinned_black[index].set_bit(blockers_no_king.trailing_zeros() as u8);
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
		false => {
			for direction in 0..4 {
				if !boards.can_move_direction(position, direction * 2, Color::Black) {
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
						boards.pinned_white[index].set_bit(blockers_no_king.trailing_zeros() as u8);
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

fn queen_moves (position: u8, color: bool, boards: &mut BitPos, rook_rays: [[BitBoard; 4]; 64], bishop_rays: [[BitBoard; 4]; 64], king_masks: [[[BitBoard; 64]; 4]; 64]) -> BitBoard {
	let rook_type_moves: BitBoard = rook_moves(position, color, boards, rook_rays, king_masks);
	let bishop_type_moves: BitBoard = bishop_moves(position, color, boards, bishop_rays, king_masks);

	BitBoard::from(rook_type_moves.0 | bishop_type_moves.0)

}