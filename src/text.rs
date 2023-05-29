use rusttype::{Font, point, PositionedGlyph, Scale};
use sdl2::rect::{Point as SDLPoint, Point};


use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;

pub(crate) fn render_score(canvas: &mut Canvas<Window>, font: &Font, score: u32, font_size: f32, color: Color) {

    canvas.set_draw_color(color);
    let scale = Scale::uniform(font_size);
    let v_metrics = font.v_metrics(scale);
    let offset = point(400.0, v_metrics.ascent);

    let glyphs: Vec<_> = font.layout(&score.to_string(), scale, offset)
        .collect();

    let mut x_offset = 0.0;

    for glyph in glyphs {
        if let Some(bb) = glyph.pixel_bounding_box() {
            glyph.draw(|x, y, v| {
                let x_pos = (x as f32 + bb.min.x as f32 + x_offset) as i32;
                let y_pos = (y as f32 + bb.min.y as f32 + v_metrics.ascent) as i32;

                canvas.draw_point(Point::new(x_pos, y_pos)).unwrap();
            });
        }

        x_offset += glyph.unpositioned().h_metrics().advance_width;
    }
}
pub(crate) fn load_font() -> Result<Font<'static>, String> {
    let font_data = include_bytes!(r"C:\shitGame\src\assets\fonts\Roboto-Regular.ttf");
    Font::try_from_bytes(font_data)
        .ok_or_else(|| String::from("Error constructing Font"))
}
