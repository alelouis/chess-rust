mod app;
mod board;
mod fonts;
mod piece;

extern crate find_folder;
extern crate freetype as ft;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate sprite;

use crate::board::Board;
use app::App;
use config::Config;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::window::WindowSettings;
use piston::{Button, CursorEvent, EventLoop, MouseCursorEvent, PressEvent, ReleaseEvent};
use sprite::Scene;

fn main() {
    // Load configuration

    let config = Config::builder()
        .add_source(config::File::with_name("config.toml"))
        .build()
        .unwrap();

    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let global_scale: f32 = config
        .get("render.global_scale")
        .expect("Couldn't find render.global_scale in config.");

    let mut window_size: u32 = config
        .get("render.window_size")
        .expect("Couldn't find render.window_size in config.");

    window_size = (window_size as f32 * global_scale) as u32;

    let mut window_settings = WindowSettings::new("Chess Engine", [window_size, window_size]);
    window_settings.set_samples(0);
    window_settings.set_resizable(false);
    let mut window: Window = window_settings
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    let freetype = ft::Library::init().unwrap();
    let font = assets.join("fonts/FiraSans-Regular.ttf");
    let face = freetype.new_face(&font, 0).unwrap();
    face.set_pixel_sizes(0, 100).unwrap();

    let mut game = App {
        config,
        global_scale,
        gl: GlGraphics::new(opengl),
        board: Board::new(false),
        pieces: vec![],
        scene: Scene::new(),
        window_size,
        square_size: window_size as f32 / 8.0,
        x: 0.0,
        y: 0.0,
        pressed: false,
        click: false,
        release: false,
        active_piece: None,
        face,
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
