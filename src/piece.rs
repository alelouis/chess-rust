use crate::board::Square;

pub struct Piece<'a> {
    pub position: &'a Square,
}

impl Piece<'_> {
    pub fn new(position: &Square) -> Piece {
        Piece { position }
    }
}
