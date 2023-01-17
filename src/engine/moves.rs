use crate::engine::bitboard::{Color, Kind, Square};

#[derive(Debug, Copy, Clone)]
pub struct Move{
    pub color: Color,
    pub kind: Kind,

    pub from: u8,
    pub to: u8,

    pub captured: Kind,
    pub capture: bool,
    pub en_passant: u8,
    pub en_passant_capture: u8,

    pub promotion: bool,
    pub promotion_to: Kind,
}

impl Move {
    pub fn new(color: Color, kind: Kind, from: u8, to: u8, captured: Kind, capture: bool, en_passant: u8, en_passant_capture: u8, promotion: bool, promotion_to: Kind) -> Self {
        Self{color, kind, from, to, captured, capture, en_passant, en_passant_capture, promotion, promotion_to}
    }

    pub fn print(&self) {
        let color = &self.color;
        let kind = &self.kind;
        let from = Square::from(*&self.from);
        let to = Square::from(*&self.to);
        let capture = &self.capture;
        let captured = &self.captured;
        // println!("{color:?}, {kind:?}, {from:?} -> {to:?}");
        println!("{from:?}{to:?}: {color:?}, {kind:?}, Capture: {capture:?}, {captured:?}");
    }
}