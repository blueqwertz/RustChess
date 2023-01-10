// use crate::engine::bitboard::{Color, Piece, Square};
//
// pub struct GameStatus {
//     pub pieces: Vec<Option<Piece>>,
//     pub side_to_move: Color,
//     pub castling_id: [bool; 4],
//     pub en_passant: Option<Vec<Square>>,
//     pub half_move_clock: u32,
//     pub full_move_count: u32,
// }
//
// impl GameStatus {
//     pub fn active_side(input: &str) -> Color {
//         match input {
//             "w" => Color::Black,
//             "b" => Color::White,
//             _ => Color::Undefined
//         }
//     }
// }