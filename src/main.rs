pub mod engine;
use crate::engine::game::Game;
use std::io;
use std::time::Instant;
use crate::engine::movegen::movegen;

fn main() {
	let mut game = Game::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

	println!("RustChess v0.0.1 by tim and marius");

	loop {
		let mut input = String::new();
		io::stdin().read_line(&mut input).unwrap();
		input = input.trim().parse().unwrap();
		if input.starts_with("fen") {
			game = Game::from_fen(input.trim_start_matches("fen "));
		} else if input == "d" {
			game.board.print();
		} else if input.starts_with("perft ") {
			let number_str = input.trim_start_matches("perft ");
			match number_str.parse::<u64>() {
				Ok(number) => {
					let now = Instant::now();

					let move_count = game.perft(number, game.side_to_move, true);
					println!("\x1b[1m{}\x1b[0m nodes in \x1b[1m{}\x1b[0m microseconds, {} NPS", move_count, now.elapsed().as_micros(), (((move_count as f64) / (now.elapsed().as_nanos() as f64)) * 1_000_000_000f64) as u64);
				},
				Err(_) => {
					println!("The input after 'perft ' is not a valid number");
				},
			}
		} else if input == "movegen" {
			let pos_moves = movegen(&mut game.board, game.side_to_move, &game.precomputed);
			for pos_move in pos_moves {
				pos_move.print();
			}
		}
		else if input == "exit" {
			break
		}
		else {
			println!("command '{}' not found", input);
		}
	}
}
