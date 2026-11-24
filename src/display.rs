use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::mpsc::Receiver;
use std::time::Duration;
mod events;
mod font;
mod render;

pub fn create_window(rx: Receiver<i32>, pause: Arc<AtomicBool>) {
    // Manage everything that can happen within the program
    let sdl_context = sdl2::init().unwrap();
    // Group everything video-related
    let video_subsystem = sdl_context.video().unwrap();
    // Initialise font API
    let ttf_context = sdl2::ttf::init().unwrap();
    // Load font
    let font = font::load_font(&ttf_context, 32).expect("failed to load font");
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
    // Game loop
    'running: loop {
        // Events when buttons pressed
        if !events::event_loop(&mut event_pump, pause.clone()) {
            break 'running;
        }
        // receive values from main thread
        while let Ok(val) = rx.try_recv() {
            count = val;
        }
        render::render_window(count, pause.clone(), &mut canvas, &texture_creator, &font);
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
