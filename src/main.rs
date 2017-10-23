extern crate rustbox;
extern crate rodio;
extern crate crossbeam;

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


fn play_note(note: &str, mark: (usize, bool), sequence: i16, endpoint: &rodio::Endpoint, rustbox: &RustBox) {
    let file_path = format!("assets/{0}{1}.ogg", note, sequence);
    let file = std::fs::File::open(file_path).unwrap();
    rodio::play_once(endpoint, BufReader::new(file))
        .unwrap()
        .detach();
    print_whitekeys(rustbox);
    print_blackkeys(rustbox);
    let (x, white) = mark;

    if white {
        rustbox.print(x, 15, rustbox::RB_BOLD, Color::Red, Color::White, "▒▒");
    } else {
        rustbox.print(x, 8, rustbox::RB_BOLD, Color::Red, Color::White, "▒");
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
                    Key::Char('z') => { play_note("a" , (1, true), sequence-1, &endpoint, &rustbox); }
                    Key::Char('s') => { play_note("as", (3, false), sequence-1, &endpoint, &rustbox); }
                    /*Key::Char('x') => { play_note("b" , sequence-1, &endpoint); }
                    Key::Char('c') => { play_note("c" , sequence,   &endpoint); }
                    Key::Char('f') => { play_note("cs", sequence,   &endpoint); }
                    Key::Char('v') => { play_note("d" , sequence,   &endpoint); }
                    Key::Char('g') => { play_note("ds", sequence,   &endpoint); }
                    Key::Char('b') => { play_note("e" , sequence,   &endpoint); }
                    Key::Char('n') => { play_note("f" , sequence,   &endpoint); }
                    Key::Char('j') => { play_note("fs", sequence,   &endpoint); }
                    Key::Char('m') => { play_note("g" , sequence,   &endpoint); }
                    Key::Char('k')  | Key::Char('1') => { play_note("gs", sequence,   &endpoint); }
                    Key::Char(',')  | Key::Char('q') => { play_note("a" , sequence,   &endpoint); }
                    Key::Char('l')  | Key::Char('2') => { play_note("as", sequence,   &endpoint); }
                    Key::Char('.')  | Key::Char('w') => { play_note("b" , sequence,   &endpoint); }
                    Key::Char('/')  | Key::Char('e') => { play_note("c" , sequence+1, &endpoint); }
                    Key::Char('\'') | Key::Char('4') => { play_note("cs", sequence+1, &endpoint); }
                    Key::Char('r') => { play_note("d" , sequence+1, &endpoint); }
                    Key::Char('5') => { play_note("ds", sequence+1, &endpoint); }
                    Key::Char('t') => { play_note("e" , sequence+1, &endpoint); }
                    Key::Char('y') => { play_note("f" , sequence+1, &endpoint); }
                    Key::Char('7') => { play_note("fs", sequence+1, &endpoint); }
                    Key::Char('u') => { play_note("g" , sequence+1, &endpoint); }
                    Key::Char('8') => { play_note("gs", sequence+1, &endpoint); }
                    Key::Char('i') => { play_note("a" , sequence+1, &endpoint); }
                    Key::Char('9') => { play_note("as", sequence+1, &endpoint); }
                    Key::Char('o') => { play_note("b" , sequence+1, &endpoint); }
                    Key::Char('p') => { play_note("c" , sequence+2, &endpoint); }
                    Key::Char('[') => if sequence < 5 { play_note("d" , sequence+2, &endpoint); }
                    Key::Char(']') => if sequence < 5 { play_note("e" , sequence+2, &endpoint); }
                    Key::Char('a') => if sequence > 0 { play_note("gs", sequence-1, &endpoint); }*/
                    Key::Right => if sequence < 5 { sequence += 1 }
                    Key::Left  => if sequence > 0 { sequence -= 1 }
                    Key::Esc   => { break; }
                    _ => { }
                }
            },
            Err(e) => panic!("{}", e),
            _ => { }
        }
    }
}

