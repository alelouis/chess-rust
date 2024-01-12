use crate::board::{Board, Square, FILE, RANK};
use crate::fonts;
use crate::piece::Piece;
use ai_behavior::{Action, Sequence};
use config::Config;
use freetype::Face;
use graphics::math::Scalar;
use graphics::{clear, rectangle, Transformed};
use opengl_graphics::{GlGraphics, Texture};
use piston::input::{RenderArgs, UpdateArgs};
use sprite::{
    Blink, Ease, EaseFunction, FadeIn, FadeOut, MoveBy, MoveTo, RotateBy, RotateTo, ScaleBy,
    ScaleTo, Scene,
};

pub struct Chess {
    pub config: Config,
    pub global_scale: f32,
    pub gl: GlGraphics,
    pub board: Board,
    pub pieces: Vec<Piece>,
    pub scene: Scene<Texture>,
    pub window_size: u32,
    pub square_size: f32,
    pub x: f32,
    pub y: f32,
    pub pressed: bool,
    pub click: bool,
    pub release: bool,
    pub active_piece: Option<u8>,
    pub face: Face,
}

impl Chess {
    pub fn init(&mut self) {
        self.pieces = self.board.load_fen(
            &mut self.scene,
            self.config
                .get("engine.fen")
                .expect("Couldn't load FEN from config."),
        );
    }

    pub fn render(&mut self, args: &RenderArgs) {
        self.render_board(args);
        self.render_pieces(args);
    }

    pub fn render_board(&mut self, args: &RenderArgs) {
        let mut font_scale = self
            .config
            .get("render.font_scale")
            .expect("Couldn't find font_scale in config.");
        font_scale = (font_scale as f32 * self.global_scale) as f64;
        self.gl.draw(args.viewport(), |c, gl| {
            clear([0.0, 1.0, 0.0, 1.0], gl);
            for s in self.board.squares.iter() {
                let (rank, file) = s.file_rank();
                let (x, y) =
                    Square::file_rank_to_xy(file, rank, self.square_size, self.window_size as f32);
                let transform = c.transform.trans(
                    (x - self.square_size / 2.0).into(),
                    (y - self.square_size / 2.0).into(),
                );
                rectangle(
                    s.color,
                    rectangle::square(0.0, 0.0, self.square_size.into()),
                    transform,
                    gl,
                );
                let mut glyphs = vec![];
                let mut glyph_offset = (0.0, 0.0);
                if file == 0 {
                    glyphs = fonts::glyphs(&mut self.face, format!("{} ", RANK[rank as usize]));
                    glyph_offset = (-self.square_size * 0.45, -self.square_size * 0.20);
                    fonts::render_text(
                        &glyphs,
                        &c.trans((x + glyph_offset.0).into(), (y + glyph_offset.1).into())
                            .scale(font_scale, font_scale),
                        gl,
                    );
                }
                if rank == 0 {
                    glyphs = fonts::glyphs(&mut self.face, format!("{} ", FILE[file as usize]));
                    glyph_offset = (self.square_size * 0.30, self.square_size * 0.45);
                    fonts::render_text(
                        &glyphs,
                        &c.trans((x + glyph_offset.0).into(), (y + glyph_offset.1).into())
                            .scale(font_scale, font_scale),
                        gl,
                    );
                }
            }
        });
    }

    pub fn render_pieces(&mut self, args: &RenderArgs) {
        let mut piece_scale = self
            .config
            .get("render.piece_scale")
            .expect("Couldn't find piece_scale in config.");
        piece_scale = (piece_scale as f32 * self.global_scale) as f64;
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
                                let (x, y) = Square::file_rank_to_xy(
                                    file,
                                    rank,
                                    self.square_size,
                                    self.window_size as f32,
                                );

                                let (cx, cy) = child.get_position();
                                if (cx as f32, cy as f32) != (x, y) {
                                    if self
                                        .scene
                                        .running_for_child(active_piece_sprite_id.unwrap())
                                        == Some(0)
                                    {
                                        let tran = Action(Ease(
                                            EaseFunction::ExponentialOut,
                                            Box::new(MoveTo(0.1, x as Scalar, y as Scalar)),
                                        ));

                                        let rotate = Sequence(vec![Action(Ease(
                                            EaseFunction::BounceInOut,
                                            Box::new(RotateTo(0.05, 0.0)),
                                        ))]);

                                        let scale = Sequence(vec![Action(Ease(
                                            EaseFunction::ExponentialOut,
                                            Box::new(ScaleTo(0.2, piece_scale, piece_scale)),
                                        ))]);

                                        self.scene.run(active_piece_sprite_id.unwrap(), &tran);
                                        self.scene.run(active_piece_sprite_id.unwrap(), &rotate);
                                        self.scene.run(active_piece_sprite_id.unwrap(), &scale);
                                    }
                                }
                                // child.set_position(x.into(), y.into());
                            }
                        }
                    }
                }
            };
            for child in self.scene.children() {
                child.draw(c.transform, gl);
            }
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        if self.click {
            let index_clicked =
                Square::xy_to_index(self.x, self.y, self.square_size, self.window_size as f32)
                    as usize;
            if let Some(id_clicked) = self.board.squares[index_clicked].piece {
                self.active_piece = Some(id_clicked);
                self.board.squares[index_clicked].piece = None;

                let mut active_piece_sprite_id = None;
                let mut piece_scale = self
                    .config
                    .get("render.piece_scale")
                    .expect("Couldn't find piece_scale in config.");
                piece_scale = (piece_scale as f32 * self.global_scale) as f64;
                for p in self.pieces.iter() {
                    if p.id == self.active_piece.unwrap() {
                        active_piece_sprite_id = Some(p.sprite_id);
                    }
                }

                let scale = Sequence(vec![Action(Ease(
                    EaseFunction::ExponentialOut,
                    Box::new(ScaleTo(0.1, 1.3 * piece_scale, 1.3 * piece_scale)),
                ))]);

                let rotate = Sequence(vec![Action(Ease(
                    EaseFunction::ExponentialOut,
                    Box::new(RotateBy(0.1, -10.0)),
                ))]);

                self.scene.run(active_piece_sprite_id.unwrap(), &scale);
                self.scene.run(active_piece_sprite_id.unwrap(), &rotate);
            }
            self.click = false;
        }
        if self.release {
            if self.active_piece.is_some() {
                let index_released =
                    Square::xy_to_index(self.x, self.y, self.square_size, self.window_size as f32)
                        as usize;
                self.board.squares[index_released].piece = self.active_piece;
            }

            self.active_piece = None;
            self.release = false;
        }
    }
}