use crate::board::{Board, Square};
use crate::piece::{Kind, Piece};
use glutin_window::GlutinWindow as Window;
use graphics::{clear, polygon, rectangle, Transformed};
use opengl_graphics::{GlGraphics, OpenGL, Texture};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use sprite::Scene;
use uuid::Uuid;

pub struct App {
    pub gl: GlGraphics,
    pub board: Board,
    pub pieces: Vec<Piece>,
    pub scene: Scene<Texture>,
    pub width: u32,
    pub height: u32,
    pub x: f32,
    pub y: f32,
    pub pressed: bool,
    pub click: bool,
    pub release: bool,
    pub active_piece: Option<u8>,
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

impl App {
    pub fn init(&mut self) {
        self.pieces.push(Piece::new(Kind::Pawn, &mut self.scene, 0));
        self.board.squares[0].piece = Some(self.pieces[0].id);

        self.pieces.push(Piece::new(Kind::Pawn, &mut self.scene, 1));
        self.board.squares[1].piece = Some(self.pieces[1].id);
    }

    pub fn render(&mut self, args: &RenderArgs) {
        self.render_board(args);
        self.render_pieces(args);
    }

    pub fn render_board(&mut self, args: &RenderArgs) {
        self.gl.draw(args.viewport(), |c, gl| {
            clear([0.0, 1.0, 0.0, 1.0], gl);
            let square_size = self.height as f32 / 8.0;
            for s in self.board.squares.iter() {
                let (rank, file) = s.file_rank();
                let (x, y) = (file as f32 * square_size, rank as f32 * square_size);
                let transform = c.transform.trans(x.into(), y.into());
                rectangle(
                    s.color,
                    rectangle::square(0.0, 0.0, square_size.into()),
                    transform,
                    gl,
                );
            }
        });
    }

    pub fn render_pieces(&mut self, args: &RenderArgs) {
        let square_size = self.height as f32 / 8.0;
        self.gl.draw(args.viewport(), |c, gl| {
            if let Some(active_piece_id) = self.active_piece {
                let mut active_piece_sprite_id = None;
                for p in self.pieces.iter() {
                    if p.id == active_piece_id {
                        active_piece_sprite_id = Some(p.sprite_id);
                    }
                }
                let child = self
                    .scene
                    .child_mut(active_piece_sprite_id.expect("No sprite id."))
                    .expect("No child for specified uuid.");
                child.set_position(self.x.into(), self.y.into());
            } else {
                for square in self.board.squares.iter() {
                    if let Some(piece_id) = square.piece {
                        let mut active_piece_sprite_id = None;
                        for p in self.pieces.iter() {
                            if p.id == piece_id {
                                active_piece_sprite_id = Some(p.sprite_id);
                                let child = self
                                    .scene
                                    .child_mut(active_piece_sprite_id.expect("No sprite id."))
                                    .expect("No child for specified uuid.");
                                let (file, rank) = square.file_rank();
                                let (x, y) = Square::file_rank_to_xy(file, rank, square_size);
                                child.set_position(x.into(), y.into());
                            }
                        }
                    }
                }
            };

            self.scene.draw(c.transform, gl);
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        let square_size = self.height as f32 / 8.0;

        if self.click {
            let (file, rank) = Square::xy_to_file_rank(self.x, self.y, square_size);
            let index_clicked = Square::file_rank_to_index(file, rank) as usize;
            let id_clicked = self.board.squares[index_clicked].piece;

            if let Some(some_id_clicked) = id_clicked {
                self.active_piece = Some(some_id_clicked);
                self.board.squares[index_clicked].piece = None;
            }
            self.click = false;
        }
        if self.release {
            if self.active_piece.is_some() {
                let (file, rank) = Square::xy_to_file_rank(self.x, self.y, square_size);
                let index_released = Square::file_rank_to_index(file, rank) as usize;
                self.board.squares[index_released].piece = self.active_piece;
            }
            self.active_piece = None;
            self.release = false;
        }
    }
}
