extern crate rustbox;

use std::default::Default;

use rustbox::{Color, RustBox};
use rustbox::Key;

/*
█▒
*/

fn main() {
    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    /* white keys */
    for y in 0..16 {
        for x in 0..52 {
            let key = x*3;
            rustbox.print(key, y, rustbox::RB_BOLD, Color::White, Color::Black, " ██");
            rustbox.print(key, y, rustbox::RB_BOLD, Color::Black, Color::White, "|");
        }
    }

    /*black keys */
    for y in 0..9 {
        rustbox.print(3, y, rustbox::RB_BOLD, Color::Black, Color::White, "█");
        for x in 0..7 {
            let g1k1 = (x*21) + 9;
            let g1k2 = g1k1 + 3;
            rustbox.print(g1k1, y, rustbox::RB_BOLD, Color::Black, Color::White, "█");
            rustbox.print(g1k2, y, rustbox::RB_BOLD, Color::Black, Color::White, "█");
        }
    }

    loop {
        rustbox.present();
        match rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Key::Char('q') => { break; }
                    _ => { }
                }
            },
            Err(e) => panic!("{}", e),
            _ => { }
        }
    }
}

