use crate::engine::bitboard::Color::Undefined;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BitBoard(pub u64);

impl BitBoard {

    pub fn print(&self) {
        println!("Value: {}", &self.0);
        println!();
        let mut x = 8;
        for rank in 0..8 {
            print!("\x1b[34m{}\x1b[0m  ", x);
            x -= 1;
            for file in 0..8 {
                let square = rank * 8 + file;
                if self.0 & (1 << square) >= 1 {
                    print!("\x1b[1m1\x1b[0m ");
                } else {
                    print!("\x1b[38;5;8m0\x1b[0m ");
                }
            }
            println!();
        }
        println!("\x1b[34m   a b c d e f g h\x1b[0m");
        println!();
    }

    pub fn empty() -> Self {
        Self(0)
    }

    pub fn from_sq(square: u8) -> Self {
        Self(1 << square)
    }

    pub fn set_bit(&mut self, square: u8) {
        self.0 |= BitBoard::from_sq(square).0;
    }

    pub fn toggle_bit(&mut self, square: u8) {
        self.0 ^= BitBoard::from_sq(square).0;
    }

}

pub struct BitPos {
    pub all: BitBoard,
    pub white: BitBoard,
    pub black: BitBoard,
    pub wp: BitBoard, // white pawns
    pub wn: BitBoard, // white knights
    pub wb: BitBoard, // white bishops
    pub wr: BitBoard, // white rooks
    pub wq: BitBoard, // white queen
    pub wk: BitBoard, // white king
    pub bp: BitBoard, // black pawns
    pub bn: BitBoard, // black knights
    pub bb: BitBoard, // black bishops
    pub br: BitBoard, // black rooks
    pub bq: BitBoard, // black queen
    pub bk: BitBoard, // black king
}

impl BitPos {
    pub fn empty () -> Self {
        Self {
            all: BitBoard::empty(),
            white: BitBoard::empty(),
            black: BitBoard::empty(),
            wp: BitBoard::empty(),
            wn: BitBoard::empty(),
            wb: BitBoard::empty(),
            wr: BitBoard::empty(),
            wq: BitBoard::empty(),
            wk: BitBoard::empty(),
            bp: BitBoard::empty(),
            bn: BitBoard::empty(),
            bb: BitBoard::empty(),
            br: BitBoard::empty(),
            bq: BitBoard::empty(),
            bk: BitBoard::empty()
        }
    }

    pub fn from_fen(fen: &str) -> BitPos {
        let mut bitpos = BitPos::empty();
        let mut position:u8 = 0;
        for p in fen.chars() {
            println!("{p} {position}");
            match p {
                'P' => bitpos.wp.set_bit(position),
                'N' => bitpos.wn.set_bit(position),
                'B' => bitpos.wb.set_bit(position),
                'R' => bitpos.wr.set_bit(position),
                'Q' => bitpos.wq.set_bit(position),
                'K' => bitpos.wk.set_bit(position),
                'p' => bitpos.bp.set_bit(position),
                'n' => bitpos.bn.set_bit(position),
                'b' => bitpos.bb.set_bit(position),
                'r' => bitpos.br.set_bit(position),
                'q' => bitpos.bq.set_bit(position),
                'k' => bitpos.bk.set_bit(position),
                '/' => position = (position + 8 % 8) - 1,
                '2' => position += 1,
                '3' => position += 2,
                '4' => position += 3,
                '5' => position += 4,
                '6' => position += 5,
                '7' => position += 6,
                '8' => {position += 7; println!("test")},
                _ => continue
            }
            if position >= 63 {
                break
            }
            position += 1
        }
        bitpos
    }
}

#[rustfmt::skip]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Square {
    A8, B8, C8, D8, E8, F8, G8, H8,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A1, B1, C1, D1, E1, F1, G1, H1,
}

// pub struct Piece {
//     pub kind: Kind,
//     pub color: Color,
//     pub symbol: char,
// }

// impl Piece {
//     pub fn gen(piece_char: char) -> Self {
//         let (color, kind, symbol) = match piece_char {
//             'P' => (Color::White, Kind::Pawn, 'P'),
//             'N' => (Color::White, Kind::Knight, 'N'),
//             'B' => (Color::White, Kind::Bishop, 'B'),
//             'R' => (Color::White, Kind::Rook, 'R'),
//             'Q' => (Color::White, Kind::Queen, 'Q'),
//             'K' => (Color::White, Kind::King, 'K'),
//             'p' => (Color::Black, Kind::Pawn, 'p'),
//             'n' => (Color::Black, Kind::Knight, 'n'),
//             'b' => (Color::Black, Kind::Bishop, 'b'),
//             'r' => (Color::Black, Kind::Rook, 'r'),
//             'q' => (Color::Black, Kind::Queen, 'q'),
//             'k' => (Color::Black, Kind::King, 'k'),
//             _ => (Color::Undefined, Kind::Undefined, '-')
//         };
//         Self {
//             color,
//             kind,
//             symbol
//         }
//     }
// }

pub enum Kind {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
    Undefined,
}

pub enum Color {
    White,
    Black,
    Undefined
}
