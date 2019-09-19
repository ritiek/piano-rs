/* pub mod notes; */
/* pub mod output; */
/* pub mod play; */

pub mod output;
pub mod play;
pub mod notes;

use rustbox::{Color, RustBox, Key};
use std::sync::{Arc, Mutex};
use std::{thread, time};
pub use notes::Note;
pub use play::Player;

#[derive(Debug)]
pub enum GameEvent {
    Note(Note),
    Quit,
}

#[derive(Clone)]
pub struct PianoKeyboard {
    sequence: i8,
    volume: f32,
    sound_duration: time::Duration,
    mark_duration: time::Duration,
    color: Color,
    player: Player,
}

impl PianoKeyboard {
    pub fn new(sequence: i8, volume: f32, sound_duration: time::Duration, mark_duration: time::Duration, color: Color) -> PianoKeyboard {
        let player = Player::new();
        PianoKeyboard {
            sequence: sequence,
            volume: volume,
            sound_duration: sound_duration,
            mark_duration: mark_duration,
            color: color,
            player: player
        }
    }

    pub fn draw(&self, rustbox: &Arc<Mutex<RustBox>>) {
        output::print_whitekeys(rustbox);
        output::print_blackkeys(rustbox);
    }

    pub fn play_note(&self, note: Note, rustbox: &Arc<Mutex<RustBox>>) {
        self.player.play(
            &note.base,
            note.frequency,
            self.sound_duration,
            self.volume,
        );

        output::mark_note(
            note.position,
            note.white,
            note.color,
            self.mark_duration,
            &rustbox,
        );

        /* note.draw(); */
        /* note.play(); */
    }

    pub fn process_key(&mut self, key: Key) -> Option<GameEvent> {
        let note = match key {
            Key::Right => {
                if self.sequence < 5 {
                    self.sequence += 1;
                }
                None
            }
            Key::Left => {
                if self.sequence > 0 {
                    self.sequence -= 1;
                }
                None
            }
            Key::Up => {
                if self.sound_duration < time::Duration::from_millis(8000) {
                    self.sound_duration += time::Duration::from_millis(50);
                }
                None
            }
            Key::Down => {
                if self.sound_duration > time::Duration::from_millis(0) {
                    self.sound_duration -= time::Duration::from_millis(50);
                }
                None
            }
            Key::Char('+') => {
                self.volume += 0.1;
                None
            }
            Key::Char('-') => {
                self.volume -= 0.1;
                None
            }
            Key::Esc => {
                Some(GameEvent::Quit)
            }
            _ => {
                let base_note = notes::key_to_base_note(key, self.sequence);

                let note = match base_note {
                    Some(v) => Note::from(&v, self.color, self.sound_duration),
                    None => None,
                };

                match note {
                    Some(v) => Some(GameEvent::Note(v)),
                    None => None,
                }
            }
        };
        note
    }
}
