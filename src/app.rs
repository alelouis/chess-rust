use crate::board::Board;
use crate::piece::Piece;
use graphics::{clear, polygon, rectangle, Transformed};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};

pub struct App {
    pub gl: GlGraphics,
    pub board: Board,
    pub width: u32,
    pub height: u32,
    pub x: f32,
    pub y: f32,
    pub pressed: bool,
}

impl App {
    pub fn render(&mut self, args: &RenderArgs) {
        let board = Board::new();
        let piece = Piece::new(board.get_square_at_file_rank(0, 0));
        self.render_board(&board, args);
        self.render_piece(&piece, args);
    }

    pub fn render_board(&mut self, board: &Board, args: &RenderArgs) {
        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen
            clear([0.0, 1.0, 0.0, 1.0], gl);

            let square_size = self.height as f32 / 8.0;
            for s in board.squares.iter() {
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

    pub fn render_piece(&mut self, piece: &Piece, args: &RenderArgs) {
        let square_size = self.height as f32 / 8.0;
        let (mut x, mut y) = (0.0, 0.0);
        self.gl.draw(args.viewport(), |c, gl| {
            let black = [0.0, 0.0, 0.0, 1.0];
            let green = [0.0, 1.0, 0.0, 1.0];
            let (file, rank) = piece.position.index_to_file_rank();
            // let (x, y) = (file as f32 * square_size, rank as f32 * square_size);
            if self.pressed {
                (x, y) = (self.x, self.y);
            } else {
                (x, y) = (
                    (self.x / square_size).floor() * square_size + square_size / 2.0,
                    (self.y / square_size).floor() * square_size + square_size / 2.0,
                );
            }
            let transform = c.transform.trans((x).into(), (y).into());
            let color = if self.pressed { green } else { black };
            polygon(
                color,
                &[
                    [0.0, (-square_size / 2.0).into()],
                    [(square_size / 2.0).into(), 0.0],
                    [0.0, (square_size / 2.0).into()],
                    [(-square_size / 2.0).into(), 0.0],
                ],
                transform,
                gl,
            );
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
    }
}
