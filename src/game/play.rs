use yaml_rust::{YamlLoader, Yaml};
use rustbox::{RustBox, Key};

use std::io::{BufReader, Read, Cursor};
use std::sync::{Arc, Mutex};
use std::{thread, time};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::fs::File;
use std::io::prelude::*;

struct Player {
    endpoint: rodio::Endpoint,
    samples: HashMap<String, Vec<u8>>,
}

impl Player {
    fn new() -> Player {
        let endpoint = rodio::get_default_endpoint().unwrap();
        let mut samples = HashMap::new();

        for note in &["a", "as", "b", "c", "cs", "d", "ds", "e", "f", "fs", "g", "gs"] {
            for sequence in -1..8_i16 {
                Self::read_note(*note, sequence)
                    .and_then(|sample| {
                        samples.insert(format!("{}{}", note, sequence), sample);
                        Some(())
                    });
            }
        }

        Player {
            endpoint,
            samples
        }
    }

    fn get(&self, note: &str, sequence: i16) -> Option<BufReader<Cursor<Vec<u8>>>> {
        self.samples.get(&format!("{}{}", note, sequence))
            .map(|v| BufReader::new(Cursor::new(v.clone())))
    }

    fn play(&self, note: &str, sequence: i16, duration: u32, volume: f32) {
        self.get(note, sequence)
            .map(|note| {
                let mut sink = rodio::play_once(&self.endpoint, note).expect("Cannot play");
                sink.set_volume(volume);
                if duration == 0 {
                    sink.detach();
                } else {
                    thread::spawn(move || {
                        let delay = time::Duration::from_millis(duration.into());
                        thread::sleep(delay);
                        sink.stop();
                    });
                }

                true
            });
    }

    fn read_note(note: &str, sequence: i16) -> Option<Vec<u8>> {
        let file_path = format!("assets/{0}{1}.ogg", note, sequence);
        std::fs::File::open(file_path)
            .map(|mut file| {
                let mut data = Vec::new();
                file.read_to_end(&mut data).unwrap();
                data
            }).ok()
    }

    /* fn write_note(&self, note: &notes::Note, duration: u32, */
    /*               file_path: &str, time_diff: time::Duration, n: u32) { */
    /*     let diff_in_ms = Self::get_ms(time_diff); */
    /*     let mut file = OpenOptions::new() */
    /*         .write(true) */
    /*         .create(true) */
    /*         .append(true) */
    /*         .open(file_path) */
    /*         .unwrap(); */
    /*     let note_details = format!("note_{}:\n  - {}\n  - {}\n  - {}\n  - {}\n  - {}\n  - {}\n", */
    /*                                n, note.sound, note.sequence, duration, diff_in_ms, note.position, note.white); */

        /* if let Err(e) = writeln!(file, "{}", note_details) { */
        /*     eprintln!("Couldn't write to file: {}", e); */
        /* } */
    /* } */

    /* fn get_ms(time_diff: time::Duration) -> u64 { */
    /*     let nanos = u64::from(time_diff.subsec_nanos()); */
		/* (1000*1000*1000 * time_diff.as_secs() + nanos)/(1000 * 1000) */
    /* } */
}
