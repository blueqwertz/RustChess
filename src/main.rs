pub mod engine;
use crate::engine::game::Game;
use std::io;
use std::time::Instant;

fn main() {
	let mut game = Game::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

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
					println!("\x1b[1m{}\x1b[0m nodes in \x1b[1m{}\x1b[0m microseconds", move_count, now.elapsed().as_micros());

				},
				Err(_) => {
					println!("The input after 'perft ' is not a valid number");
				},
			}
		}

		else {
			println!("command '{}' not found", input);
		}
	}
}
