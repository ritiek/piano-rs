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

    for y in 0..16 {
        for x in 0..52 {
            rustbox.print(x*3, y, rustbox::RB_BOLD, Color::White, Color::Black, " ██");
            rustbox.print(x*3, y, rustbox::RB_BOLD, Color::Black, Color::White, "|");
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

