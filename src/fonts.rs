extern crate freetype as ft;
use graphics::{Context, Graphics, ImageSize};
use opengl_graphics::{Texture, TextureSettings};

pub(crate) fn glyphs(face: &mut ft::Face, text: String) -> Vec<(Texture, [f64; 2])> {
    let mut x = 0;
    let mut y = 0;
    let mut res = vec![];
    for ch in text.chars() {
        face.load_char(ch as usize, ft::face::LoadFlag::RENDER)
            .unwrap();
        let g = face.glyph();

        let bitmap = g.bitmap();
        let texture = Texture::from_memory_alpha(
            bitmap.buffer(),
            bitmap.width() as u32,
            bitmap.rows() as u32,
            &TextureSettings::new(),
        )
        .unwrap();
        res.push((
            texture,
            [(x + g.bitmap_left()) as f64, (y - g.bitmap_top()) as f64],
        ));

        x += (g.advance().x >> 6) as i32;
        y += (g.advance().y >> 6) as i32;
    }
    res
}

pub(crate) fn render_text<G, T>(glyphs: &[(T, [f64; 2])], c: &Context, gl: &mut G)
where
    G: Graphics<Texture = T>,
    T: ImageSize,
{
    for &(ref texture, [x, y]) in glyphs {
        use graphics::*;
        Image::new_color(color::WHITE).draw(texture, &c.draw_state, c.transform.trans(x, y), gl);
    }
}
