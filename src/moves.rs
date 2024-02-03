#[derive(Debug)]
pub enum MoveType {
    Empty,
    Take(u8),
    OccupiedBySameColor,
    Illegal,
}
