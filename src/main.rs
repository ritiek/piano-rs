extern crate clap;
extern crate rustbox;
extern crate rodio;

use std::default::Default;
use std::io::{BufReader, Read, Cursor};
use std::{thread, time};
use std::collections::HashMap;

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
            let k = x * 3;
            rustbox.print(k, y, rustbox::RB_BOLD, Color::Black, Color::White, "|");
            rustbox.print(k + 1, y, rustbox::RB_BOLD, Color::White, Color::Black, "██");
        }
    }
}


fn print_blackkeys(rustbox: &RustBox) {
    for y in 0..9 {
        // first black key is lonely
        rustbox.print(3, y, rustbox::RB_BOLD, Color::Black, Color::White, "█");

        for x in 0..7 {
            let g1k1 = x * 21 + 9;
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


fn draw(sequence: i16, mark: (i16, &str, bool), rustbox: &RustBox) {
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

    print_whitekeys(rustbox);
    print_blackkeys(rustbox);
    let (x, color, white) = mark;
    let color_pos = colors.iter().position(|&c| c == color).unwrap();

    let pos = x + (sequence + 1) * 21;
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

    let player = Player::new();

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
                    Key::Char('z') => {
                        player.play("a", seq - 1, dur);
                        draw(seq - 1, (1, c, true), &rb);
                    }
                    Key::Char('s') => {
                        player.play("as", seq - 1, dur);
                        draw(seq - 1, (3, c, false), &rb);
                    }
                    Key::Char('x') => {
                        player.play("b", seq - 1, dur);
                        draw(seq - 1, (4, c, true), &rb);
                    }
                    Key::Char('c') => {
                        player.play("c", seq, dur);
                        draw(seq, (7 - 21, c, true), &rb);
                    }
                    Key::Char('f') => {
                        player.play("cs", seq, dur);
                        draw(seq, (9 - 21, c, false), &rb);
                    }
                    Key::Char('v') => {
                        player.play("d", seq, dur);
                        draw(seq, (10 - 21, c, true), &rb);
                    }
                    Key::Char('g') => {
                        player.play("ds", seq, dur);
                        draw(seq, (12 - 21, c, false), &rb);
                    }
                    Key::Char('b') => {
                        player.play("e", seq, dur);
                        draw(seq, (13 - 21, c, true), &rb);
                    }
                    Key::Char('n') => {
                        player.play("f", seq, dur);
                        draw(seq, (16 - 21, c, true), &rb);
                    }
                    Key::Char('j') => {
                        player.play("fs", seq, dur);
                        draw(seq, (18 - 21, c, false), &rb);
                    }
                    Key::Char('m') => {
                        player.play("g", seq, dur);
                        draw(seq, (19 - 21, c, true), &rb);
                    }
                    Key::Char('k') | Key::Char('1') => {
                        player.play("gs", seq, dur);
                        draw(seq, (21 - 21, c, false), &rb);
                    }
                    Key::Char(',') | Key::Char('q') => {
                        player.play("a", seq, dur);
                        draw(seq, (22 - 21, c, true), &rb);
                    }
                    Key::Char('l') | Key::Char('2') => {
                        player.play("as", seq, dur);
                        draw(seq, (24 - 21, c, false), &rb);
                    }
                    Key::Char('.') | Key::Char('w') => {
                        player.play("b", seq, dur);
                        draw(seq, (25 - 21, c, true), &rb);
                    }
                    Key::Char('/') | Key::Char('e') => {
                        player.play("c", seq + 1, dur);
                        draw(seq + 1, (28 - 42, c, true), &rb);
                    }
                    Key::Char('\'') | Key::Char('4') => {
                        player.play("cs", seq + 1, dur);
                        draw(seq + 1, (30 - 42, c, false), &rb);
                    }
                    Key::Char('r') => {
                        player.play("d", seq + 1, dur);
                        draw(seq + 1, (31 - 42, c, true), &rb);
                    }
                    Key::Char('5') => {
                        player.play("ds", seq + 1, dur);
                        draw(seq + 1, (33 - 42, c, false), &rb);
                    }
                    Key::Char('t') => {
                        player.play("e", seq + 1, dur);
                        draw(seq + 1, (34 - 42, c, true), &rb);
                    }
                    Key::Char('y') => {
                        player.play("f", seq + 1, dur);
                        draw(seq + 1, (37 - 42, c, true), &rb);
                    }
                    Key::Char('7') => {
                        player.play("fs", seq + 1, dur);
                        draw(seq + 1, (39 - 42, c, false), &rb);
                    }
                    Key::Char('u') => {
                        player.play("g", seq + 1, dur);
                        draw(seq + 1, (40 - 42, c, true), &rb);
                    }
                    Key::Char('8') => {
                        player.play("gs", seq + 1, dur);
                        draw(seq + 1, (42 - 42, c, false), &rb);
                    }
                    Key::Char('i') => {
                        player.play("a", seq + 1, dur);
                        draw(seq + 1, (43 - 42, c, true), &rb);
                    }
                    Key::Char('9') => {
                        player.play("as", seq + 1, dur);
                        draw(seq + 1, (45 - 42, c, false), &rb);
                    }
                    Key::Char('o') => {
                        player.play("b", seq + 1, dur);
                        draw(seq + 1, (46 - 42, c, true), &rb);
                    }
                    Key::Char('p') => {
                        player.play("c", seq + 2, dur);
                        draw(seq + 2, (49 - 63, c, true), &rb);
                    }
                    Key::Char('[') => {
                        if seq < 5 {
                            player.play("d", seq + 2, dur);
                            draw(seq + 2, (52 - 63, c, true), &rb);
                        }
                    }
                    Key::Char(']') => {
                        if seq < 5 {
                            player.play("e", seq + 2, dur);
                            draw(seq + 2, (55 - 63, c, true), &rb);
                        }
                    }
                    Key::Char('a') => {
                        if seq > 0 {
                            player.play("gs", seq - 1, dur);
                            draw(seq - 1, (0, c, false), &rb);
                        }
                    }
                    Key::Right => { if seq < 5 { seq += 1 } }
                    Key::Left => { if seq > 0 { seq -= 1 } }
                    Key::Up => { if dur < 8000 { dur += 50 } }
                    Key::Down => { if dur > 0 { dur -= 50 } }
                    Key::Esc => { break; }
                    _ => {}
                }
            }
            Err(e) => panic!("{}", e),
            _ => {}
        }
    }
}


