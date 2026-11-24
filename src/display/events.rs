use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

pub fn event_loop(event_pump: &mut EventPump, pause: Arc<AtomicBool>) -> bool {
    // Events when buttons pressed
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => {
                // Break loop if game paused
                pause.store(false, Ordering::Relaxed);
                return false;
            }
            Event::KeyDown {
                keycode: Some(Keycode::Space),
                repeat: false,
                ..
            } => {
                // undo pause if quitting when game paused
                pause.fetch_xor(true, Ordering::Relaxed);
            }
            _ => {}
        }
    }
    true
}
