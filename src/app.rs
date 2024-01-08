use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use crate::board::Board;

pub struct App {
    pub gl: GlGraphics,
    pub board: Board,
    pub width: u32,
    pub height: u32,
}

impl App {
    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        let square_size = self.height as f32 / 8.0;

        let board = Board::new();

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            for s in board.squares.iter() {
                let (rank, file) = s.index_to_file_rank();
                let square = rectangle::square(0.0, 0.0, square_size.into());

                let (x, y) = (file as f32 * square_size, rank as f32 * square_size);

                let transform = c
                    .transform
                    .trans(x.into(), y.into());

                // Draw a box rotating around the middle of the screen.
                rectangle(s.color, square, transform, gl);
            }
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.

    }
}