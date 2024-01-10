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
    pub width: u32,
    pub height: u32,
    pub x: f32,
    pub y: f32,
    pub pressed: bool,
    pub active_piece: Option<Uuid>,
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

impl App {
    pub fn render(&mut self, args: &RenderArgs) {
        let mut scene = Scene::new();
        let mut piece = Piece::new(Kind::Pawn, &mut scene, 0);
        self.render_board(args);
        self.render_pieces(&scene, &mut piece, args);
    }

    pub fn render_board(&mut self, args: &RenderArgs) {
        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen
            clear([0.0, 1.0, 0.0, 1.0], gl);

            let square_size = self.height as f32 / 8.0;
            for s in self.board.squares.iter() {
                let (rank, file) = s.index_to_file_rank();
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

    pub fn render_pieces<'a>(
        &mut self,
        scene: &Scene<Texture>,
        piece: &mut Piece,
        args: &RenderArgs,
    ) {
        let square_size = self.height as f32 / 8.0;
        let (mut x, mut y) = (0.0, 0.0);
        self.gl.draw(args.viewport(), |c, gl| {
            if self.pressed {
                (x, y) = (self.x, self.y);
            } else {
                let (next_file, next_rank) = Square::xy_to_file_rank(self.x, self.y, square_size);
                let index = Square::file_rank_to_index(next_file, next_rank) as usize;
                self.board.squares[index].piece = Some(piece.id);
                (x, y) = Square::file_rank_to_xy(next_file, next_file, square_size);
            }
            let transform = c.transform.trans((x).into(), (y).into());
            scene.draw(transform, gl);
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        let square_size = self.height as f32 / 8.0;

        if self.pressed {
            let (file, rank) = Square::xy_to_file_rank(self.x, self.y, square_size);
            let index_clicked = Square::file_rank_to_index(file, rank) as usize;
            let id_clicked = self.board.squares[index_clicked].piece;
            println!("{:?}", id_clicked);
        }
    }
}
