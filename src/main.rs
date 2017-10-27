extern crate clap;
extern crate rustbox;
extern crate rodio;

use std::default::Default;
use std::io::BufReader;
use std::{thread, time};

use clap::{Arg, App};

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


fn play_note(note: &str, mark: (i16, &str, bool), sequence: i16, duration: u32, endpoint: &rodio::Endpoint, rustbox: &RustBox) {
    let rb_colors = [
        Color::Black,
        Color::Red,
        Color::Green,
        Color::Yellow,
        Color::Blue,
        Color::Magenta,
        Color::Cyan,
        Color::White
    ];

    let colors = [
        "black",
        "red",
        "green",
        "yellow",
        "blue",
        "magenta",
        "cyan",
        "white"
    ];

    let file_path = format!("assets/{0}{1}.ogg", note, sequence);
    let file = std::fs::File::open(file_path).unwrap();
    let sink = rodio::play_once(endpoint, BufReader::new(file)).unwrap();
    if duration == 0 {
        sink.detach();
    } else {
        thread::spawn(move || {
            thread::sleep(time::Duration::from_millis(300));
            sink.stop();
        });
    }

    print_whitekeys(rustbox);
    print_blackkeys(rustbox);
    let (x, color, white) = mark;
    let color_pos = colors.iter().position(|&c| c == color).unwrap();

    let pos = x + (sequence + 1)*21;
    if white {
        rustbox.print(pos as usize, 15, rustbox::RB_BOLD, rb_colors[color_pos], Color::White, "▒▒");
    } else {
        rustbox.print(pos as usize, 8, rustbox::RB_BOLD, rb_colors[color_pos], Color::White, "▒");
    }
}


fn main() {
    let matches = App::new("piano-rs")
        .version("0.1.0")
        .author("Ritiek Malhotra <ritiekmalhotra123@gmail.com>")
        .about("Play piano in the terminal using PC keyboard.")

        .arg(Arg::with_name("color")
        .short("c")
        .long("color")
        .value_name("COLOR")
        .takes_value(true)
        .help("Color of block to generate when a note is played (Default: \"red\")"))

        .arg(Arg::with_name("sequence")
        .short("s")
        .long("sequence")
        .value_name("SEQUENCE")
        .takes_value(true)
        .help("Frequency sequence from 0 to 5 to begin with (Default: 2)"))

        .arg(Arg::with_name("duration")
        .short("d")
        .long("duration")
        .value_name("DURATION")
        .takes_value(true)
        .help("Duration to play each note for, where 0 means till the end of note (Default: 0)"))

        .get_matches();

    let rb: RustBox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };
    let ep: rodio::Endpoint = rodio::get_default_endpoint().unwrap();

    print_whitekeys(&rb);
    print_blackkeys(&rb);
    let mut seq: i16 = matches.value_of("sequence").unwrap_or("2").parse().unwrap();
    let mut dur: u32 = matches.value_of("duration").unwrap_or("0").parse().unwrap();
    let c = matches.value_of("color").unwrap_or("red");

    loop {
        rb.present();
        match rb.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                // println!("{:?}", key);
                match key {
                    Key::Char('z') => { play_note("a" , (1,     c, true),  seq-1, dur, &ep, &rb); }
                    Key::Char('s') => { play_note("as", (3,     c, false), seq-1, dur, &ep, &rb); }
                    Key::Char('x') => { play_note("b" , (4,     c, true),  seq-1, dur, &ep, &rb); }
                    Key::Char('c') => { play_note("c" , (7-21,  c, true),  seq,   dur, &ep, &rb); }
                    Key::Char('f') => { play_note("cs", (9-21,  c, false), seq,   dur, &ep, &rb); }
                    Key::Char('v') => { play_note("d" , (10-21, c, true),  seq,   dur, &ep, &rb); }
                    Key::Char('g') => { play_note("ds", (12-21, c, false), seq,   dur, &ep, &rb); }
                    Key::Char('b') => { play_note("e" , (13-21, c, true),  seq,   dur, &ep, &rb); }
                    Key::Char('n') => { play_note("f" , (16-21, c, true),  seq,   dur, &ep, &rb); }
                    Key::Char('j') => { play_note("fs", (18-21, c, false), seq,   dur, &ep, &rb); }
                    Key::Char('m') => { play_note("g" , (19-21, c, true),  seq,   dur, &ep, &rb); }
                    Key::Char('k')  | Key::Char('1') => { play_note("gs", (21-21, c, false), seq,   dur, &ep, &rb); }
                    Key::Char(',')  | Key::Char('q') => { play_note("a" , (22-21, c, true),  seq,   dur, &ep, &rb); }
                    Key::Char('l')  | Key::Char('2') => { play_note("as", (24-21, c, false), seq,   dur, &ep, &rb); }
                    Key::Char('.')  | Key::Char('w') => { play_note("b" , (25-21, c, true),  seq,   dur, &ep, &rb); }
                    Key::Char('/')  | Key::Char('e') => { play_note("c" , (28-42, c, true),  seq+1, dur, &ep, &rb); }
                    Key::Char('\'') | Key::Char('4') => { play_note("cs", (30-42, c, false), seq+1, dur, &ep, &rb); }
                    Key::Char('r') => { play_note("d" , (31-42, c, true),  seq+1, dur, &ep, &rb); }
                    Key::Char('5') => { play_note("ds", (33-42, c, false), seq+1, dur, &ep, &rb); }
                    Key::Char('t') => { play_note("e" , (34-42, c, true),  seq+1, dur, &ep, &rb); }
                    Key::Char('y') => { play_note("f" , (37-42, c, true),  seq+1, dur, &ep, &rb); }
                    Key::Char('7') => { play_note("fs", (39-42, c, false), seq+1, dur, &ep, &rb); }
                    Key::Char('u') => { play_note("g" , (40-42, c, true),  seq+1, dur, &ep, &rb); }
                    Key::Char('8') => { play_note("gs", (42-42, c, false), seq+1, dur, &ep, &rb); }
                    Key::Char('i') => { play_note("a" , (43-42, c, true),  seq+1, dur, &ep, &rb); }
                    Key::Char('9') => { play_note("as", (45-42, c, false), seq+1, dur, &ep, &rb); }
                    Key::Char('o') => { play_note("b" , (46-42, c, true),  seq+1, dur, &ep, &rb); }
                    Key::Char('p') => { play_note("c" , (49-63, c, true),  seq+2, dur, &ep, &rb); }
                    Key::Char('[') => { if seq < 5 { play_note("d" , (52-63, c, true),  seq+2, dur, &ep, &rb); } }
                    Key::Char(']') => { if seq < 5 { play_note("e" , (55-63, c, true),  seq+2, dur, &ep, &rb); } }
                    Key::Char('a') => { if seq > 0 { play_note("gs", (0,     c, false), seq-1, dur, &ep, &rb); } }
                    Key::Right => { if seq < 5 { seq += 1 } }
                    Key::Left  => { if seq > 0 { seq -= 1 } }
                    Key::Up    => { if dur < 8000 { dur += 50 } }
                    Key::Down  => { if dur > 0    { dur -= 50 } }
                    Key::Esc   => { break; }
                    _ => { }
                }
            },
            Err(e) => panic!("{}", e),
            _ => { }
        }
    }
}