struct Player {
    endpoint: rodio::Endpoint,
    samples: HashMap<String, Vec<u8>>,
}

impl Player {
    pub fn new() -> Player {
        let endpoint: rodio::Endpoint = rodio::get_default_endpoint().unwrap();
        let mut samples = HashMap::new();

        for note in ["a", "as", "b", "c", "cs", "d", "ds", "e", "f", "fs", "g", "gs"].iter() {
            for sequence in -1..8_i16 {
                Player::read_note(*note, sequence)
                    .and_then(|sample| {
                        samples.insert(format!("{}{}", note, sequence), sample);
                        Some(())
                    });
            }
        }

        Player {
            endpoint,
            samples
        }
    }

    fn get(&self, note: &str, sequence: i16) -> Option<BufReader<Cursor<Vec<u8>>>> {
        self.samples.get(&format!("{}{}", note, sequence))
            .map(|v| BufReader::new(Cursor::new(v.clone())))
    }

    fn play(&self, note: &str, sequence: i16, duration: u32) {
        self.get(note, sequence)
            .map(|note| {
                let sink = rodio::play_once(&self.endpoint, note).expect("Cannot play");
                if duration == 0 {
                    sink.detach();
                } else {
                    thread::spawn(move || {
                        thread::sleep(time::Duration::from_millis(duration.into()));
                        sink.stop();
                    });
                }

                true
            });
    }

    fn read_note(note: &str, sequence: i16) -> Option<Vec<u8>> {
        let file_path = format!("assets/{0}{1}.ogg", note, sequence);
        std::fs::File::open(file_path)
            .map(|mut file| {
                let mut data = Vec::new();
                file.read_to_end(&mut data).unwrap();
                data
            }).ok()
    }
}
