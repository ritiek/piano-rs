pub mod screen;
pub mod notes;
pub mod notes_file;

use rustbox::{Color, Key};
use std::time::Duration;
use std::path::PathBuf;
pub use notes::Note;
pub use notes::Player;
pub use notes_file::{NoteReader, FileNote, NoteRecorder};
use screen::pianokeys;
use serde_derive::{Serialize, Deserialize};
use crossterm::{KeyEvent, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameEvent {
    Note(Note),
    Quit,
}

pub struct PianoKeyboard {
    sequence: i8,
    volume: f32,
    sound_duration: Duration,
    mark_duration: Duration,
    pub color: Color,
    player: Player,
    recorder: NoteRecorder,
}

impl PianoKeyboard {
    pub fn new(sequence: i8, volume: f32, sound_duration: Duration, mark_duration: Duration, color: Color) -> PianoKeyboard {
        let player = Player::new();

        PianoKeyboard {
            sequence: sequence,
            volume: volume,
            sound_duration: sound_duration,
            mark_duration: mark_duration,
            color: color,
            player: player,
            recorder: NoteRecorder::new(),
        }
    }

    pub fn set_record_file(&mut self, record_file: PathBuf) {
        self.recorder.set_file(record_file);
    }

    pub fn draw(&self) -> Result<()> {
        pianokeys::draw()?;
        Ok(())
    }

    pub fn play_note(&mut self, note: Note) {
        note.play(&self.player, self.volume);

        screen::mark_note(
            note.position,
            note.white,
            note.color,
            self.mark_duration,
        );

        if let Some(_) = &self.recorder.record_file {
            self.recorder.write_note(note);
        }
    }

    pub fn set_note_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn process_key(&mut self, key: KeyEvent) -> Option<GameEvent> {
        println!("{:?}", key);
        None
    }

    /* pub fn process_key(&mut self, key: KeyEvent) -> Option<GameEvent> { */
    /*     let note = match key { */
    /*         Key::Right => { */
    /*             if self.sequence < 6 { */
    /*                 self.sequence += 1; */
    /*             } */
    /*             None */
    /*         } */
    /*         Key::Left => { */
    /*             if self.sequence > 0 { */
    /*                 self.sequence -= 1; */
    /*             } */
    /*             None */
    /*         } */
    /*         Key::Up => { */
    /*             // The note sound files are maximum 8s in length */
    /*             if self.sound_duration < Duration::from_millis(8000) { */
    /*                 self.sound_duration += Duration::from_millis(50); */
    /*             } */
    /*             None */
    /*         } */
    /*         Key::Down => { */
    /*             if self.sound_duration > Duration::new(0, 0) { */
    /*                 self.sound_duration -= Duration::from_millis(50); */
    /*             } */
    /*             None */
    /*         } */
    /*         Key::Char('+') => { */
    /*             self.volume += 0.1; */
    /*             None */
    /*         } */
    /*         Key::Char('-') => { */
    /*             self.volume -= 0.1; */
    /*             None */
    /*         } */
    /*         Key::Esc => { */
    /*             Some(GameEvent::Quit) */
    /*         } */
    /*         _ => notes::key_to_base_note(key, self.sequence) */
    /*             .and_then(|note| Note::from(&note, self.color, self.sound_duration)) */
    /*             .map(GameEvent::Note), */
    /*     }; */
    /*     note */
    /* } */

}

#[cfg(test)]
mod test {
    use super::{
        PianoKeyboard,
        Color,
        Key,
        Player,
        Duration,
        GameEvent,
        Note,
        NoteRecorder,
    };

    #[test]
    fn new_pianokeyboard() {
        let actual_keyboard = PianoKeyboard::new(
            2,
            0.4,
            Duration::from_millis(7000),
            Duration::from_millis(500),
            Color::Blue,
        );

        let expected_keyboard = PianoKeyboard {
            sequence: 2,
            volume: 0.4,
            sound_duration: Duration::from_millis(7000),
            mark_duration: Duration::from_millis(500),
            color: Color::Blue,
            player: Player::new(),
            recorder: NoteRecorder::new(),
        };

        assert_eq!(actual_keyboard.sequence, expected_keyboard.sequence);
        assert_eq!(actual_keyboard.volume, expected_keyboard.volume);
        assert_eq!(actual_keyboard.sound_duration, expected_keyboard.sound_duration);
        assert_eq!(actual_keyboard.mark_duration, expected_keyboard.mark_duration);
        assert_eq!(actual_keyboard.color, expected_keyboard.color);
    }

    #[test]
    fn set_note_color() {
        let mut keyboard = PianoKeyboard::new(
            2,
            0.4,
            Duration::from_millis(7000),
            Duration::from_millis(500),
            Color::Blue,
        );
        keyboard.set_note_color(Color::Red);
        assert_eq!(keyboard.color, Color::Red);
    }

    #[test]
    fn process_increase_volume_key() {
        let mut keyboard = PianoKeyboard::new(
            2,
            0.4,
            Duration::from_millis(7000),
            Duration::from_millis(500),
            Color::Blue,
        );

        let event = keyboard.process_key(Key::Char('+'));
        assert!(event.is_none());
        assert_eq!(keyboard.volume, 0.5);
    }

    #[test]
    fn process_decrease_volume_key() {
        let mut keyboard = PianoKeyboard::new(
            2,
            0.4,
            Duration::from_millis(7000),
            Duration::from_millis(500),
            Color::Blue,
        );

        let event = keyboard.process_key(Key::Char('-'));
        assert!(event.is_none());
        assert_eq!(keyboard.volume, 0.3);
    }

    #[test]
    fn process_increase_sequence_key() {
        let mut keyboard = PianoKeyboard::new(
            2,
            0.4,
            Duration::from_millis(7000),
            Duration::from_millis(500),
            Color::Blue,
        );

        let event = keyboard.process_key(Key::Right);
        assert!(event.is_none());
        assert_eq!(keyboard.sequence, 3);
    }

    #[test]
    fn process_decrease_sequence_key() {
        let mut keyboard = PianoKeyboard::new(
            2,
            0.4,
            Duration::from_millis(7000),
            Duration::from_millis(500),
            Color::Blue,
        );

        let event = keyboard.process_key(Key::Left);
        assert!(event.is_none());
        assert_eq!(keyboard.sequence, 1);
    }

    #[test]
    fn process_increase_note_duration_key() {
        let mut keyboard = PianoKeyboard::new(
            2,
            0.4,
            Duration::from_millis(7000),
            Duration::from_millis(500),
            Color::Blue,
        );

        let event = keyboard.process_key(Key::Up);
        assert!(event.is_none());
        assert_eq!(keyboard.sound_duration, Duration::from_millis(7050));
    }

    #[test]
    fn process_decrease_note_duration_key() {
        let mut keyboard = PianoKeyboard::new(
            2,
            0.4,
            Duration::from_millis(7000),
            Duration::from_millis(500),
            Color::Blue,
        );

        let event = keyboard.process_key(Key::Down);
        assert!(event.is_none());
        assert_eq!(keyboard.sound_duration, Duration::from_millis(6950));
    }

    #[test]
    fn process_quit_key() {
        let mut keyboard = PianoKeyboard::new(
            2,
            0.4,
            Duration::from_millis(7000),
            Duration::from_millis(500),
            Color::Blue,
        );

        let event = keyboard.process_key(Key::Esc);
        match event {
            Some(GameEvent::Quit) => assert!(true),
            _ => panic!("This key should have returned a Quit event!"),
        }
    }

    #[test]
    fn process_note_key() {
        let mut keyboard = PianoKeyboard::new(
            2,
            0.4,
            Duration::from_millis(7000),
            Duration::from_millis(500),
            Color::Blue,
        );

        let event = keyboard.process_key(Key::Char('a'));

        let expected_note = Note {
            sound: "gs1".to_string(),
            base: "gs".to_string(),
            frequency: 1,
            position: 42,
            white: false,
            color: Color::Blue,
            duration: Duration::from_millis(7000),
        };

        match event {
            Some(GameEvent::Note(v)) => assert_eq!(v, expected_note),
            _ => panic!("This key should have returned a corresponding Note!"),
        }
    }
}
