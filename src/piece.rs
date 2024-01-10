use crate::board::Square;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{Texture, TextureSettings};
use piston_window::*;
use sprite::*;
use std::rc::Rc;
use uuid::Uuid;

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
        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets")
            .unwrap();

        let tex =
            Rc::new(Texture::from_path(assets.join("rust.png"), &TextureSettings::new()).unwrap());
        let mut sprite = Sprite::from_texture(tex);
        sprite.set_position(0.0, 0.0);
        sprite.set_scale(0.5, 0.5);
        let sprite_id = scene.add_child(sprite);

        Piece {
            kind,
            sprite_id,
            id,
        }
    }
}
