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
        println!();
        println!("\x1b[34m   a b c d e f g h\x1b[0m");
        println!();
    }

    pub fn empty() -> Self {
        Self(0)
    }

    fn from_sq(square: Square) -> Self {
        Self(1 << square as u8)
    }

    pub fn set_bit(&mut self, square: Square) {
        self.0 |= BitBoard::from_sq(square).0;
    }

    fn toggle_bit(&mut self, square: Square) {
        self.0 ^= BitBoard::from_sq(square).0;
    }

}

pub struct BitPos {
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
