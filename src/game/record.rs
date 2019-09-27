use std::time::Instant;
use std::path::PathBuf;
use std::fs::{OpenOptions, File};
use std::io::Write;
use crate::game::Note;

#[derive(Debug)]
pub struct NoteRecorder {
    pub record_file: Option<File>,
    pub note_number: usize,
    pub previous_note_time: Instant,
}

impl NoteRecorder {
    pub fn new() -> NoteRecorder {
        NoteRecorder {
            record_file: None,
            note_number: 1,
            previous_note_time: Instant::now(),
        }
    }

    pub fn set_file(&mut self, filename: PathBuf) {
        self.record_file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(filename)
            .ok();
    }

    pub fn write_note(&mut self, note: Note) {
        let delay_gap = self.previous_note_time
            .elapsed()
            .as_millis();
        let current_time = Instant::now();
        self.set_previous_note_time(current_time);

        let note_details = format!(
            "note_{}:\n
            - {}\n
            - {}\n",
            self.note_number,
            note.sound,
            delay_gap,
        );

        if let Err(e) = writeln!(self.record_file.as_ref().unwrap(), "{}", note_details) {
            panic!("Couldn't write note to file: {}", e);
        }

        self.increment_note();
    }

    fn increment_note(&mut self) {
        self.note_number += 1;
    }

    fn set_previous_note_time(&mut self, time: Instant) {
        self.previous_note_time = time;
    }
}
