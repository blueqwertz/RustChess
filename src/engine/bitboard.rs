use crate::engine::movegen::Move;

#[derive(Copy, Clone)]
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
        self.0 |= (1 << square);
    }

    pub fn unset_bit(&mut self, square: u8) {
        self.0 &= !(1 << square);
    }

    pub fn toggle_bit(&mut self, square: u8) {
        self.0 ^= (1 << square);
    }

    pub fn get_bit(&self, square: u8) -> bool {
        (self.0 & (1 << square)) != 0
    }

    pub fn print_index(&self) {
        println!("Value: {}", &self.0);
        println!();
        let mut x = 8;
        for rank in 0..8 {
            print!("\x1b[34m{}\x1b[0m  ", x);
            x -= 1;
            for file in 0..8 {
                let square = rank * 8 + file;
                if self.0 & (1 << square) >= 1 {
                    print!("\x1b[1m{:02}\x1b[0m ", square);
                } else {
                    print!("\x1b[38;5;8m{:02}\x1b[0m ", square);
                }
            }
            println!();
        }
        println!("\x1b[34m    a  b  c  d  e  f  g  h\x1b[0m");
        println!();
    }
}

#[derive(Clone, Copy)]
pub struct BitPos {
    pub all: BitBoard,
    pub white: BitBoard,
    pub black: BitBoard,
    pub attack_white: BitBoard,
    pub attack_black: BitBoard,
    pub pinned: [BitBoard; 8],
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
            pinned: [BitBoard::empty(); 8],
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

    pub fn make_move(&mut self, bit_move: Move) {
        &self.all.unset_bit(bit_move.from);
        &self.all.set_bit(bit_move.to);
        bit_move.print();
        match bit_move.color {
            Color::White => {
                &self.white.unset_bit(bit_move.from);
                &self.white.set_bit(bit_move.to);
                if bit_move.capture {
                    &self.black.unset_bit(bit_move.to);
                    match bit_move.captured {
                        Kind::King => {&self.bk.unset_bit(bit_move.to);}
                        Kind::Queen => {&self.bq.unset_bit(bit_move.to);}
                        Kind::Bishop => {&self.bb.unset_bit(bit_move.to);}
                        Kind::Knight => {&self.bk.unset_bit(bit_move.to);}
                        Kind::Rook => {&self.br.unset_bit(bit_move.to);}
                        Kind::Pawn => {&self.bp.unset_bit(bit_move.to);}
                        _ => {},
                    }
                } else if bit_move.en_passant_capture != 0 {
                    &self.black.unset_bit(bit_move.en_passant_capture);
                }
                match bit_move.kind {
                    Kind::King => {
                        &self.wk.unset_bit(bit_move.from);
                        &self.wk.set_bit(bit_move.to);
                    },
                    Kind::Queen => {
                        &self.wq.unset_bit(bit_move.from);
                        &self.wq.set_bit(bit_move.to);
                    },
                    Kind::Bishop => {
                        &self.wb.unset_bit(bit_move.from);
                        &self.wb.set_bit(bit_move.to);
                    },
                    Kind::Knight => {
                        &self.wn.unset_bit(bit_move.from);
                        &self.wn.set_bit(bit_move.to);
                    },
                    Kind::Rook => {
                        &self.wr.unset_bit(bit_move.from);
                        &self.wr.set_bit(bit_move.to);
                    },
                    Kind::Pawn => {
                        &self.wp.unset_bit(bit_move.from);
                        &self.wp.set_bit(bit_move.to);
                    },
                    _ => {}
                }
            },
            Color::Black => {
                &self.black.unset_bit(bit_move.from);
                &self.black.set_bit(bit_move.to);
                if bit_move.capture {
                    &self.white.unset_bit(bit_move.to);
                    match bit_move.captured {
                        Kind::King => {&self.wk.unset_bit(bit_move.to);}
                        Kind::Queen => {&self.wq.unset_bit(bit_move.to);}
                        Kind::Bishop => {&self.wb.unset_bit(bit_move.to);}
                        Kind::Knight => {&self.wk.unset_bit(bit_move.to);}
                        Kind::Rook => {&self.wr.unset_bit(bit_move.to);}
                        Kind::Pawn => {&self.wp.unset_bit(bit_move.to);}
                        _ => {},
                    }
                } else if bit_move.en_passant_capture != 0 {
                    &self.white.unset_bit(bit_move.en_passant);
                }
                match bit_move.kind {
                    Kind::King => {
                        &self.bk.unset_bit(bit_move.from);
                        &self.bk.set_bit(bit_move.to);
                    },
                    Kind::Queen => {
                        &self.bq.unset_bit(bit_move.from);
                        &self.bq.set_bit(bit_move.to);
                    },
                    Kind::Bishop => {
                        &self.bb.unset_bit(bit_move.from);
                        &self.bb.set_bit(bit_move.to);
                    },
                    Kind::Knight => {
                        &self.bn.unset_bit(bit_move.from);
                        &self.bn.set_bit(bit_move.to);
                    },
                    Kind::Rook => {
                        &self.br.unset_bit(bit_move.from);
                        &self.br.set_bit(bit_move.to);
                    },
                    Kind::Pawn => {
                        &self.bp.unset_bit(bit_move.from);
                        &self.bp.set_bit(bit_move.to);
                    },
                    _ => {}
                }
            }
            _ => {}
        }
    }

