extern crate rustbox;
extern crate rodio;

use std::default::Default;
use std::io::BufReader;
//use std::thread;
//use std::time::Duration;

use rustbox::{Color, RustBox};
use rustbox::Key;

/*
█▒
*/

fn print_whitekeys(rustbox: &RustBox) {
    for y in 0..16 {
        // last border is lonely
        rustbox.print(156, y, rustbox::RB_BOLD, Color::Black, Color::White, "|");
        for x in 0..52 {
            let k = x*3;
            rustbox.print(k, y, rustbox::RB_BOLD, Color::Black, Color::White, "|");
            rustbox.print(k+1, y, rustbox::RB_BOLD, Color::White, Color::Black, "██");
        }
    }
}


fn print_blackkeys(rustbox: &RustBox) {
    for y in 0..9 {
        // first black key is lonely
        rustbox.print(3, y, rustbox::RB_BOLD, Color::Black, Color::White, "█");

        for x in 0..7 {
            let g1k1 = x*21 + 9;
            let g1k2 = g1k1 + 3;
            rustbox.print(g1k1, y, rustbox::RB_BOLD, Color::Black, Color::White, "█");
            rustbox.print(g1k2, y, rustbox::RB_BOLD, Color::Black, Color::White, "█");

            let g2k1 = g1k2 + 6;
            let g2k2 = g2k1 + 3;
            let g2k3 = g2k2 + 3;
            rustbox.print(g2k1, y, rustbox::RB_BOLD, Color::Black, Color::White, "█");
            rustbox.print(g2k2, y, rustbox::RB_BOLD, Color::Black, Color::White, "█");
            rustbox.print(g2k3, y, rustbox::RB_BOLD, Color::Black, Color::White, "█");
        }

    }
}


fn play_note(note: &str, mark: (i16, bool), sequence: i16, endpoint: &rodio::Endpoint, rustbox: &RustBox) {
    let file_path = format!("assets/{0}{1}.ogg", note, sequence);
    let file = std::fs::File::open(file_path).unwrap();
    rodio::play_once(endpoint, BufReader::new(file))
        .unwrap()
        .detach();
    print_whitekeys(rustbox);
    print_blackkeys(rustbox);
    let (x, white) = mark;

    let pos = x + (sequence + 1)*21;
    if white {
        rustbox.print(pos as usize, 15, rustbox::RB_BOLD, Color::Red, Color::White, "▒▒");
    } else {
        rustbox.print(pos as usize, 8, rustbox::RB_BOLD, Color::Red, Color::White, "▒");
    }
}


fn main() {
    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };
    let endpoint = rodio::get_default_endpoint().unwrap();

    print_whitekeys(&rustbox);
    print_blackkeys(&rustbox);
    let mut sequence: i16 = 2;

    loop {
        rustbox.present();
        match rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                // println!("{:?}", key);
                match key {
                    Key::Char('z') => { play_note("a" , (1, true)     , sequence-1, &endpoint, &rustbox); }
                    Key::Char('s') => { play_note("as", (3, false)    , sequence-1, &endpoint, &rustbox); }
                    Key::Char('x') => { play_note("b" , (4, true)     , sequence-1, &endpoint, &rustbox); }
                    Key::Char('c') => { play_note("c" , (7-21, true)  , sequence,   &endpoint, &rustbox); }
                    Key::Char('f') => { play_note("cs", (9-21, false) , sequence,   &endpoint, &rustbox); }
                    Key::Char('v') => { play_note("d" , (10-21, true) , sequence,   &endpoint, &rustbox); }
                    Key::Char('g') => { play_note("ds", (12-21, false), sequence,   &endpoint, &rustbox); }
                    Key::Char('b') => { play_note("e" , (13-21, true) , sequence,   &endpoint, &rustbox); }
                    Key::Char('n') => { play_note("f" , (16-21, true) , sequence,   &endpoint, &rustbox); }
                    Key::Char('j') => { play_note("fs", (18-21, false), sequence,   &endpoint, &rustbox); }
                    Key::Char('m') => { play_note("g" , (19-21, true) , sequence,   &endpoint, &rustbox); }
                    Key::Char('k')  | Key::Char('1') => { play_note("gs", (21-21, false), sequence,   &endpoint, &rustbox); }
                    Key::Char(',')  | Key::Char('q') => { play_note("a" , (22-21, true) , sequence,   &endpoint, &rustbox); }
                    Key::Char('l')  | Key::Char('2') => { play_note("as", (24-21, false), sequence,   &endpoint, &rustbox); }
                    Key::Char('.')  | Key::Char('w') => { play_note("b" , (25-21, true) , sequence,   &endpoint, &rustbox); }
                    Key::Char('/')  | Key::Char('e') => { play_note("c" , (28-42, true) , sequence+1, &endpoint, &rustbox); }
                    Key::Char('\'') | Key::Char('4') => { play_note("cs", (30-42, false), sequence+1, &endpoint, &rustbox); }
                    Key::Char('r') => { play_note("d" , (31-42, true) , sequence+1, &endpoint, &rustbox); }
                    Key::Char('5') => { play_note("ds", (33-42, false), sequence+1, &endpoint, &rustbox); }
                    Key::Char('t') => { play_note("e" , (34-42, true) , sequence+1, &endpoint, &rustbox); }
                    Key::Char('y') => { play_note("f" , (37-42, true) , sequence+1, &endpoint, &rustbox); }
                    Key::Char('7') => { play_note("fs", (39-42, false), sequence+1, &endpoint, &rustbox); }
                    Key::Char('u') => { play_note("g" , (40-42, true) , sequence+1, &endpoint, &rustbox); }
                    Key::Char('8') => { play_note("gs", (42-42, false), sequence+1, &endpoint, &rustbox); }
                    Key::Char('i') => { play_note("a" , (43-42, true) , sequence+1, &endpoint, &rustbox); }
                    Key::Char('9') => { play_note("as", (45-42, false), sequence+1, &endpoint, &rustbox); }
                    Key::Char('o') => { play_note("b" , (46-42, true) , sequence+1, &endpoint, &rustbox); }
                    Key::Char('p') => { play_note("c" , (49-63, true) , sequence+2, &endpoint, &rustbox); }
                    Key::Char('[') => { if sequence < 5 { play_note("d" , (52-63, true), sequence+2, &endpoint, &rustbox); } }
                    Key::Char(']') => { if sequence < 5 { play_note("e" , (55-63, true), sequence+2, &endpoint, &rustbox); } }
                    Key::Char('a') => { if sequence > 0 { play_note("gs", (0, false), sequence-1, &endpoint, &rustbox); } }
                    Key::Right => { if sequence < 5 { sequence += 1 } }
                    Key::Left  => { if sequence > 0 { sequence -= 1 } }
                    Key::Esc   => { break; }
                    _ => { }
                }
            },
            Err(e) => panic!("{}", e),
            _ => { }
        }
    }
}

