use crate::moves::MoveType;
use crate::piece::{Color, Kind, Piece};
use opengl_graphics::Texture;
use piston::Touch::Move;
use sprite::Scene;

const BLACK: [f32; 4] = [81.0 / 255.0, 79.0 / 255.0, 174.0 / 255.0, 1.0];
const WHITE: [f32; 4] = [133.0 / 255.0, 131.0 / 255.0, 198.0 / 255.0, 1.0];
pub const RANK: [&str; 8] = ["1", "2", "3", "4", "5", "6", "7", "8"];
pub const FILE: [&str; 8] = ["a", "b", "c", "d", "e", "f", "g", "h"];

#[derive(Copy, Clone, Debug)]
pub struct Square {
    pub color: [f32; 4],
    pub index: u8,
    pub piece_id: Option<u8>,
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
            piece_id: None,
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

    pub fn index_to_file_rank(index: u8) -> (u8, u8) {
        (index % 8, index / 8)
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
                    self.squares[index as usize].piece_id = Some(pieces.last().unwrap().id);
                    index += 1;
                }
            }
        }
        pieces
    }

    pub fn generate_legal_moves(
        &mut self,
        pieces: &Vec<Piece>,
        piece: &Piece,
        file: u8,
        rank: u8,
    ) -> Vec<(u8, u8)> {
        let mut legal_moves: Vec<(i16, i16)> = vec![];
        match piece.kind {
            Kind::Pawn => {
                legal_moves = self.generate_pawn(pieces, piece, file, rank);
            }
            Kind::Bishop => {
                let deltas = vec![(1, 1), (1, -1), (-1, -1), (-1, 1)];
                legal_moves = self.generate_line(deltas, pieces, piece, file, rank);
            }
            Kind::Knight => {
                let pos: Vec<(i16, i16)> = vec![
                    (1, 2),
                    (2, 1),
                    (1, -2),
                    (2, -1),
                    (-1, 2),
                    (-2, 1),
                    (-1, -2),
                    (-2, -1),
                ];
                legal_moves = self.generate_custom(pos, pieces, piece, file, rank);
            }
            Kind::Rook => {
                let deltas = vec![(1, 0), (0, -1), (-1, 0), (0, 1)];
                legal_moves = self.generate_line(deltas, pieces, piece, file, rank);
            }
            Kind::Queen => {
                let deltas = vec![
                    (1, 1),
                    (1, -1),
                    (-1, -1),
                    (-1, 1),
                    (1, 0),
                    (0, -1),
                    (-1, 0),
                    (0, 1),
                ];
                legal_moves = self.generate_line(deltas, pieces, piece, file, rank);
            }
            Kind::King => {
                let pos: Vec<(i16, i16)> = vec![
                    (0, 1),
                    (1, 1),
                    (1, 0),
                    (0, -1),
                    (-1, -1),
                    (-1, 0),
                    (-1, 1),
                    (1, -1),
                ];
                legal_moves = self.generate_custom(pos, pieces, piece, file, rank);
            }
        }
        legal_moves
            .iter()
            .map(|(file, rank)| {
                (
                    u8::try_from(file.clone()).unwrap(),
                    u8::try_from(rank.clone()).unwrap(),
                )
            })
            .collect()
    }

    pub fn generate_pawn(
        &mut self,
        pieces: &Vec<Piece>,
        piece: &Piece,
        file: u8,
        rank: u8,
    ) -> Vec<(i16, i16)> {
        let mut legal_moves: Vec<(i16, i16)> = vec![];

        let color = piece.color;
        let mut moves = vec![];
        if color == Color::White {
            moves.append(&mut vec![(-1, 1), (1, 1)])
        } else {
            moves.append(&mut vec![(-1, -1), (1, -1)])
        }
        let attack_moves: Vec<(i16, i16)> = moves
            .iter()
            .map(|(f, r)| (f + file as i16, r + rank as i16))
            .filter(|(f, r)| Board::is_inside_board(f.clone(), r.clone()))
            .filter(|(f, r)| {
                Board::is_attacking(&pieces, &self.squares, piece, f.clone(), r.clone())
            })
            .collect();

        legal_moves.append(&mut attack_moves.clone());

        let mut moves = vec![];
        if color == Color::White {
            moves.append(&mut vec![(0, 1)])
        } else {
            moves.append(&mut vec![(0, -1)])
        }

        let mut normal_moves: Vec<(i16, i16)> = moves
            .iter()
            .map(|(f, r)| (f + file as i16, r + rank as i16))
            .filter(|(f, r)| Board::is_inside_board(f.clone(), r.clone()))
            .filter(|(f, r)| {
                !Board::is_attacking(&pieces, &self.squares, piece, f.clone(), r.clone())
            })
            .filter(|(f, r)| !Board::is_allied(&pieces, &self.squares, piece, f.clone(), r.clone()))
            .collect();
        legal_moves.append(&mut normal_moves);
        legal_moves
    }

    pub fn generate_custom(
        &mut self,
        custom: Vec<(i16, i16)>,
        pieces: &Vec<Piece>,
        piece: &Piece,
        file: u8,
        rank: u8,
    ) -> Vec<(i16, i16)> {
        let mut first = true;
        custom
            .iter()
            .map(|(f, r)| (f + file as i16, r + rank as i16))
            .filter(|(f, r)| {
                Board::is_legal(
                    &pieces,
                    &self.squares,
                    piece,
                    &mut first,
                    f.clone(),
                    r.clone(),
                    false,
                )
            })
            .collect()
    }

    pub fn generate_line(
        &mut self,
        deltas: Vec<(i16, i16)>,
        pieces: &Vec<Piece>,
        piece: &Piece,
        file: u8,
        rank: u8,
    ) -> Vec<(i16, i16)> {
        let mut legal_moves: Vec<(i16, i16)> = vec![];
        for (df, dr) in deltas {
            let mut first = true;
            let mut ite: Vec<(i16, i16)> = vec![(file as i16, rank as i16)]
                .iter()
                .cycle()
                .enumerate()
                .map(|(index, (f, r))| (f + (1 + index) as i16 * df, r + (1 + index) as i16 * dr))
                .take_while(|(f, r)| {
                    Board::is_legal(
                        &pieces,
                        &self.squares,
                        piece,
                        &mut first,
                        f.clone(),
                        r.clone(),
                        true,
                    )
                })
                .collect();
            legal_moves.append(&mut ite);
        }
        legal_moves
    }

    fn is_inside_board(file: i16, rank: i16) -> bool {
        (file >= 0) & (rank >= 0) & (file < 8) & (rank < 8)
    }

    fn is_allied(
        pieces: &Vec<Piece>,
        squares: &Vec<Square>,
        piece: &Piece,
        file: i16,
        rank: i16,
    ) -> bool {
        let mut allied = false;
        let index = Square::file_rank_to_index(
            u8::try_from(file.clone()).unwrap(),
            u8::try_from(rank.clone()).unwrap(),
        ) as usize;

        if squares[index].piece_id.is_some() {
            let dest_piece =
                Piece::get_piece_from_id(pieces, squares[index].piece_id.unwrap()).unwrap();
            if dest_piece.color == piece.color {
                allied = true;
            } else {
                allied = false;
            }
        }
        allied
    }

    fn is_attacking(
        pieces: &Vec<Piece>,
        squares: &Vec<Square>,
        piece: &Piece,
        file: i16,
        rank: i16,
    ) -> bool {
        let index = Square::file_rank_to_index(
            u8::try_from(file.clone()).unwrap(),
            u8::try_from(rank.clone()).unwrap(),
        ) as usize;

        let mut attacking = false;

        if squares[index].piece_id.is_some() {
            let dest_piece =
                Piece::get_piece_from_id(pieces, squares[index].piece_id.unwrap()).unwrap();

            attacking = dest_piece.color != piece.color;
        }
        return attacking;
    }

    fn is_legal(
        pieces: &Vec<Piece>,
        squares: &Vec<Square>,
        piece: &Piece,
        first: &mut bool,
        file: i16,
        rank: i16,
        check_first: bool,
    ) -> bool {
        let mut is_legal = true;

        let is_inside = Board::is_inside_board(file, rank);
        is_legal &= is_inside;

        if is_inside {
            let index = Square::file_rank_to_index(
                u8::try_from(file.clone()).unwrap(),
                u8::try_from(rank.clone()).unwrap(),
            ) as usize;

            if squares[index].piece_id.is_some() {
                let dest_piece =
                    Piece::get_piece_from_id(pieces, squares[index].piece_id.unwrap()).unwrap();
                if dest_piece.color == piece.color {
                    is_legal &= false;
                } else {
                    if check_first {
                        if *first {
                            is_legal &= true;
                            *first = false;
                        } else {
                            is_legal &= false;
                        }
                    }
                }
            }
        }

        is_legal
    }

    pub fn move_piece(
        self: &mut Self,
        active_piece_id: Option<u8>,
        pieces: &mut Vec<Piece>,
        from_index: usize,
        to_index: usize,
    ) -> Result<MoveType, MoveType> {
        let (file, rank) = Square::index_to_file_rank(from_index as u8);
        let piece = Piece::get_piece_from_id(pieces, active_piece_id.expect("No active piece id."))
            .expect("No piece.");
        let legal_moves_indices: Vec<u8> = self
            .generate_legal_moves(pieces, &piece, file, rank)
            .iter()
            .map(|(file, rank)| Square::file_rank_to_index(file.clone(), rank.clone()))
            .collect();

        return if !legal_moves_indices.contains(&(to_index as u8)) {
            Err(MoveType::Illegal)
        } else if self.squares[to_index].piece_id.is_none() {
            self.squares[to_index].piece_id = active_piece_id;
            self.squares[from_index].piece_id = None;
            Ok(MoveType::Empty)
        } else {
            let target_piece_id = self.squares[to_index]
                .piece_id
                .expect("No target piece id.");

            self.squares[to_index].piece_id = active_piece_id;
            self.squares[from_index].piece_id = None;
            Ok(MoveType::Take(target_piece_id))
        };
    }
}
