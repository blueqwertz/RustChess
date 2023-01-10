#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BitBoard(pub u64);

impl BitBoard {

    pub fn empty() -> Self {
        Self(0)
    }

    pub fn from(num: u64) -> Self {
        Self(num)
    }

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

    pub fn from_sq(square: u8) -> Self {
        Self(1 << square)
    }

    pub fn set_bit(&mut self, square: u8) {
        self.0 |= BitBoard::from_sq(square).0;
    }

    pub fn unset_bit(&mut self, square: u8) {
        self.0 != BitBoard::from_sq(square).0;
    }

    pub fn toggle_bit(&mut self, square: u8) {
        self.0 ^= BitBoard::from_sq(square).0;
    }

    pub fn get_bit(&self, square: u8) -> bool {
        (self.0 & (1 << square)) != 0
    }

}

pub struct BitPos {
    pub all: BitBoard,
    pub white: BitBoard,
    pub black: BitBoard,
    pub attack_white: BitBoard,
    pub attack_black: BitBoard,
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
            attack_white: BitBoard::empty(),
            attack_black: BitBoard::empty(),
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

    pub fn bit_move(&mut self, color: Color, kind: u8, from: u8, to: u8) {
        &self.all.unset_bit(from);
        &self.all.set_bit(to);
        match color {
            Color::White => {
                &self.white.unset_bit(from);
                &self.white.set_bit(to);
                match kind {
                    1 => {
                        &self.wk.unset_bit(from);
                        &self.wk.set_bit(to);
                    },
                    2 => {
                        &self.wq.unset_bit(from);
                        &self.wq.set_bit(to);
                    },
                    3 => {
                        &self.wb.unset_bit(from);
                        &self.wb.set_bit(to);
                    },
                    4 => {
                        &self.wn.unset_bit(from);
                        &self.wn.set_bit(to);
                    },
                    5 => {
                        &self.wr.unset_bit(from);
                        &self.wr.set_bit(to);
                    },
                    6 => {
                        &self.wp.unset_bit(from);
                        &self.wp.set_bit(to);
                    },
                    _ => {}
                }
            },
            Color::Black => {
                &self.black.unset_bit(from);
                &self.black.set_bit(to);
                match kind {
                        1 => {
                    &self.bk.unset_bit(from);
                    &self.bk.set_bit(to);
                    },
                        2 => {
                    &self.bq.unset_bit(from);
                    &self.bq.set_bit(to);
                    },
                        3 => {
                    &self.bb.unset_bit(from);
                    &self.bb.set_bit(to);
                    },
                        4 => {
                    &self.bn.unset_bit(from);
                    &self.bn.set_bit(to);
                    },
                        5 => {
                    &self.br.unset_bit(from);
                    &self.br.set_bit(to);
                    },
                        6 => {
                    &self.bp.unset_bit(from);
                    &self.bp.set_bit(to);
                    },
                    _ => {}
                }
            }
            _ => {}
        }
    }

    pub fn print(&self) {
        let mut board: Vec<char> = Vec::new();
        println!("Value: {}", &self.all.0);
        println!();
        let mut x = 8;
        for rank in 0..8 {
            print!("\x1b[34m{}\x1b[0m  ", x);
            x -= 1;
            for file in 0..8 {
                let square = rank * 8 + file;
                if &self.all.0 & (1 << square) != 0 {
                    if &self.wp.0 & (1 << square) != 0 {
                        print!("\x1b[1mp\x1b[0m ")
                    }
                    else if &self.wn.0 & (1 << square) != 0 {
                        print!("\x1b[1mn\x1b[0m ")
                    }
                    else if &self.wb.0 & (1 << square) != 0 {
                        print!("\x1b[1mb\x1b[0m ")
                    }
                    else if &self.wr.0 & (1 << square) != 0 {
                        print!("\x1b[1mr\x1b[0m ")
                    }
                    else if &self.wq.0 & (1 << square) != 0 {
                        print!("\x1b[1mq\x1b[0m ")
                    }
                    else if &self.wk.0 & (1 << square) != 0 {
                        print!("\x1b[1mk\x1b[0m ")
                    }
                    else if &self.bp.0 & (1 << square) != 0 {
                        print!("\x1b[1mP\x1b[0m ")
                    }
                    else if &self.bn.0 & (1 << square) != 0 {
                        print!("\x1b[1mN\x1b[0m ")
                    }
                    else if &self.bb.0 & (1 << square) != 0 {
                        print!("\x1b[1mB\x1b[0m ")
                    }
                    else if &self.br.0 & (1 << square) != 0 {
                        print!("\x1b[1mR\x1b[0m ")
                    }
                    else if &self.bq.0 & (1 << square) != 0 {
                        print!("\x1b[1mQ\x1b[0m ")
                    }
                    else if &self.bk.0 & (1 << square) != 0 {
                        print!("\x1b[1mK\x1b[0m ")
                    } else {
                        print!("\x1b[1m1\x1b[0m ")
                    }
                }
                else {
                    print!("\x1b[38;5;8m0\x1b[0m ");
                }
            }
            println!();
        }
        println!("\x1b[34m   a b c d e f g h\x1b[0m");
        println!();
    }

