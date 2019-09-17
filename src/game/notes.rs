use std::ascii::AsciiExt;
use std::num::ParseIntError;
use std::convert::Infallible;
use rodio::Endpoint;
use serde_derive::{Serialize, Deserialize};
use rustbox::Key;


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Note {
    pub sound: String,
    base: String,
    frequency: i8,
    pub position: i8,
    pub white: bool,
}

/* #[derive(Debug, PartialEq, Serialize, Deserialize)] */
/* pub struct Note { */
/*     pub sound: String, */
/*     pub frequency: i8, */
/* } */

impl Note {
    pub fn from(sound: &str) -> Option<Note> {
        let (base_sound, frequency) = Self::extract_base_sound_and_frequency(sound);
        /* let position = Self::position(); */
        match frequency {
            Ok(v) => {
                Some(Note {
                    sound: sound.to_string(),
                    base: base_sound.unwrap(),
                    frequency: v,
                    position: 10,
                    white: true,
                })
            },
            Err(e) => None,
        }
    }

    fn extract_base_sound_and_frequency(note: &str) -> (Result<String, Infallible>, Result<i8, ParseIntError>) {
        let mut frequency: Result<i8, ParseIntError> = note.parse();
        let mut base_sound: Result<String, Infallible> = note.parse();
        for start_index in 1..note.len() {
            frequency = note[start_index..].parse();
            base_sound = note[..start_index].parse();
            if let Ok(v) = frequency {
                break;
            }
        }
        (base_sound, frequency)
    }

    pub fn play(endponit: Endpoint) {
    }
}

pub fn match_note(mut key: Key, mut raw_seq: i8) -> Option<Note> {
    let mut sound = String::new();
    let mut white: bool;
    let mut factor: i8;
    let mut position: i8;
    let mut frequency: i8;

    let keys = ['z', 's', 'x', 'c', 'f', 'v', 'g', 'b', 'n',
                'j', 'm', 'k', '1', ',', 'q', 'l', '2', '.',
                'w', '/', 'e', '\'', '4', 'r', '5', 't', 'y',
                '7', 'u', '8', 'i', '9', 'o', 'p', '[', ']', 'a'];

    let notes = ["a", "as", "b", "c", "cs", "d", "ds", "e", "f",
                 "fs", "g", "gs", "gs", "a", "a", "as", "as", "b",
                 "b", "c", "c", "cs", "cs", "d", "ds", "e", "f",
                 "fs", "g", "gs", "a", "as", "b", "c", "d", "e", "gs"];

    let init_poses = [1, 3, 4, 7, 9, 10, 12, 13, 16,
                      18, 19, 21, 21, 22, 22, 24, 24, 25,
                      25, 28, 28, 30, 30, 31, 33, 34, 37,
                      39, 40, 42, 43, 45, 46, 49, 52, 55, 0];

    let whites = [true, false, true, true, false, true, false, true, true,
                  false, true, false, false, true, true, false, false, true,
                  true, true, true, false, false, true, false, true, true,
                  false, true, false, true, false, true, true, true, true, false];

    let factors = [-1, -1, -1, 0, 0, 0, 0, 0, 0,
                   0, 0, 0, 0, 0, 0, 0, 0, 0,
                   0, 1, 1, 1, 1, 1, 1, 1, 1,
                   1, 1, 1, 1, 1, 1, 2, 2, 2, -1];

    let special = ['!', '@', '$', '%', '&', '*', '(', '"', '<',
                   '>', '?', '{', '}'];

    let special_matches = ['1', '2', '4', '5', '7', '8', '9', '\'', ',',
                           '.', '/', '[', ']'];


    // Handle terminal control characters
    if key == Key::Enter {
        // Ctrl+m sends Enter in terminal
        key = Key::Ctrl('m');
    } else if key == Key::Tab {
        // Ctrl+i sends Tab in terminal
        key = Key::Ctrl('i');
    }

    // Translate Ctrl+<character> to <character>
    if let Key::Ctrl(c) = key {
        key = Key::Char(c);
        raw_seq -= 1;
    }

    // Increment `raw_seq` if key was shift prefixed (Shift+<character>)
    let note: Option<Note> = if let Key::Char(mut c) = key {
        if c.is_uppercase() {
            c = c.to_ascii_lowercase();
            raw_seq += 1;
        } else if special.contains(&c) {
            let j = special.iter()
                           .position(|&key| key == c)
                           .unwrap();
            c = special_matches[j];
            raw_seq += 1;
        }

        if let Some(i) = keys.iter().position(|&key| key == c) {
            let init_pos = init_poses[i];
            let factor = factors[i];

            let note_sound = format!("{}{}", notes[i].to_string(), raw_seq + factor);

            Note::from(&note_sound)
            /* Some(Note { */
            /*     sound: note_sound, */
            /*     /1* base: notes[i].to_string(), *1/ */
            /*     /1* frequency: raw_seq + factor, *1/ */
            /*     position: init_pos + 21 * raw_seq, */
            /*     white: whites[i], */
            /* }) */
        } else {
            None
        }
    } else {
        None
    };

    note
}


#[cfg(test)]
mod tests {
    use super::{
        Note,
        match_note,
        Key
    };

    #[test]
    fn check_note_attributes() {
        // Check attributes for random note
        let note = match_note(Key::Char('q'), 2);
        let expect_note = Note {
                              base: "a".to_string(),
                              frequency: 2,
                              position: 64,
                              white: true
                           };

        assert_eq!(note, expect_note);
    }
}
