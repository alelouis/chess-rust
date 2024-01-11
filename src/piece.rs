use crate::board::Square;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{Texture, TextureSettings};
use piston_window::*;
use sprite::*;
use std::collections::HashMap;
use std::rc::Rc;
use uuid::Uuid;

fn get_set() -> HashMap<Kind, String> {
    let mut set = HashMap::new();
    set.insert(Kind::Pawn, String::from("bP.png"));
    set.insert(Kind::Bishop, String::from("bB.png"));
    set.insert(Kind::Knight, String::from("bN.png"));
    set.insert(Kind::King, String::from("bK.png"));
    set.insert(Kind::Queen, String::from("bQ.png"));
    set.insert(Kind::Rook, String::from("bR.png"));
    set
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub enum Kind {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
}

pub struct Piece {
    pub kind: Kind,
    pub sprite_id: Uuid,
    pub id: u8,
}

impl Piece {
    pub fn new(kind: Kind, scene: &mut Scene<Texture>, id: u8) -> Piece {
        let set = get_set();

        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets")
            .unwrap();

        let s = set.get(&kind).unwrap();
        let tex = Rc::new(
            Texture::from_path(
                assets.join(format!("set/merida/{s}")),
                &TextureSettings::new(),
            )
            .unwrap(),
        );
        let mut sprite = Sprite::from_texture(tex);
        sprite.set_position(0.0, 0.0);
        sprite.set_scale(0.09, 0.09);
        let sprite_id = scene.add_child(sprite);

        Piece {
            kind,
            sprite_id,
            id,
        }
    }
}
