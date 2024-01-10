use crate::piece::Piece;
use uuid::Uuid;

const BLACK: [f32; 4] = [209.0 / 255.0, 139.0 / 255.0, 71.0 / 255.0, 1.0];
const WHITE: [f32; 4] = [255.0 / 255.0, 206.0 / 255.0, 158.0 / 255.0, 1.0];

#[derive(Copy, Clone, Debug)]
pub struct Square {
    pub color: [f32; 4],
    pub index: u8,
    pub piece: Option<u8>,
}

pub struct Board {
    pub squares: Vec<Square>,
}

impl Square {
    pub fn new(color: [f32; 4], index: u8) -> Self {
        Square {
            color,
            index,
            piece: None,
        }
    }
    pub fn index_to_file_rank(self: &Self) -> (u8, u8) {
        (self.index / 8, 7 - self.index % 8)
    }
    pub fn xy_to_file_rank(x: f32, y: f32, square_size: f32) -> (u8, u8) {
        let square_size = square_size as f32;
        (
            (x / square_size).floor() as u8,
            7 - (y / square_size).floor() as u8,
        )
    }
    pub fn file_rank_to_index(file: u8, rank: u8) -> u8 {
        return file + 8 * rank;
    }

    pub fn file_rank_to_xy(file: u8, rank: u8, square_size: f32) -> (f32, f32) {
        (
            file as f32 * square_size + square_size / 2.0,
            rank as f32 * square_size + square_size / 2.0,
        )
    }
}

impl Board {
    pub fn new() -> Self {
        let mut squares: Vec<Square> = Vec::new();
        for i in 0..64 {
            if (i + i / 8) % 2 == 0 {
                squares.push(Square::new(WHITE, i))
            } else {
                squares.push(Square::new(BLACK, i))
            }
        }
        Board { squares }
    }
    pub fn get_square_at_file_rank(&mut self, file: u8, rank: u8) -> &Square {
        &self.squares[(rank + file * 8) as usize]
    }
}
