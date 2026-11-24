use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::ttf::Font;
use sdl2::video::{Window, WindowContext};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

pub fn render_window(
    count: i32,
    pause: Arc<AtomicBool>,
    canvas: &mut Canvas<Window>,
    texture_creator: &TextureCreator<WindowContext>,
    font: &Font,
) {
    // Window full black
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    // draw the text
    let label = if pause.load(Ordering::Relaxed) {
        format!("count: {count} (paused)")
    } else {
        format!("count: {count}")
    };
    // define font color
    let surface = font
        .render(&label)
        .blended(Color::RGB(255, 255, 255))
        .unwrap();
    // create texture
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .unwrap();

    let q = texture.query();
    canvas
        .copy(&texture, None, Rect::new(20, 20, q.width, q.height))
        .unwrap();
    canvas.present();
}
