mod notes;

extern crate clap;
extern crate rodio;
extern crate rustbox;
extern crate yaml_rust;

use std::default::Default;
use std::io::{BufReader, Read, Cursor};
use std::sync::{Arc, Mutex};
use std::{thread, time};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::fs::File;
use std::io::prelude::*;
use yaml_rust::{YamlLoader, YamlEmitter, Yaml};

use clap::{Arg, App};

use rustbox::{Color, RustBox};
use rustbox::Key;

/*
█▒
*/

struct Player {
    endpoint: rodio::Endpoint,
    samples: HashMap<String, Vec<u8>>,
}

impl Player {
    pub fn new() -> Player {
        let endpoint: rodio::Endpoint = rodio::get_default_endpoint().unwrap();
        let mut samples = HashMap::new();

        for note in ["a", "as", "b", "c", "cs", "d", "ds", "e", "f", "fs", "g", "gs"].iter() {
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

    fn play(&self, note: &str, sequence: i16, duration: u32) {
        self.get(note, sequence)
            .map(|note| {
                let sink = rodio::play_once(&self.endpoint, note).expect("Cannot play");
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

    fn write_note(&self, note: &str, sequence: i16, duration: u32, position: i16, white: bool,
                  file_path: &str, time_diff: time::Duration, n: u32) {
        let diff_in_ms = Self::get_ms(time_diff);
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(file_path)
            .unwrap();
        let note_details = format!("note_{}:\n  - {}\n  - {}\n  - {}\n  - {}\n  - {}\n  - {}\n",
                                   n, note, sequence, duration, diff_in_ms, position, white);

        if let Err(e) = writeln!(file, "{}", note_details) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }

    fn get_ms(time_diff: time::Duration) -> u64 {
        let nanos = time_diff.subsec_nanos() as u64;
		(1000*1000*1000 * time_diff.as_secs() + nanos)/(1000 * 1000)
    }
}


fn print_whitekeys(rustbox: &Arc<Mutex<RustBox>>) {
    for y in 0..16 {
        // last border is lonely
        rustbox.lock().unwrap().print(156, y, rustbox::RB_BOLD, Color::Black, Color::White, "|");
        for x in 0..52 {
            let k = x * 3;
            rustbox.lock().unwrap().print(k, y, rustbox::RB_BOLD, Color::Black, Color::White, "|");
            rustbox.lock().unwrap().print(k + 1, y, rustbox::RB_BOLD, Color::White, Color::Black, "██");
        }
    }
}


fn print_blackkeys(rustbox: &Arc<Mutex<RustBox>>) {
    for y in 0..9 {
        // first black key is lonely
        rustbox.lock().unwrap().print(3, y, rustbox::RB_BOLD, Color::Black, Color::White, "█");

        for x in 0..7 {
            let g1k1 = x * 21 + 9;
            let g1k2 = g1k1 + 3;
            rustbox.lock().unwrap().print(g1k1, y, rustbox::RB_BOLD, Color::Black, Color::White, "█");
            rustbox.lock().unwrap().print(g1k2, y, rustbox::RB_BOLD, Color::Black, Color::White, "█");

            let g2k1 = g1k2 + 6;
            let g2k2 = g2k1 + 3;
            let g2k3 = g2k2 + 3;
            rustbox.lock().unwrap().print(g2k1, y, rustbox::RB_BOLD, Color::Black, Color::White, "█");
            rustbox.lock().unwrap().print(g2k2, y, rustbox::RB_BOLD, Color::Black, Color::White, "█");
            rustbox.lock().unwrap().print(g2k3, y, rustbox::RB_BOLD, Color::Black, Color::White, "█");
        }
    }
}


fn draw(pos: i16, white: bool, color: &str, duration: u32, rustbox: Arc<Mutex<RustBox>>) {
    let rb_colors = [
        Color::Black,
        Color::Red,
        Color::Green,
        Color::Yellow,
        Color::Blue,
        Color::Magenta,
        Color::Cyan,
        Color::White
    ];

    let colors = [
        "black",
        "red",
        "green",
        "yellow",
        "blue",
        "magenta",
        "cyan",
        "white"
    ];

    let color_pos = colors.iter().position(|&c| c == color).unwrap();

    if white {
        rustbox.lock().unwrap().print(pos as usize, 15, rustbox::RB_BOLD, rb_colors[color_pos], Color::White, "▒▒");
    } else {
        rustbox.lock().unwrap().print(pos as usize, 8, rustbox::RB_BOLD, rb_colors[color_pos], Color::White, "▒");
    }

    rustbox.lock().unwrap().present();
    thread::spawn(move || {
        let delay = time::Duration::from_millis(duration.into());
        thread::sleep(delay);
        if white {
            rustbox.lock().unwrap().print(pos as usize, 15, rustbox::RB_BOLD, Color::White, Color::White, "▒▒");
        } else {
            rustbox.lock().unwrap().print(pos as usize, 8, rustbox::RB_BOLD, Color::Black, Color::White, "▒");
        }
    });
}


fn play_from_file(filename: &str, color: &str, mark_duration: u32,
                  rustbox: Arc<Mutex<RustBox>>) {
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
        let note_ops = match &doc[note.as_str()] {
            &Yaml::Array(ref x) => x,
            _ => break,
        };
        let duration = time::Duration::from_millis(note_ops[3].as_i64().unwrap() as u64);
        thread::sleep(duration);
        player.play(note_ops[0].as_str().unwrap(),
                    note_ops[1].as_i64().unwrap() as i16,
                    note_ops[2].as_i64().unwrap() as u32);
        draw(note_ops[4].as_i64().unwrap() as i16, note_ops[5].as_bool().unwrap(), color, mark_duration, rustbox);
        note_num += 1;
    }
}


fn main() {
    let matches = App::new("piano-rs")
        .version("0.1.0")
        .author("Ritiek Malhotra <ritiekmalhotra123@gmail.com>")
        .about("Play piano in the terminal using PC keyboard.")

        .arg(Arg::with_name("record")
            .short("r")
            .long("recordfile")
            .value_name("FILEPATH")
            .takes_value(true)
            .help("Record notes to .yml file (Default: none)"))

        .arg(Arg::with_name("play")
            .short("p")
            .long("playfile")
            .value_name("FILEPATH")
            .takes_value(true)
            .help("Play notes from .yml file (Default: none)"))

        .arg(Arg::with_name("color")
            .short("c")
            .long("color")
            .value_name("COLOR")
            .takes_value(true)
            .help("Color of block to generate when a note is played (Default: \"red\")"))

        .arg(Arg::with_name("replaycolor")
            .short("x")
            .long("replaycolor")
            .value_name("COLOR")
            .takes_value(true)
            .help("Color of block to generate when notes are played from file (Default: \"blue\")"))

        .arg(Arg::with_name("sequence")
            .short("s")
            .long("sequence")
            .value_name("SEQUENCE")
            .takes_value(true)
            .help("Frequency sequence from 0 to 5 to begin with (Default: 2)"))

        .arg(Arg::with_name("noteduration")
            .short("n")
            .long("note-duration")
            .value_name("DURATION")
            .takes_value(true)
            .help("Duration to play each note for, where 0 means till the end of note (Default: 0)"))

        .arg(Arg::with_name("markduration")
            .short("m")
            .long("mark-duration")
            .value_name("DURATION")
            .takes_value(true)
            .help("Duration to show piano mark for, in ms (Default: 500)"))

        .get_matches();

    // A workaround to stop cracking noise after note ends (#4)
    let blank_point = rodio::get_default_endpoint().unwrap();
    let blank_sink = rodio::Sink::new(&blank_point);
    let blank_source = rodio::source::SineWave::new(0);
    blank_sink.append(blank_source);

    let rb = match RustBox::init(Default::default()) {
        Result::Ok(v) => Arc::new(Mutex::new(v)),
        Result::Err(e) => panic!("{}", e),
    };

    let player = Player::new();

    print_whitekeys(&rb);
    print_blackkeys(&rb);
    let mut raw_sequence: i16 = matches.value_of("sequence").unwrap_or("2").parse().unwrap();
    let mut note_duration: u32 = matches.value_of("noteduration").unwrap_or("0").parse().unwrap();
    let mark_duration: u32 = matches.value_of("markduration").unwrap_or("500").parse().unwrap();
    let color = matches.value_of("color").unwrap_or("red");
    let replaycolor = matches.value_of("replaycolor").unwrap_or("blue");
    rb.lock().unwrap().present();
    let mut now = time::Instant::now();
    let mut note_number = 1;

    if matches.is_present("play") {
        let playfile = matches.value_of("play").unwrap();
        play_from_file(playfile, replaycolor, mark_duration, rb.clone());
    } else {
        // TODO: put below code into a function or something
        loop {
            let pe = rb.lock().unwrap().poll_event(false);
            let rb = rb.clone();
            match pe {
                Ok(rustbox::Event::KeyEvent(key)) => {
                    let note = notes::match_note(key, raw_sequence);
                    if note.position > 0 && note.position < 155 {
                        if matches.is_present("record") {
                            let record_file = matches.value_of("record").unwrap();
                            player.write_note(&note.sound, note.sequence, note_duration,
                                              note.position, note.white, record_file,
                                              now.elapsed(), note_number);
                        }
                        player.play(&note.sound, note.sequence, note_duration);
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
}


#[cfg(test)]
mod tests {
    use super::{notes, Key};
    use std::path::Path;

    #[test]
    fn check_missing_notes() {
        // find missing notes in assets/*.ogg, if any
        let mut missing_notes = Vec::new();
        let expected_notes = ["a", "as", "b", "c", "cs", "d", "ds", "e", "f", "fs", "g", "gs"];
        for expected_note in expected_notes.iter() {
            if expected_note == &"a" || expected_note == &"as" {
                let note = format!("{}-1.ogg", expected_note);
				let note_path = format!("assets/{}", note);
                if !Path::new(&note_path).exists() {
                    missing_notes.push(note);
                }
            }
			for sequence in 0..8_u16 {
				let note = format!("{}{}.ogg", expected_note, sequence);
				let note_path = format!("assets/{}", note);
                if !Path::new(&note_path).exists() {
                    missing_notes.push(note);
                }
            }
        }

        assert!(missing_notes.len() == 0,
                "Some note sounds are missing: {}", missing_notes.join(", "));
    }

    #[test]
    fn check_note_attributes() {
        // check attributes for random note
        let note = notes::match_note(Key::Char('q'), 2);
        let expect_note = notes::Note {
                                sound: "a".to_string(),
                                sequence: 2,
                                position: 64,
                                white: true
                           };

        assert_eq!(note, expect_note);
    }
}
