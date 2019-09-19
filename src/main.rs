use rustbox::{Color, RustBox, Key};
use clap::value_t;
use std::default::Default;
use std::sync::{Arc, Mutex};
use std::{thread, time};

use piano_rs::arguments;
use piano_rs::game::Note;
use piano_rs::game::play::Player;
/* use piano_rs::game::output; */
use piano_rs::game::{PianoKeyboard, GameEvent};


fn main() {
    let matches = arguments::get_arguments();
    // A workaround to stop cracking noise after note ends (issue #4)
    let blank_point = rodio::get_default_endpoint().unwrap();
    let blank_sink = rodio::Sink::new(&blank_point);
    let blank_source = rodio::source::SineWave::new(0);
    blank_sink.append(blank_source);

    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => Arc::new(Mutex::new(v)),
        Result::Err(e) => panic!("{}", e),
    };

    let volume = value_t!(matches.value_of("volume"), f32).unwrap_or(1.0);
    let mark_duration = value_t!(matches.value_of("markduration"), u64).unwrap_or(500);

    if let Some(playfile) = matches.value_of("play") {
        let replaycolor = matches.value_of("replaycolor").unwrap_or("blue");
        let tempo = value_t!(matches.value_of("tempo"), f32).unwrap_or(1.0);
        /* play::play_from_file(playfile, replaycolor, */
                             /* mark_duration, volume, tempo, &rustbox); */
    }

    let sequence = value_t!(matches.value_of("sequence"), i8).unwrap_or(2);
    let sound_duration = value_t!(matches.value_of("noteduration"), u64).unwrap_or(0);
    let mark_duration = value_t!(matches.value_of("markduration"), u64).unwrap_or(500);
    let record_file = matches.value_of("record");

    let mut keyboard = PianoKeyboard::new(
        sequence,
        volume,
        time::Duration::from_millis(sound_duration),
        time::Duration::from_millis(mark_duration),
        Color::Red,
    );

    keyboard.draw(&rustbox);
    loop {
        let event = rustbox.lock().unwrap().poll_event(false);
        match event {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match keyboard.process_key(key) {
                    Some(GameEvent::Note(note)) => keyboard.play_note(note, &rustbox),
                    Some(GameEvent::Quit) => break,
                    None => { },
                };
            }
            Err(e) => panic!("{}", e),
            _ => { },
        }
    }

    /* let timeout = time::Duration::from_millis(10000); */
    /* thread::sleep(timeout); */
}


#[cfg(test)]
mod tests {
    use std::path::Path;

    #[test]
    fn check_missing_notes() {
        // Find missing notes in assets/*.ogg, if any
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
}