    pub fn get_piece_type_at (&self, position: u8) -> Kind {
        if !self.all.get_bit(position) {
            return Kind::Undefined
        }

        if self.white.get_bit(position) {
            if self.wp.get_bit(position) {return Kind::Pawn}
            if self.wn.get_bit(position) {return Kind::Knight}
            if self.wb.get_bit(position) {return Kind::Bishop}
            if self.wr.get_bit(position) {return Kind::Rook}
            if self.wq.get_bit(position) {return Kind::Queen}
            if self.wk.get_bit(position) {return Kind::King}
        }

        if self.black.get_bit(position) {
            if self.bp.get_bit(position) {return Kind::Pawn}
            if self.bn.get_bit(position) {return Kind::Knight}
            if self.bb.get_bit(position) {return Kind::Bishop}
            if self.br.get_bit(position) {return Kind::Rook}
            if self.bq.get_bit(position) {return Kind::Queen}
            if self.bk.get_bit(position) {return Kind::King}
        }

        Kind::Undefined
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
#[derive(Copy, Clone, Debug)]
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
            0 => Square::A8,
            1 => Square::B8,
            2 => Square::C8,
            3 => Square::D8,
            4 => Square::E8,
            5 => Square::F8,
            6 => Square::G8,
            7 => Square::H8,
            8 => Square::A7,
            9 => Square::B7,
            10 => Square::C7,
            11 => Square::D7,
            12 => Square::E7,
            13 => Square::F7,
            14 => Square::G7,
            15 => Square::H7,
            16 => Square::A6,
            17 => Square::B6,
            18 => Square::C6,
            19 => Square::D6,
            20 => Square::E6,
            21 => Square::F6,
            22 => Square::G6,
            23 => Square::H6,
            24 => Square::A5,
            25 => Square::B5,
            26 => Square::C5,
            27 => Square::D5,
            28 => Square::E5,
            29 => Square::F5,
            30 => Square::G5,
            31 => Square::H5,
            32 => Square::A4,
            33 => Square::B4,
            34 => Square::C4,
            35 => Square::D4,
            36 => Square::E4,
            37 => Square::F4,
            38 => Square::G4,
            39 => Square::H4,
            40 => Square::A3,
            41 => Square::B3,
            42 => Square::C3,
            43 => Square::D3,
            44 => Square::E3,
            45 => Square::F3,
            46 => Square::G3,
            47 => Square::H3,
            48 => Square::A2,
            49 => Square::B2,
            50 => Square::C2,
            51 => Square::D2,
            52 => Square::E2,
            53 => Square::F2,
            54 => Square::G2,
            55 => Square::H2,
            56 => Square::A1,
            57 => Square::B1,
            58 => Square::C1,
            59 => Square::D1,
            60 => Square::E1,
            61 => Square::F1,
            62 => Square::G1,
            63 => Square::H1,
            _ => Square::Undefined
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Kind {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
    Undefined,
}

#[derive(Copy, Clone, Debug)]
pub enum Color {
    White,
    Black,
    Undefined
}
