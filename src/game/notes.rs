use rustbox::Key;
use std::ascii::AsciiExt;


#[derive(Debug, PartialEq)]
pub struct Note {
    pub sound: String,
    pub sequence: i16,
    pub position: i16,
    pub white: bool,
}

pub fn match_note(mut key: Key, mut raw_seq: i16) -> Note {
    //TODO: Return smthn instinctive instead of fake data if key not matched
    let mut sound = String::new();
    let mut white = true;
    let mut factor = -1;
    let mut init_pos = -106;

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
    if let Key::Char(mut c) = key {
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
            sound = notes[i].to_string();
            init_pos = init_poses[i];
            white = whites[i];
            factor = factors[i];
        }
    }

    let position = init_pos + 21 * raw_seq;
    let sequence = raw_seq + factor;

    Note {
        sound: sound,
        sequence: sequence,
        position: position,
        white: white
    }
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
                              sound: "a".to_string(),
                              sequence: 2,
                              position: 64,
                              white: true
                           };

        assert_eq!(note, expect_note);
    }
}
