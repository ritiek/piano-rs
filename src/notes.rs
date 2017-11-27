extern crate rustbox;

use rustbox::Key;


#[derive(Debug)]
pub struct Note {
    pub sound: String,
    pub sequence: i16,
    pub position: i16,
    pub white: bool,
}

pub fn match_note(key: rustbox::Key, mut raw_seq: i16) -> Note {
    //TODO: return smthn instinctive instead of fake data if key not matched
    let mut sound = String::new();
    let mut white = true;
    let mut factor = -1;
    let mut init_pos = -106;

    match key {
        Key::Char('z') => {
            sound = "a".to_string();
            init_pos = 1;
            white = true;
            factor = -1;
        }
        Key::Char('s') => {
            sound = "as".to_string();
            init_pos = 3;
            white = false;
            factor = -1;
        }
        Key::Char('x') => {
            sound = "b".to_string();
            init_pos = 4;
            white = true;
            factor = -1;
        }
        Key::Char('c') => {
            sound = "c".to_string();
            init_pos = 7;
            white = true;
            factor = 0;
        }
        Key::Char('f') => {
            sound = "cs".to_string();
            init_pos = 9;
            white = false;
            factor = 0;
        }
        Key::Char('v') => {
            sound = "d".to_string();
            init_pos = 10;
            white = true;
            factor = 0;
        }
        Key::Char('g') => {
            sound = "ds".to_string();
            init_pos = 12;
            white = false;
            factor = 0;
        }
        Key::Char('b') => {
            sound = "e".to_string();
            init_pos = 13;
            white = true;
            factor = 0;
        }
        Key::Char('n') => {
            sound = "f".to_string();
            init_pos = 16;
            white = true;
            factor = 0;
        }
        Key::Char('j') => {
            sound = "fs".to_string();
            init_pos = 18;
            white = false;
            factor = 0;
        }
        Key::Char('m') => {
            sound = "g".to_string();
            init_pos = 19;
            white = true;
            factor = 0;
        }
        Key::Char('k') | Key::Char('1') => {
            sound = "gs".to_string();
            init_pos = 21;
            white = false;
            factor = 0;
        }
        Key::Char(',') | Key::Char('q') => {
            sound = "a".to_string();
            init_pos = 22;
            white = true;
            factor = 0;
        }
        Key::Char('l') | Key::Char('2') => {
            sound = "as".to_string();
            init_pos = 24;
            white = false;
            factor = 0;
        }
        Key::Char('.') | Key::Char('w') => {
            sound = "b".to_string();
            init_pos = 25;
            white = true;
            factor = 0;
        }
        Key::Char('/') | Key::Char('e') => {
            sound= "c".to_string();
            init_pos = 28;
            white = true;
            factor = 1;
        }
        Key::Char('\'') | Key::Char('4') => {
            sound = "cs".to_string();
            init_pos = 30;
            white = false;
            factor = 1;
        }
        Key::Char('r') => {
            sound = "d".to_string();
            init_pos = 31;
            white = true;
            factor = 1;
        }
        Key::Char('5') => {
            sound = "ds".to_string();
            init_pos = 33;
            white = false;
            factor = 1;
        }
        Key::Char('t') => {
            sound = "e".to_string();
            init_pos = 34;
            white = true;
            factor = 1;
        }
        Key::Char('y') => {
            sound = "f".to_string();
            init_pos = 37;
            white = true;
            factor = 1;
        }
        Key::Char('7') => {
            sound= "fs".to_string();
            init_pos = 39;
            white = false;
            factor = 1;
        }
        Key::Char('u') => {
            sound = "g".to_string();
            init_pos = 40;
            white = true;
            factor = 1;
        }
        Key::Char('8') => {
            sound = "gs".to_string();
            init_pos = 42;
            white = false;
            factor = 1;
        }
        Key::Char('i') => {
            sound = "a".to_string();
            init_pos = 43;
            white = true;
            factor = 1;
        }
        Key::Char('9') => {
            sound = "as".to_string();
            init_pos = 45;
            white = false;
            factor = 1;
        }
        Key::Char('o') => {
            sound = "b".to_string();
            init_pos = 46;
            white = true;
            factor = 1;
        }
        Key::Char('p') => {
            sound = "c".to_string();
            init_pos = 49;
            white = true;
            factor = 2;
        }
        Key::Char('[') => {
            sound = "d".to_string();
            init_pos = 52;
            white = true;
            factor = 2;
        }
        Key::Char(']') => {
            sound = "e".to_string();
            init_pos = 55;
            white = true;
            factor = 2;
        }
        Key::Char('a') => {
            sound = "gs".to_string();
            init_pos = 0;
            white = false;
            factor = -1;
        }

        Key::Char('Z') => {
            sound = "a".to_string();
            init_pos = 1;
            white = true;
            factor = -1;
            raw_seq += 1;
        }
        Key::Char('S') => {
            sound = "as".to_string();
            init_pos = 3;
            white = false;
            factor = -1;
            raw_seq += 1;
        }
        Key::Char('X') => {
            sound = "b".to_string();
            init_pos = 4;
            white = true;
            factor = -1;
            raw_seq += 1;
        }
        Key::Char('C') => {
            sound = "c".to_string();
            init_pos = 7;
            factor = 0;
            raw_seq += 1;
        }
        Key::Char('F') => {
            sound = "cs".to_string();
            init_pos = 9;
            white = false;
            factor = 0;
            raw_seq += 1;
        }
        Key::Char('V') => {
            sound = "d".to_string();
            init_pos = 10;
            white = true;
            factor = 0;
            raw_seq += 1;
        }
        Key::Char('G') => {
            sound = "ds".to_string();
            init_pos = 12;
            white = false;
            factor = 0;
            raw_seq += 1;
        }
        Key::Char('B') => {
            sound = "e".to_string();
            init_pos = 13;
            white = true;
            factor = 0;
            raw_seq += 1;
        }
        Key::Char('N') => {
            sound = "f".to_string();
            init_pos = 16;
            white = true;
            factor = 0;
            raw_seq += 1;
        }
        Key::Char('J') => {
            sound = "fs".to_string();
            init_pos = 18;
            white = false;
            factor = 0;
            raw_seq += 1;
        }
        Key::Char('M') => {
            sound = "g".to_string();
            init_pos = 19;
            white = true;
            factor = 0;
            raw_seq += 1;
        }
        Key::Char('K') | Key::Char('!') => {
            sound = "gs".to_string();
            init_pos = 21;
            white = false;
            factor = 0;
            raw_seq += 1;
        }
        Key::Char('<') | Key::Char('Q') => {
            sound = "a".to_string();
            init_pos = 22;
            white = true;
            factor = 0;
            raw_seq += 1;
        }
        Key::Char('L') | Key::Char('@') => {
            sound = "as".to_string();
            init_pos = 24;
            white = false;
            factor = 0;
            raw_seq += 1;
        }
        Key::Char('>') | Key::Char('W') => {
            sound = "b".to_string();
            init_pos = 25;
            white = true;
            factor = 0;
            raw_seq += 1;
        }
        Key::Char('?') | Key::Char('E') => {
            sound = "c".to_string();
            init_pos = 28;
            white = true;
            factor = 1;
            raw_seq += 1;
        }
        Key::Char('|') | Key::Char('$') => {
            sound = "cs".to_string();
            init_pos = 30;
            white = false;
            factor = 1;
            raw_seq += 1;
        }
        Key::Char('R') => {
            sound = "d".to_string();
            init_pos = 31;
            white = true;
            factor = 1;
            raw_seq += 1;
        }
        Key::Char('%') => {
            sound = "ds".to_string();
            init_pos = 33;
            white = false;
            factor = 1;
            raw_seq += 1;
        }
        Key::Char('T') => {
            sound = "e".to_string();
            init_pos = 34;
            white = true;
            factor = 1;
            raw_seq += 1;
        }
        Key::Char('Y') => {
            sound = "f".to_string();
            init_pos = 37;
            white = true;
            factor = 1;
            raw_seq += 1;
        }
        Key::Char('&') => {
            sound = "fs".to_string();
            init_pos = 39;
            white = false;
            factor = 1;
            raw_seq += 1;
        }
        Key::Char('U') => {
            sound = "g".to_string();
            init_pos = 40;
            white = true;
            factor = 1;
            raw_seq += 1;
        }
        Key::Char('*') => {
            sound = "gs".to_string();
            init_pos = 42;
            white = false;
            factor = 1;
            raw_seq += 1;
        }
        Key::Char('I') => {
            sound = "a".to_string();
            init_pos = 43;
            white = true;
            factor = 1;
            raw_seq += 1;
        }
        Key::Char('(') => {
            sound = "as".to_string();
            init_pos = 45;
            white = false;
            factor = 1;
            raw_seq += 1;
        }
        Key::Char('O') => {
            sound = "b".to_string();
            init_pos = 46;
            white = true;
            factor = 1;
            raw_seq += 1;
        }
        Key::Char('P') => {
            sound = "c".to_string();
            init_pos = 49;
            white = true;
            factor = 2;
            raw_seq += 1;
        }
        Key::Char('{') => {
            sound = "d".to_string();
            init_pos = 52;
            white = true;
            factor = 2;
            raw_seq += 1;
        }
        Key::Char('}') => {
            sound = "e".to_string();
            init_pos = 55;
            white = true;
            factor = 3;
            raw_seq += 1;
        }

        Key::Ctrl('a') => {
            sound = "gs".to_string();
            init_pos = 0;
            white = false;
            factor = -1;
            raw_seq += -1;
        }
        Key::Ctrl('z') => {
            sound = "a".to_string();
            init_pos = 1;
            white = true;
            factor = -1;
            raw_seq += -1;
        }
        Key::Ctrl('s') => {
            sound = "as".to_string();
            init_pos = 3;
            white = false;
            factor = -1;
            raw_seq += -1;
        }
        Key::Ctrl('x') => {
            sound = "b".to_string();
            init_pos = 4;
            white = true;
            factor = -1;
            raw_seq += -1;
        }
        Key::Ctrl('c') => {
            sound = "c".to_string();
            init_pos = 7;
            white = true;
            factor = 0;
            raw_seq += -1;
        }
        Key::Ctrl('f') => {
            sound = "cs".to_string();
            init_pos = 9;
            white = false;
            factor = 0;
            raw_seq += -1;
        }
        Key::Ctrl('v') => {
            sound = "d".to_string();
            init_pos = 10;
            white = true;
            factor = 0;
            raw_seq += -1;
        }
        Key::Ctrl('g') => {
            sound = "ds".to_string();
            init_pos = 12;
            white = false;
            factor = 0;
            raw_seq += -1;
        }
        Key::Ctrl('b') => {
            sound = "e".to_string();
            init_pos = 13;
            white = true;
            factor = 0;
            raw_seq += -1;
        }
        Key::Ctrl('n') => {
            sound = "f".to_string();
            init_pos = 16;
            white = true;
            factor = 0;
            raw_seq += -1;
        }
        Key::Ctrl('j') => {
            sound = "fs".to_string();
            init_pos = 18;
            white = false;
            factor = 0;
            raw_seq += -1;
        }
        Key::Ctrl('m') => {
            sound = "g".to_string();
            init_pos = 19;
            white = true;
            factor = 0;
            raw_seq += -1;
        }
        Key::Ctrl('k') | Key::Ctrl('1') => {
            sound = "gs".to_string();
            init_pos = 21;
            white = false;
            factor = 0;
            raw_seq += -1;
        }
        Key::Ctrl(',') | Key::Ctrl('q') => {
            sound = "a".to_string();
            init_pos = 22;
            white = true;
            factor = 0;
            raw_seq += -1;
        }
        Key::Ctrl('l') | Key::Ctrl('2') => {
            sound = "as".to_string();
            init_pos = 24;
            white = false;
            factor = 0;
            raw_seq += -1;
        }
        Key::Ctrl('.') | Key::Ctrl('w') => {
            sound = "b".to_string();
            init_pos = 25;
            white = true;
            factor = 0;
            raw_seq += -1;
        }
        Key::Ctrl('/') | Key::Ctrl('e') => {
            sound = "c".to_string();
            init_pos = 28;
            white = true;
            factor = 0;
            raw_seq += -1;
        }
        Key::Ctrl('\'') | Key::Ctrl('4') => {
            sound = "cs".to_string();
            init_pos = 30;
            white = false;
            factor = 1;
            raw_seq += -1;
        }
        Key::Ctrl('r') => {
            sound = "d".to_string();
            init_pos = 31;
            white = true;
            factor = 1;
            raw_seq += -1;
        }
        Key::Ctrl('5') => {
            sound = "ds".to_string();
            init_pos = 33;
            white = false;
            factor = 1;
            raw_seq += -1;
        }
        Key::Ctrl('t') => {
            sound = "e".to_string();
            init_pos = 34;
            white = true;
            factor = 1;
            raw_seq += -1;
        }
        Key::Ctrl('y') => {
            sound = "f".to_string();
            init_pos = 37;
            white = true;
            factor = 1;
            raw_seq += -1;
        }
        Key::Ctrl('7') => {
            sound = "fs".to_string();
            init_pos = 39;
            white = false;
            factor = 1;
            raw_seq += -1;
        }
        Key::Ctrl('u') => {
            sound = "g".to_string();
            init_pos = 40;
            white = true;
            factor = 1;
            raw_seq += -1;
        }
        Key::Ctrl('8') => {
            sound = "gs".to_string();
            init_pos = 42;
            white = false;
            factor = 1;
            raw_seq += -1;
        }
        Key::Ctrl('i') => {
            sound = "a".to_string();
            init_pos = 43;
            white = true;
            factor = 1;
            raw_seq += -1;
        }
        Key::Ctrl('9') => {
            sound = "as".to_string();
            init_pos = 45;
            white = false;
            factor = 1;
            raw_seq += -1;
        }
        Key::Ctrl('o') => {
            sound = "b".to_string();
            init_pos = 46;
            white = true;
            factor = 1;
            raw_seq += -1;
        }
        Key::Ctrl('p') => {
            sound = "c".to_string();
            init_pos = 49;
            white = true;
            factor = 2;
            raw_seq += -1;
        }
        Key::Ctrl('[') => {
            sound = "d".to_string();
            init_pos = 52;
            white = true;
            factor = 2;
            raw_seq += -1;
        }
        Key::Ctrl(']') => {
            sound = "e".to_string();
            init_pos = 55;
            white = true;
            factor = 2;
            raw_seq += -1;
        }
        _ => {}
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
