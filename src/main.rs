#[macro_use]
extern crate clap;
extern crate rodio;
extern crate rustbox;
extern crate yaml_rust;

use rustbox::RustBox;
use std::default::Default;
use std::sync::{Arc, Mutex};

mod arguments;
mod notes;
mod play;
mod output;


fn main() {
    let matches = arguments::get_arguments();
    // A workaround to stop cracking noise after note ends (issue #4)
    let blank_point = rodio::get_default_endpoint().unwrap();
    let blank_sink = rodio::Sink::new(&blank_point);
    let blank_source = rodio::source::SineWave::new(0);
    blank_sink.append(blank_source);

    let rb = match RustBox::init(Default::default()) {
        Result::Ok(v) => Arc::new(Mutex::new(v)),
        Result::Err(e) => panic!("{}", e),
    };

    output::display_keyboard(&rb);
    let volume = value_t!(matches.value_of("volume"), f32).unwrap_or(1.0);
    let mark_duration = value_t!(matches.value_of("markduration"), u32).unwrap_or(500);

    if let Some(playfile) = matches.value_of("play") {
        let replaycolor = matches.value_of("replaycolor").unwrap_or("blue");
        let tempo = value_t!(matches.value_of("tempo"), f32).unwrap_or(1.0);
        play::play_from_file(playfile, replaycolor,
                             mark_duration, volume, tempo, &rb);
    }

    let raw_sequence = value_t!(matches.value_of("sequence"), i16).unwrap_or(2);
    let note_duration = value_t!(matches.value_of("noteduration"), u32).unwrap_or(0);
    let record_file = matches.value_of("record");
    let color = matches.value_of("color").unwrap_or("red");
    play::play_from_keyboard(&rb, color, mark_duration, note_duration,
                             raw_sequence, volume, record_file);
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
