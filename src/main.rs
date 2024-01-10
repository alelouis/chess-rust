mod app;
mod board;
mod piece;

extern crate find_folder;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate sprite;

use crate::board::Board;
use crate::piece::{Kind, Piece};
use app::App;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use piston::{Button, CursorEvent, EventLoop, MouseCursorEvent, PressEvent, ReleaseEvent};
use sprite::Scene;

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let height = 768;
    let width = 768;
    let mut window: Window = WindowSettings::new("chess", [height, width])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = App {
        gl: GlGraphics::new(opengl),
        board: Board::new(),
        pieces: vec![],
        scene: Scene::new(),
        height: height,
        width: width,
        x: 0.0,
        y: 0.0,
        pressed: false,
        click: false,
        release: false,
        active_piece: None,
    };

    game.init();

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(Button::Mouse(button)) = e.press_args() {
            game.pressed = true;
            game.click = true;
        }
        if let Some(Button::Mouse(button)) = e.release_args() {
            game.pressed = false;
            game.release = true;
        };
        if let Some(args) = e.render_args() {
            game.render(&args);
        }
        if let Some(args) = e.update_args() {
            game.update(&args);
        }

        e.mouse_cursor(|pos| {
            game.x = pos[0] as f32;
            game.y = pos[1] as f32;
        });
    }
}
