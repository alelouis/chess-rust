use crate::piece::Color::{Black, White};
use opengl_graphics::{Texture, TextureSettings};
use sprite::*;
use std::collections::HashMap;
use std::rc::Rc;
use uuid::Uuid;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum Color {
    White,
    Black,
}
#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum Kind {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
}

#[derive(Debug)]
pub struct Piece {
    pub kind: Kind,
    pub sprite_id: Uuid,
    pub id: u8,
    pub color: Color,
}

fn get_set() -> HashMap<(Kind, Color), String> {
    let mut set = HashMap::new();
    set.insert((Kind::Pawn, White), String::from("wP.png"));
    set.insert((Kind::Bishop, White), String::from("wB.png"));
    set.insert((Kind::Knight, White), String::from("wN.png"));
    set.insert((Kind::King, White), String::from("wK.png"));
    set.insert((Kind::Queen, White), String::from("wQ.png"));
    set.insert((Kind::Rook, White), String::from("wR.png"));
    set.insert((Kind::Pawn, Black), String::from("bP.png"));
    set.insert((Kind::Bishop, Black), String::from("bB.png"));
    set.insert((Kind::Knight, Black), String::from("bN.png"));
    set.insert((Kind::King, Black), String::from("bK.png"));
    set.insert((Kind::Queen, Black), String::from("bQ.png"));
    set.insert((Kind::Rook, Black), String::from("bR.png"));
    set
}

impl Piece {
    pub fn new(kind: Kind, color: Color, scene: &mut Scene<Texture>, id: u8) -> Piece {
        let set: HashMap<(Kind, Color), String> = get_set();
        let assets = find_folder::Search::ParentsThenKids(1, 1)
            .for_folder("assets")
            .unwrap();
        let s = set.get(&(kind, color)).unwrap();
        let tex = Rc::new(
            Texture::from_path(
                assets.join(format!("set/merida/{s}")),
                &TextureSettings::new(),
            )
            .unwrap(),
        );
        let mut sprite = Sprite::from_texture(tex);
        sprite.set_scale(0.0, 0.0);
        let sprite_id: Uuid = scene.add_child(sprite);

        Piece {
            kind,
            color,
            sprite_id,
            id,
        }
    }

    pub fn find_color_for_id(pieces: &Vec<Piece>, id: u8) -> Option<Color> {
        let mut color: Option<Color> = None;
        for piece in pieces.iter() {
            if piece.id == id {
                color = Some(piece.color)
            }
        }
        color
    }

    pub fn find_sprite_id_for_id(pieces: &Vec<Piece>, id: u8) -> Option<Uuid> {
        let mut sprite_id = None;
        for piece in pieces.iter() {
            if piece.id == id {
                sprite_id = Some(piece.sprite_id)
            }
        }
        sprite_id
    }

    pub fn remove_id_from_pieces(pieces: &mut Vec<Piece>, id: u8) {
        for (idx, piece) in pieces.iter().enumerate() {
            if piece.id == id {
                pieces.remove(idx);
                break;
            }
        }
    }

    pub fn get_piece_from_id(pieces: &Vec<Piece>, id: u8) -> Option<&Piece> {
        let mut return_piece = None;
        for piece in pieces.iter() {
            if piece.id == id {
                return_piece = Some(piece);
                break;
            }
        }
        return_piece
    }
}
