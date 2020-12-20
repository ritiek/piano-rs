use std::time::{Instant, Duration};
use std::path::PathBuf;
use std::fs::{OpenOptions, File};
use std::io::Write;
use yaml_rust::{YamlLoader, Yaml};
use crate::game::Note;
use std::io::{Error, Read};

#[derive(Debug)]
pub struct FileNote {
    pub delay: Duration,
    pub base_note: String,
    pub duration: Duration,
}

#[derive(Debug)]
pub struct NoteReader {
    play_file: File,
    yaml: Vec<Yaml>,
}

impl NoteReader {
    pub fn from(filename: PathBuf) -> NoteReader {
        let mut file = File::open(filename).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        let yaml_content = YamlLoader::load_from_str(&content).unwrap();

        NoteReader {
            play_file: file,
            yaml: yaml_content,
        }
    }

    pub fn parse_notes(&self) -> Vec<FileNote> {
        let mut counter = 1;
        let mut file_base_notes: Vec<FileNote> = Vec::new();
        while let Ok(v) = self.parse_yaml_entry(counter) {
            file_base_notes.push(v);
            counter += 1;
        }
        file_base_notes
    }

    pub fn parse_yaml_entry(&self, id: u32) -> Result<FileNote, String> {
        let note_id = format!("note_{}", id);

        let future_base_note = match self.yaml[0][note_id.as_str()] {
            Yaml::Array(ref x) => {
                let delay = Duration::from_millis(x[0].as_i64().unwrap() as u64);
                let base_note = x[1].as_str().unwrap();
                let duration = Duration::from_millis(x[2].as_i64().unwrap() as u64);
                Ok(FileNote {
                    delay,
                    base_note: base_note.to_string(),
                    duration,
                })
            },
            _ => Err(String::from("Could not parse note")),
        };

        future_base_note
    }
}

#[derive(Debug)]
pub struct NoteRecorder {
    pub record_file: Option<File>,
    note_number: usize,
    previous_note_time: Instant,
}

impl NoteRecorder {
    pub fn new() -> NoteRecorder {
        Self::from(PathBuf::new())
    }

    pub fn from(filename: PathBuf) -> NoteRecorder {
        NoteRecorder {
            record_file: Self::open_file(filename).ok(),
            note_number: 1,
            previous_note_time: Instant::now(),
        }
    }

    pub fn set_file(&mut self, filename: PathBuf) {
        self.record_file = Self::open_file(filename).ok();
    }

    pub fn write_note(&mut self, note: Note) {
        let delay_gap = self.previous_note_time
            .elapsed()
            .as_millis();
        let current_time = Instant::now();
        self.set_previous_note_time(current_time);

        let note_details = format!(
            "note_{}:\n  - {}\n  - {}\n  - {}\n",
            self.note_number,
            delay_gap,
            note.sound,
            note.duration.as_millis(),
        );

        if let Err(e) = writeln!(self.record_file.as_ref().unwrap(), "{}", note_details) {
            panic!("Couldn't write note to file: {}", e);
        }

        self.increment_note();
    }

    fn open_file(filename: PathBuf) -> Result<File, Error> {
        OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(filename)
    }

    fn increment_note(&mut self) {
        self.note_number += 1;
    }

    fn set_previous_note_time(&mut self, time: Instant) {
        self.previous_note_time = time;
    }
}

impl Default for NoteRecorder {
    fn default() -> Self {
        Self::new()
    }
}

