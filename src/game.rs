/* pub mod notes; */
/* pub mod output; */
/* pub mod play; */

pub mod notes;
pub mod output;
pub mod play;

use rustbox::{Color, RustBox};
use std::sync::{Arc, Mutex};
use std::{thread, time};
pub use crate::game::notes::Note;

#[derive(Debug, Copy, Clone)]
pub struct PianoKeyboard {
}

impl PianoKeyboard {
    pub fn new() -> PianoKeyboard {
        PianoKeyboard { }
    }

    pub fn draw(self, rustbox: &Arc<Mutex<RustBox>>) {
        output::print_whitekeys(rustbox);
        output::print_blackkeys(rustbox);
    }

    pub fn play_note(self, note: Note, color: Color, duration: u32, rustbox: Arc<Mutex<RustBox>>) {
        if note.white {
            rustbox.lock().unwrap().print(note.position as usize, 15, rustbox::RB_BOLD, color, Color::White, "▒▒");
        } else {
            rustbox.lock().unwrap().print(note.position as usize, 8, rustbox::RB_BOLD, color, Color::White, "▒");
        }

        rustbox.lock().unwrap().present();
        let rustbox = rustbox.clone();
        thread::spawn(move || {
            let delay = time::Duration::from_millis(duration.into());
            thread::sleep(delay);
            if note.white {
                rustbox.lock().unwrap().print(note.position as usize, 15, rustbox::RB_BOLD, Color::White, Color::White, "▒▒");
            } else {
                rustbox.lock().unwrap().print(note.position as usize, 8, rustbox::RB_BOLD, Color::Black, Color::White, "▒");
            }
        });
    }

    /* pub fn play_from_file(self, ) { */
    /* } */
}