    pub fn from_fen(fen: &str) -> BitPos {
        let mut bitpos = BitPos::empty();
        let mut position:u8 = 0;
        for p in fen.chars() {
            // println!("{p} {position}");
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
                '1' => {},
                '2' => position += 1,
                '3' => position += 2,
                '4' => position += 3,
                '5' => position += 4,
                '6' => position += 5,
                '7' => position += 6,
                '8' => position += 7,
                _ => continue
            }
            if position >= 63 {
                break
            }
            position += 1
        }
        bitpos.white.0 = bitpos.wp.0 | bitpos.wn.0 | bitpos.wb.0 | bitpos.wr.0 | bitpos.wq.0 | bitpos.wk.0;
        bitpos.black.0 = bitpos.bp.0 | bitpos.bn.0 | bitpos.bb.0 | bitpos.br.0 | bitpos.bq.0 | bitpos.bk.0;
        bitpos.all.0 = bitpos.white.0 | bitpos.black.0;
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
    A1, B1, C1, D1, E1, F1, G1, H1, Undefined
}

impl From<u8> for Square {
    fn from(index: u8) -> Self {
        match index {
            0 => Square::A1,
            1 => Square::B1,
            2 => Square::C1,
            3 => Square::D1,
            4 => Square::E1,
            5 => Square::F1,
            6 => Square::G1,
            7 => Square::H1,
            8 => Square::A2,
            9 => Square::B2,
            10 => Square::C2,
            11 => Square::D2,
            12 => Square::E2,
            13 => Square::F2,
            14 => Square::G2,
            15 => Square::H2,
            16 => Square::A3,
            17 => Square::B3,
            18 => Square::C3,
            19 => Square::D3,
            20 => Square::E3,
            21 => Square::F3,
            22 => Square::G3,
            23 => Square::H3,
            24 => Square::A4,
            25 => Square::B4,
            26 => Square::C4,
            27 => Square::D4,
            28 => Square::E4,
            29 => Square::F4,
            30 => Square::G4,
            31 => Square::H4,
            32 => Square::A5,
            33 => Square::B5,
            34 => Square::C5,
            35 => Square::D5,
            36 => Square::E5,
            37 => Square::F5,
            38 => Square::G5,
            39 => Square::H5,
            40 => Square::A6,
            41 => Square::B6,
            42 => Square::C6,
            43 => Square::D6,
            44 => Square::E6,
            45 => Square::F6,
            46 => Square::G6,
            47 => Square::H6,
            48 => Square::A7,
            49 => Square::B7,
            50 => Square::C7,
            51 => Square::D7,
            52 => Square::E7,
            53 => Square::F7,
            54 => Square::G7,
            55 => Square::H7,
            56 => Square::A8,
            57 => Square::B8,
            58 => Square::C8,
            59 => Square::D8,
            60 => Square::E8,
            61 => Square::F8,
            62 => Square::G8,
            63 => Square::H8,
            _ => Square::Undefined
        }
    }
}

#[derive(Debug)]
pub enum Kind {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
    Undefined,
}

#[derive(Debug)]
pub enum Color {
    White,
    Black,
    Undefined
}
