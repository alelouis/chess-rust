mod app;
mod board;

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use opengl_graphics::{GlGraphics, OpenGL};
use glutin_window::GlutinWindow as Window;
use piston::window::WindowSettings;
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use app::App;
use crate::board::Board;


fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let height = 512;
    let width = 512;
    let mut window: Window = WindowSettings::new("chess", [height, width])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut game = App {
        gl: GlGraphics::new(opengl),
        board: Board::new(),
        height: height,
        width: width
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            game.render(&args);
        }

        if let Some(args) = e.update_args() {
            game.update(&args);
        }
    }
}
