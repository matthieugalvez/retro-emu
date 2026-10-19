extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Receiver;
use std::sync::Arc;
use std::time::Duration;

pub fn create_window(rx: Receiver<i32>, pause: Arc<AtomicBool>) {
    // Manage everything that can happen within the program
    let sdl_context = sdl2::init().unwrap();
    // Group everything video-related
    let video_subsystem = sdl_context.video().unwrap();
    // Initialise font API
    let ttf_context = sdl2::ttf::init().unwrap();
    // Load font
    let font = ttf_context
        .load_font("/usr/share/fonts/truetype/noto/NotoSans-Regular.ttf", 32)
        .unwrap();
    // Spawn the main window
    let window = video_subsystem
        .window("RetroEmu", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    // Manage and own a target (surface or window) and allow drawing in it
    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut count = 0;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    pause.store(false, Ordering::Relaxed);
                    break 'running;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    repeat: false,
                    ..
                } => {
                    pause.fetch_xor(true, Ordering::Relaxed); // toggle
                }
                _ => {}
            }
        }
        while let Ok(val) = rx.try_recv() {
            count = val;
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        // draw the text
        let label = if pause.load(Ordering::Relaxed) {
            format!("count: {count} (paused)")
        } else {
            format!("count: {count}")
        };
        let surface = font
            .render(&label)
            .blended(Color::RGB(255, 255, 255))
            .unwrap();

        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .unwrap();

        let q = texture.query();
        canvas
            .copy(&texture, None, Rect::new(20, 20, q.width, q.height))
            .unwrap();
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
