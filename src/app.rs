use graphics::{clear, polygon, rectangle, Transformed};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use crate::board::Board;
use crate::piece::Piece;

pub struct App {
    pub gl: GlGraphics,
    pub board: Board,
    pub width: u32,
    pub height: u32,
}

impl App {
    pub fn render(&mut self, args: &RenderArgs) {
        let board = Board::new();
        let piece = Piece::new(board.get_square_at_file_rank(7, 2));
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
                let transform = c
                    .transform
                    .trans(x.into(), y.into());
                rectangle(s.color, rectangle::square(0.0, 0.0, square_size.into()), transform, gl);
            }

        });
    }

    pub fn render_piece(&mut self, piece: &Piece, args: &RenderArgs) {
        let square_size = self.height as f32 / 8.0;
        self.gl.draw(args.viewport(), |c, gl| {
            let magenta = [1.0, 0.0, 0.5, 1.0];
            let (file, rank) = piece.position.index_to_file_rank();
            let (x, y) = (file as f32 * square_size, rank as f32 * square_size);
            let transform = c
                .transform
                .trans((x + square_size / 2.0).into(), (y + square_size / 2.0).into());
            polygon(magenta, &[
                [0.0, (-square_size / 2.0).into()],
                [(square_size / 2.0).into(), 0.0],
                [0.0, (square_size / 2.0).into()],
                [(-square_size / 2.0).into(), 0.0],
            ], transform, gl);
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.

    }
}