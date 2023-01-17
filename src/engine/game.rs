use std::collections::HashMap;
use std::ops::Div;
use std::path::{Prefix, PrefixComponent};
use crate::engine::bitboard::{BitBoard, BitPos, Color, Square};
use crate::engine::movegen::{movegen};
use crate::engine::moves::Move;
use crate::engine::bitboard::Kind::Undefined;

pub struct PrecomputedBitBoards {
	pub rook_directions: [[BitBoard; 4]; 64],
	pub bishop_directions: [[BitBoard; 4]; 64],
	pub knight_boards: [BitBoard; 64],

	pub king_dir_masks: [[[BitBoard; 64]; 4]; 64],
}

fn on_board (x: i8, y: i8) -> bool {
	x <= 7 && x >= 0 && y <= 7 && y >= 0
}

fn calc_knight_boards () ->  [BitBoard; 64] {
	let mut boards: [BitBoard; 64] = [BitBoard::empty(); 64];

	let direction = [[-2, 1], [-1, 2], [1, 2], [2, 1], [-2, -1], [-1, -2], [2, -1], [1, -2]];

	for file in 0..8 {
		for rank in 0..8 {
			for pos_dir in direction {
				if on_board(rank + pos_dir[0], file + pos_dir[1]) {
					boards[(file * 8 + rank) as usize].set_bit(((file + pos_dir[1]) * 8 + rank + pos_dir[0]) as u8);
				}
			}
		}
	}

	boards
}

fn calc_rook_directions () -> [[BitBoard; 4]; 64] {
	let mut boards: [[BitBoard; 4]; 64] = [[BitBoard::empty(); 4]; 64];

    let direction = [[0, -1], [1, 0], [0, 1], [-1, 0]];

	for file in 0..8 {
		for rank in 0..8 {
			for index in 0usize..4usize {
                let mut dir_board = BitBoard::empty();
                let mut i = 1;
                let dir = direction[index];
                let (mut newX, mut newY) = (rank + dir[0] * i, file + dir[1] * i);
                loop {
					(newX, newY) = (rank + dir[0] * i, file + dir[1] * i);
					if !on_board(newX, newY) {
						break
					}
                    i += 1;
                    dir_board.set_bit((newX + newY * 8) as u8);
                }
                boards[(rank + file * 8) as usize][index].0 |= dir_board.0;
			}
		}
	}

	boards
}

fn calc_bishop_directions () -> [[BitBoard; 4]; 64] {
	let mut boards: [[BitBoard; 4]; 64] = [[BitBoard::empty(); 4]; 64];

	let direction = [[1, -1], [1, 1], [-1, 1], [-1, -1]];

	for file in 0..8 {
		for rank in 0..8 {
			for index in 0usize..4usize {
				let mut dir_board = BitBoard::empty();
				let mut i = 1;
				let dir = direction[index];
				let (mut newX, mut newY) = (rank + dir[0] * i, file + dir[1] * i);
				loop {
					(newX, newY) = (rank + dir[0] * i, file + dir[1] * i);
					if !on_board(newX, newY) {
						break
					}
					i += 1;
					dir_board.set_bit((newX + newY * 8) as u8);
				}
				boards[(rank + file * 8) as usize][index].0 |= dir_board.0;
			}
		}
	}

	boards
}

fn calc_king_dir_masks () -> [[[BitBoard; 64]; 4]; 64] {
	let mut boards: [[[BitBoard; 64]; 4]; 64] = [[[BitBoard::empty(); 64]; 4]; 64];

	let r_direction = [[0, -1], [1, 0], [0, 1], [-1, 0]];

	for file in 0..8 {
		for rank in 0..8 {
			for index in 0usize..4usize {
				let mut dir_board = BitBoard::empty();
				let mut i = 1;
				let dir = r_direction[index];
				let (mut newX, mut newY) = (rank + dir[0] * i, file + dir[1] * i);
				loop {
					(newX, newY) = (rank + dir[0] * i, file + dir[1] * i);
					if !on_board(newX, newY) {
						break
					}
					i += 1;
					dir_board.set_bit((newX + newY * 8) as u8);
					boards[(rank + file * 8) as usize][index][(newX + newY * 8) as usize].0 |= dir_board.0;
				}
			}
		}
	}

	let b_direction = [[1, -1], [1, 1], [-1, 1], [-1, -1]];

	for file in 0..8 {
		for rank in 0..8 {
			for index in 0usize..4usize {
				let mut dir_board = BitBoard::empty();
				let mut i = 1;
				let dir = b_direction[index];
				let (mut newX, mut newY) = (rank + dir[0] * i, file + dir[1] * i);
				loop {
					(newX, newY) = (rank + dir[0] * i, file + dir[1] * i);
					if !on_board(newX, newY) {
						break
					}
					i += 1;
					dir_board.set_bit((newX + newY * 8) as u8);
					boards[(rank + file * 8) as usize][index][(newX + newY * 8) as usize].0 |= dir_board.0;
				}
			}
		}
	}

	boards
}

impl PrecomputedBitBoards {
	fn new() -> Self {
		Self {
			knight_boards: calc_knight_boards(),
			bishop_directions: calc_bishop_directions(),
            rook_directions: calc_rook_directions(),
			king_dir_masks: calc_king_dir_masks(),
		}
	}
}

pub struct Game {
	pub board: BitPos,
	pub side_to_move: bool,
	precomputed: PrecomputedBitBoards,
}

impl Game {
	pub fn new() -> Self {
		Self {
			board: BitPos::empty(),
			side_to_move: true,
			precomputed: PrecomputedBitBoards::new(),
		}
	}

	pub fn from_fen(fen: &str) -> Self {
		Self {
			board: BitPos::from_fen(fen),
			side_to_move: true,
			precomputed: PrecomputedBitBoards::new(),
		}
	}

	pub fn perft(&mut self, depth: u64) -> u64{
		if depth == 0 {
			return 1u64
		}
		let moves: Vec<Move> = movegen(&mut self.board, self.side_to_move, &self.precomputed);
		let mut move_count = 0;
		for pos_move in moves {
			&self.board.make_move(pos_move);
			self.side_to_move = !self.side_to_move;
			move_count += self.perft(depth - 1);
			&self.board.unmake_move(pos_move);
			self.side_to_move = !self.side_to_move;
		}
		move_count
	}

	pub fn start(&mut self) {
// 		let moves: Vec<Move> = movegen(&mut self.board, Color::White as u8, &self.precomputed);
// +		for pos_move in moves {
// 			&self.board.make_move(pos_move);
// 			&self.board.unmake_move(pos_move);
// 		}
//
		println!("Starting");

		movegen(&mut self.board, self.side_to_move, &self.precomputed);

		// let move_count = self.perft(2);
		// println!("{}", move_count);
	}
}