use std::ascii::AsciiExt;
use std::num::ParseIntError;
use std::convert::Infallible;
use rodio::Endpoint;
use serde_derive::{Serialize, Deserialize};
use serde::ser::{Serialize, Serializer, SerializeStruct};
use rustbox::{Key, Color};
use std::time::Duration;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "Color")]
pub enum ColorDef {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Byte(u16),
    Default,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Note {
    pub sound: String,
    base: String,
    frequency: i8,
    pub position: i8,
    pub white: bool,
    #[serde(with = "ColorDef")]
    pub color: Color,
    pub duration: Duration,
}

impl Note {
    pub fn from(sound: &str, color: Color, duration: Duration) -> Option<Note> {
        match Self::extract_base_sound_and_frequency(sound) {
            (Ok(base_sound), Ok(frequency)) => Some(Self::parse_note(
                &base_sound,
                frequency,
                color,
                duration,
            )),
            _ => None,
        }
    }

    fn extract_base_sound_and_frequency(note: &str) -> (Result<String, Infallible>, Result<i8, ParseIntError>) {
        let mut base_sound: Result<String, Infallible> = note.parse();
        let mut frequency: Result<i8, ParseIntError> = note.parse();
        for start_index in 1..note.len() {
            frequency = note[start_index..].parse();
            base_sound = note[..start_index].parse();
            if let Ok(v) = frequency {
                break;
            }
        }
        (base_sound, frequency)
    }

    fn parse_note(base_sound: &str, frequency: i8, color: Color, duration: Duration) -> Note {
        let base_sounds = ["a", "as", "b", "c", "cs", "d", "ds", "e", "f",
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

        let index = base_sounds.iter()
                       .position(|&key| key == base_sound)
                       .unwrap();

        Note {
            sound: format!("{}{}", base_sound, frequency),
            base: base_sounds[index].to_string(),
            frequency: frequency,
            position: init_poses[index] + 21 * (frequency + 1),
            white: whites[index],
            color: color,
            duration: duration,
        }
    }

    pub fn play(endpoint: Endpoint) {
    }
}

pub fn key_to_base_note(mut key: Key, sequence: i8) -> Option<String> {
    let mut offset: i8 = 0;

    let keys = ['z', 's', 'x', 'c', 'f', 'v', 'g', 'b', 'n',
                'j', 'm', 'k', '1', ',', 'q', 'l', '2', '.',
                'w', '/', 'e', '\'', '4', 'r', '5', 't', 'y',
                '7', 'u', '8', 'i', '9', 'o', 'p', '[', ']', 'a'];

    let base_sounds = ["a", "as", "b", "c", "cs", "d", "ds", "e", "f",
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
        offset -= 1;
    }

    // Increment `offset` if key was shift prefixed (Shift+<character>)
    let note: Option<String> = if let Key::Char(mut c) = key {
        if c.is_uppercase() {
            c = c.to_ascii_lowercase();
            offset += 1;
        } else if special.contains(&c) {
            let j = special.iter()
                           .position(|&key| key == c)
                           .unwrap();
            c = special_matches[j];
            offset += 1;
        }

        if let Some(i) = keys.iter().position(|&key| key == c) {
            let init_pos = init_poses[i];
            let factor = factors[i];

            let base_note = format!("{}{}",
                base_sounds[i].to_string(),
                offset + factor + sequence
            );

            Some(base_note)

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
