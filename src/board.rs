use crate::piece::{Color, Kind, Piece};
use opengl_graphics::Texture;
use sprite::Scene;

const BLACK: [f32; 4] = [81.0 / 255.0, 79.0 / 255.0, 174.0 / 255.0, 1.0];
const WHITE: [f32; 4] = [133.0 / 255.0, 131.0 / 255.0, 198.0 / 255.0, 1.0];
pub const RANK: [&str; 8] = ["1", "2", "3", "4", "5", "6", "7", "8"];
pub const FILE: [&str; 8] = ["a", "b", "c", "d", "e", "f", "g", "h"];

#[derive(Copy, Clone, Debug)]
pub struct Square {
    pub color: [f32; 4],
    pub index: u8,
    pub piece: Option<u8>,
}

pub struct Board {
    pub squares: Vec<Square>,
    pub flip: bool,
}

impl Square {
    pub fn new(color: [f32; 4], index: u8) -> Self {
        Square {
            color,
            index,
            piece: None,
        }
    }

    pub fn file_rank(self: &Self) -> (u8, u8) {
        (self.index % 8, self.index / 8)
    }

    pub fn file_rank_to_index(file: u8, rank: u8) -> u8 {
        return file + 8 * rank;
    }

    pub fn file_rank_to_xy(file: u8, rank: u8, square_size: f32, height: f32) -> (f32, f32) {
        (
            file as f32 * square_size + square_size / 2.0,
            height - (rank as f32 * square_size + square_size / 2.0),
        )
    }

    pub fn xy_to_file_rank(x: f32, y: f32, square_size: f32, height: f32) -> (u8, u8) {
        (
            (x / square_size).floor() as u8,
            ((height - y) / square_size).floor() as u8,
        )
    }

    pub fn xy_to_index(x: f32, y: f32, square_size: f32, height: f32) -> u8 {
        let (file, rank) = Self::xy_to_file_rank(x, y, square_size, height);
        Self::file_rank_to_index(file, rank)
    }
}

impl Board {
    pub fn new(flip: bool) -> Self {
        let mut squares: Vec<Square> = Vec::new();
        for i in 0..64 {
            if (i + i / 8) % 2 == 0 {
                squares.push(Square::new(WHITE, i))
            } else {
                squares.push(Square::new(BLACK, i))
            }
        }
        Board { squares, flip }
    }

    pub fn get_square_at_file_rank(&mut self, file: u8, rank: u8) -> &Square {
        &self.squares[(rank + file * 8) as usize]
    }

    /// Forsyth–Edwards Notation parsing
    pub fn load_fen(&mut self, scene: &mut Scene<Texture>, fen: String) -> Vec<Piece> {
        let mut index = 0;
        let mut pieces = vec![];
        for rank in fen.split("/") {
            for c in rank.chars() {
                if c.is_ascii_digit() {
                    index += c.to_digit(10).unwrap();
                } else {
                    let color = match c.is_ascii_lowercase() {
                        true => Color::White,
                        false => Color::Black,
                    };
                    pieces.push(match c.to_lowercase().last().unwrap() {
                        'p' => Piece::new(Kind::Pawn, color, scene, index as u8),
                        'n' => Piece::new(Kind::Knight, color, scene, index as u8),
                        'b' => Piece::new(Kind::Bishop, color, scene, index as u8),
                        'r' => Piece::new(Kind::Rook, color, scene, index as u8),
                        'q' => Piece::new(Kind::Queen, color, scene, index as u8),
                        'k' => Piece::new(Kind::King, color, scene, index as u8),
                        _ => {
                            panic!("Unknown letter found in FEN string: {c}")
                        }
                    });
                    self.squares[index as usize].piece = Some(pieces.last().unwrap().id);
                    index += 1;
                }
            }
        }
        pieces
    }
}
