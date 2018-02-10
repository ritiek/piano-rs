extern crate rodio;
extern crate std;
extern crate rustbox;

use yaml_rust::{YamlLoader, Yaml};
use rustbox::{RustBox, Key};
use output::draw;
use notes;

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

    fn write_note(&self, note: &notes::Note, duration: u32,
                  file_path: &str, time_diff: time::Duration, n: u32) {
        let diff_in_ms = Self::get_ms(time_diff);
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(file_path)
            .unwrap();
        let note_details = format!("note_{}:\n  - {}\n  - {}\n  - {}\n  - {}\n  - {}\n  - {}\n",
                                   n, note.sound, note.sequence, duration, diff_in_ms, note.position, note.white);

        if let Err(e) = writeln!(file, "{}", note_details) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }

    fn get_ms(time_diff: time::Duration) -> u64 {
        let nanos = u64::from(time_diff.subsec_nanos());
		(1000*1000*1000 * time_diff.as_secs() + nanos)/(1000 * 1000)
    }
}


pub fn play_from_keyboard(rb: &Arc<Mutex<RustBox>>, color: &str, mark_duration: u32,
                          note_dur: u32, raw_seq: i16, volume: f32, record_file: Option<&str>) {
    let mut note_duration = note_dur;
    let mut raw_sequence = raw_seq;
    let player = Player::new();

    let mut note_number = 1;
    let mut note_volume = volume;
    let mut now = time::Instant::now();

    loop {
        let pe = rb.lock().unwrap().poll_event(false);
        let rb = rb.clone();
        match pe {
            Ok(rustbox::Event::KeyEvent(key)) => {
                let note = notes::match_note(key, raw_sequence);
                if note.position > 0 && note.position < 155 {
                    if let Some(r) = record_file {
                        player.write_note(&note, note_duration, r, now.elapsed(), note_number);
                    }
                    player.play(&note.sound, note.sequence, note_duration, note_volume);
                    draw(note.position, note.white, color, mark_duration, rb);
                    note_number += 1;
                    now = time::Instant::now();
                }
                match key {
                    Key::Right => {
                        if raw_sequence < 5 {
                            raw_sequence += 1;
                        }
                    }
                    Key::Left => {
                        if raw_sequence > 0 {
                            raw_sequence -= 1;
                        }
                    }
                    Key::Up => {
                        if note_duration < 8000 {
                            note_duration += 50;
                        }
                    }
                    Key::Down => {
                        if note_duration > 0 {
                            note_duration -= 50;
                        }
                    }
                    Key::Char('+') => {
                        note_volume += 0.1;
                    }
                    Key::Char('-') => {
                        note_volume -= 0.1;
                    }
                    Key::Esc => {
                        break;
                    }
                    _ => {}
                }
            }
            Err(e) => panic!("{}", e),
            _ => {}
        }
    }
}


pub fn play_from_file(filename: &str, color: &str, mark_duration: u32,
                      volume: f32, tempo: f32, rustbox: &Arc<Mutex<RustBox>>) {
    let mut file = File::open(filename).expect("Unable to open the file");
    let mut s = String::new();
    file.read_to_string(&mut s).expect("Unable to read the file");

    let docs = YamlLoader::load_from_str(&s).unwrap();
    let doc = &docs[0];
    let mut note_num = 1;
    let player = Player::new();

    loop {
        let rustbox = rustbox.clone();
        let note = format!("note_{}", note_num);

        let note_ops = &match doc[note.as_str()] {
            Yaml::Array(ref x) => x,
            _ => break,
        };

        let duration = time::Duration::from_millis((note_ops[3].as_i64().unwrap() as f32 / tempo) as u64);
        thread::sleep(duration);
        player.play(note_ops[0].as_str().unwrap(),
                    note_ops[1].as_i64().unwrap() as i16,
                    note_ops[2].as_i64().unwrap() as u32,
                    volume);
        draw(note_ops[4].as_i64().unwrap() as i16, note_ops[5].as_bool().unwrap(), color, mark_duration, rustbox);
        note_num += 1;
    }
}

