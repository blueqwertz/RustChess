use crate::engine::bitboard::{BitBoard, BitPos, Color, Kind, Square};

pub fn movegen(board: BitPos, color: u8) {
    let mut moves = Vec::new();

    match color {
        0 => {
            for i in 0u8..64u8 {
                if board.white.get_bit(i) {
                    if board.wp.get_bit(i) {
                        let pos_moves = pawn_moves(i, color, board);
                        for pos in pos_moves {
                            moves.push(pos)
                        }
                    } else if board.wn.get_bit(i) {
                        let pos_moves = knight_moves(i, color, board);
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
                        let pos_moves = knight_moves(i, color, board);
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

fn knight_moves(position: u8, color: u8, mut boards: BitPos) -> Vec<Move> {
    let mut pos_moves: Vec<Move> = Vec::new();

    if boards.pinned.get_bit(position) {
        return pos_moves
    }

    let knight_boards: [BitBoard; 64] = [
        BitBoard::from(132096),
        BitBoard::from(329728),
        BitBoard::from(659712),
        BitBoard::from(1319424),
        BitBoard::from(2638848),
        BitBoard::from(5277696),
        BitBoard::from(10489856),
        BitBoard::from(4202496),
        BitBoard::from(33816580),
        BitBoard::from(84410376),
        BitBoard::from(168886289),
        BitBoard::from(337772578),
        BitBoard::from(675545156),
        BitBoard::from(1351090312),
        BitBoard::from(2685403152),
        BitBoard::from(1075839008),
        BitBoard::from(8657044482),
        BitBoard::from(21609056261),
        BitBoard::from(43234889994),
        BitBoard::from(86469779988),
        BitBoard::from(172939559976),
        BitBoard::from(345879119952),
        BitBoard::from(687463207072),
        BitBoard::from(275414786112),
        BitBoard::from(2216203387392),
        BitBoard::from(5531918402816),
        BitBoard::from(11068131838464),
        BitBoard::from(22136263676928),
        BitBoard::from(44272527353856),
        BitBoard::from(88545054707712),
        BitBoard::from(175990581010432),
        BitBoard::from(70506185244672),
        BitBoard::from(567348067172352),
        BitBoard::from(1416171111120896),
        BitBoard::from(2833441750646784),
        BitBoard::from(5666883501293568),
        BitBoard::from(11333767002587136),
        BitBoard::from(22667534005174272),
        BitBoard::from(45053588738670592),
        BitBoard::from(18049583422636032),
        BitBoard::from(145241105196122112),
        BitBoard::from(362539804446949376),
        BitBoard::from(725361088165576704),
        BitBoard::from(1450722176331153408),
        BitBoard::from(2901444352662306816),
        BitBoard::from(5802888705324613632),
        BitBoard::from(11533718717099671552),
        BitBoard::from(4620693356194824192),
        BitBoard::from(288234782788157440),
        BitBoard::from(576469569871282176),
        BitBoard::from(1224997833292120064),
        BitBoard::from(2449995666584240128),
        BitBoard::from(4899991333168480256),
        BitBoard::from(9799982666336960512),
        BitBoard::from(1152939783987658752),
        BitBoard::from(2305878468463689728),
        BitBoard::from(1128098930098176),
        BitBoard::from(2257297371824128),
        BitBoard::from(4796069720358912),
        BitBoard::from(9592139440717824),
        BitBoard::from(19184278881435648),
        BitBoard::from(38368557762871296),
        BitBoard::from(4679521487814656),
        BitBoard::from(9077567998918656),
    ];

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

fn rook_moves(position: u8, color: u8, mut boards: BitPos) -> Vec<Move> {
    let pos_moves: Vec<Move> = Vec::new();

    // generate moves



    pos_moves
}