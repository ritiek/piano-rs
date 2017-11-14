extern crate clap;
extern crate rodio;
extern crate rustbox;

use std::default::Default;
use std::io::{BufReader, Read, Cursor};
use std::sync::{Arc, Mutex};
use std::{thread, time};
use std::collections::HashMap;

use clap::{Arg, App};

use rustbox::{Color, RustBox};
use rustbox::Key;

/*
█▒
*/

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
                        let delay = time::Duration::from_millis(duration.into());
                        thread::sleep(delay);
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


fn print_whitekeys(rustbox: &Arc<Mutex<RustBox>>) {
    for y in 0..16 {
        // last border is lonely
        rustbox.lock().unwrap().print(156, y, rustbox::RB_BOLD, Color::Black, Color::White, "|");
        for x in 0..52 {
            let k = x * 3;
            rustbox.lock().unwrap().print(k, y, rustbox::RB_BOLD, Color::Black, Color::White, "|");
            rustbox.lock().unwrap().print(k + 1, y, rustbox::RB_BOLD, Color::White, Color::Black, "██");
        }
    }
}


fn print_blackkeys(rustbox: &Arc<Mutex<RustBox>>) {
    for y in 0..9 {
        // first black key is lonely
        rustbox.lock().unwrap().print(3, y, rustbox::RB_BOLD, Color::Black, Color::White, "█");

        for x in 0..7 {
            let g1k1 = x * 21 + 9;
            let g1k2 = g1k1 + 3;
            rustbox.lock().unwrap().print(g1k1, y, rustbox::RB_BOLD, Color::Black, Color::White, "█");
            rustbox.lock().unwrap().print(g1k2, y, rustbox::RB_BOLD, Color::Black, Color::White, "█");

            let g2k1 = g1k2 + 6;
            let g2k2 = g2k1 + 3;
            let g2k3 = g2k2 + 3;
            rustbox.lock().unwrap().print(g2k1, y, rustbox::RB_BOLD, Color::Black, Color::White, "█");
            rustbox.lock().unwrap().print(g2k2, y, rustbox::RB_BOLD, Color::Black, Color::White, "█");
            rustbox.lock().unwrap().print(g2k3, y, rustbox::RB_BOLD, Color::Black, Color::White, "█");
        }
    }
}


fn draw(sequence: i16, mark: (i16, &str, bool), duration: u32, rustbox: Arc<Mutex<RustBox>>) {
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

    let (x, color, white) = mark;
    let color_pos = colors.iter().position(|&c| c == color).unwrap();

    let pos = x + (sequence + 1) * 21;
    if white {
        rustbox.lock().unwrap().print(pos as usize, 15, rustbox::RB_BOLD, rb_colors[color_pos], Color::White, "▒▒");
    } else {
        rustbox.lock().unwrap().print(pos as usize, 8, rustbox::RB_BOLD, rb_colors[color_pos], Color::White, "▒");
    }

    rustbox.lock().unwrap().present();
    thread::spawn(move || {
        let delay = time::Duration::from_millis(duration.into());
        thread::sleep(delay);
        if white {
            rustbox.lock().unwrap().print(pos as usize, 15, rustbox::RB_BOLD, Color::White, Color::White, "▒▒");
        } else {
            rustbox.lock().unwrap().print(pos as usize, 8, rustbox::RB_BOLD, Color::Black, Color::White, "▒");
        }
    });
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

        .arg(Arg::with_name("noteduration")
            .short("n")
            .long("note-duration")
            .value_name("DURATION")
            .takes_value(true)
            .help("Duration to play each note for, where 0 means till the end of note (Default: 0)"))

        .arg(Arg::with_name("markduration")
            .short("m")
            .long("mark-duration")
            .value_name("DURATION")
            .takes_value(true)
            .help("Duration to show piano mark for, in ms (Default: 500)"))

        .get_matches();

    // A workaround to stop cracking noise after note ends (#4)
    let blank_point = rodio::get_default_endpoint().unwrap();
    let blank_sink = rodio::Sink::new(&blank_point);
    let blank_source = rodio::source::SineWave::new(0);
    blank_sink.append(blank_source);

    let rb = match RustBox::init(Default::default()) {
        Result::Ok(v) => Arc::new(Mutex::new(v)),
        Result::Err(e) => panic!("{}", e),
    };

    let player = Player::new();

    print_whitekeys(&rb);
    print_blackkeys(&rb);
    let mut seq: i16 = matches.value_of("sequence").unwrap_or("2").parse().unwrap();
    let mut ndur: u32 = matches.value_of("noteduration").unwrap_or("0").parse().unwrap();
    let mdur: u32 = matches.value_of("markduration").unwrap_or("500").parse().unwrap();
    let c = matches.value_of("color").unwrap_or("red");
    rb.lock().unwrap().present();

    loop {
        let pe = rb.lock().unwrap().poll_event(false);
        let rb = rb.clone();
        match pe {
            Ok(rustbox::Event::KeyEvent(key)) => {
                // println!("{:?}", key);
                match key {
                    Key::Char('z') => {
                        player.play("a", seq - 1, ndur);
                        draw(seq - 1, (1, c, true), mdur, rb);
                    }
                    Key::Char('s') => {
                        player.play("as", seq - 1, ndur);
                        draw(seq - 1, (3, c, false), mdur, rb);
                    }
                    Key::Char('x') => {
                        player.play("b", seq - 1, ndur);
                        draw(seq - 1, (4, c, true), mdur, rb);
                    }
                    Key::Char('c') => {
                        player.play("c", seq, ndur);
                        draw(seq, (7 - 21, c, true), mdur, rb);
                    }
                    Key::Char('f') => {
                        player.play("cs", seq, ndur);
                        draw(seq, (9 - 21, c, false), mdur, rb);
                    }
                    Key::Char('v') => {
                        player.play("d", seq, ndur);
                        draw(seq, (10 - 21, c, true), mdur, rb);
                    }
                    Key::Char('g') => {
                        player.play("ds", seq, ndur);
                        draw(seq, (12 - 21, c, false), mdur, rb);
                    }
                    Key::Char('b') => {
                        player.play("e", seq, ndur);
                        draw(seq, (13 - 21, c, true), mdur, rb);
                    }
                    Key::Char('n') => {
                        player.play("f", seq, ndur);
                        draw(seq, (16 - 21, c, true), mdur, rb);
                    }
                    Key::Char('j') => {
                        player.play("fs", seq, ndur);
                        draw(seq, (18 - 21, c, false), mdur, rb);
                    }
                    Key::Char('m') => {
                        player.play("g", seq, ndur);
                        draw(seq, (19 - 21, c, true), mdur, rb);
                    }
                    Key::Char('k') | Key::Char('1') => {
                        player.play("gs", seq, ndur);
                        draw(seq, (21 - 21, c, false), mdur, rb);
                    }
                    Key::Char(',') | Key::Char('q') => {
                        player.play("a", seq, ndur);
                        draw(seq, (22 - 21, c, true), mdur, rb);
                    }
                    Key::Char('l') | Key::Char('2') => {
                        player.play("as", seq, ndur);
                        draw(seq, (24 - 21, c, false), mdur, rb);
                    }
                    Key::Char('.') | Key::Char('w') => {
                        player.play("b", seq, ndur);
                        draw(seq, (25 - 21, c, true), mdur, rb);
                    }
                    Key::Char('/') | Key::Char('e') => {
                        player.play("c", seq + 1, ndur);
                        draw(seq + 1, (28 - 42, c, true), mdur, rb);
                    }
                    Key::Char('\'') | Key::Char('4') => {
                        player.play("cs", seq + 1, ndur);
                        draw(seq + 1, (30 - 42, c, false), mdur, rb);
                    }
                    Key::Char('r') => {
                        player.play("d", seq + 1, ndur);
                        draw(seq + 1, (31 - 42, c, true), mdur, rb);
                    }
                    Key::Char('5') => {
                        player.play("ds", seq + 1, ndur);
                        draw(seq + 1, (33 - 42, c, false), mdur, rb);
                    }
                    Key::Char('t') => {
                        player.play("e", seq + 1, ndur);
                        draw(seq + 1, (34 - 42, c, true), mdur, rb);
                    }
                    Key::Char('y') => {
                        player.play("f", seq + 1, ndur);
                        draw(seq + 1, (37 - 42, c, true), mdur, rb);
                    }
                    Key::Char('7') => {
                        player.play("fs", seq + 1, ndur);
                        draw(seq + 1, (39 - 42, c, false), mdur, rb);
                    }
                    Key::Char('u') => {
                        player.play("g", seq + 1, ndur);
                        draw(seq + 1, (40 - 42, c, true), mdur, rb);
                    }
                    Key::Char('8') => {
                        player.play("gs", seq + 1, ndur);
                        draw(seq + 1, (42 - 42, c, false), mdur, rb);
                    }
                    Key::Char('i') => {
                        player.play("a", seq + 1, ndur);
                        draw(seq + 1, (43 - 42, c, true), mdur, rb);
                    }
                    Key::Char('9') => {
                        player.play("as", seq + 1, ndur);
                        draw(seq + 1, (45 - 42, c, false), mdur, rb);
                    }
                    Key::Char('o') => {
                        player.play("b", seq + 1, ndur);
                        draw(seq + 1, (46 - 42, c, true), mdur, rb);
                    }
                    Key::Char('p') => {
                        player.play("c", seq + 2, ndur);
                        draw(seq + 2, (49 - 63, c, true), mdur, rb);
                    }
                    Key::Char('[') => {
                        if seq < 5 {
                            player.play("d", seq + 2, ndur);
                            draw(seq + 2, (52 - 63, c, true), mdur, rb);
                        }
                    }
                    Key::Char(']') => {
                        if seq < 5 {
                            player.play("e", seq + 2, ndur);
                            draw(seq + 2, (55 - 63, c, true), mdur, rb);
                        }
                    }
                    Key::Char('a') => {
                        if seq > 0 {
                            player.play("gs", seq - 1, ndur);
                            draw(seq - 1, (0, c, false), mdur, rb);
                        }
                    }
                    Key::Char('Z') => {
                        player.play("a", seq, ndur);
                        draw(seq, (1, c, true), mdur, rb);
                    }
                    Key::Char('S') => {
                        player.play("as", seq, ndur);
                        draw(seq, (3, c, false), mdur, rb);
                    }
                    Key::Char('X') => {
                        player.play("b", seq, ndur);
                        draw(seq, (4, c, true), mdur, rb);
                    }
                    Key::Char('C') => {
                        player.play("c", seq + 1, ndur);
                        draw(seq + 1, (7 - 21, c, true), mdur, rb);
                    }
                    Key::Char('F') => {
                        player.play("cs", seq + 1, ndur);
                        draw(seq + 1, (9 - 21, c, false), mdur, rb);
                    }
                    Key::Char('V') => {
                        player.play("d", seq + 1, ndur);
                        draw(seq + 1, (10 - 21, c, true), mdur, rb);
                    }
                    Key::Char('G') => {
                        player.play("ds", seq + 1, ndur);
                        draw(seq + 1, (12 - 21, c, false), mdur, rb);
                    }
                    Key::Char('B') => {
                        player.play("e", seq + 1, ndur);
                        draw(seq + 1, (13 - 21, c, true), mdur, rb);
                    }
                    Key::Char('N') => {
                        player.play("f", seq + 1, ndur);
                        draw(seq + 1, (16 - 21, c, true), mdur, rb);
                    }
                    Key::Char('J') => {
                        player.play("fs", seq + 1, ndur);
                        draw(seq + 1, (18 - 21, c, false), mdur, rb);
                    }
                    Key::Char('M') => {
                        player.play("g", seq + 1, ndur);
                        draw(seq + 1, (19 - 21, c, true), mdur, rb);
                    }
                    Key::Char('K') | Key::Char('!') => {
                        player.play("gs", seq + 1, ndur);
                        draw(seq + 1, (21 - 21, c, false), mdur, rb);
                    }
                    Key::Char('<') | Key::Char('Q') => {
                        player.play("a", seq + 1, ndur);
                        draw(seq + 1, (22 - 21, c, true), mdur, rb);
                    }
                    Key::Char('L') | Key::Char('@') => {
                        player.play("as", seq + 1, ndur);
                        draw(seq + 1, (24 - 21, c, false), mdur, rb);
                    }
                    Key::Char('>') | Key::Char('W') => {
                        player.play("b", seq + 1, ndur);
                        draw(seq + 1, (25 - 21, c, true), mdur, rb);
                    }
                    Key::Char('?') | Key::Char('E') => {
                        player.play("c", seq + 2, ndur);
                        draw(seq + 2, (28 - 42, c, true), mdur, rb);
                    }
                    Key::Char('|') | Key::Char('$') => {
                        player.play("cs", seq + 2, ndur);
                        draw(seq + 2, (30 - 42, c, false), mdur, rb);
                    }
                    Key::Char('R') => {
                        player.play("d", seq + 2, ndur);
                        draw(seq + 2, (31 - 42, c, true), mdur, rb);
                    }
                    Key::Char('%') => {
                        player.play("ds", seq + 2, ndur);
                        draw(seq + 2, (33 - 42, c, false), mdur, rb);
                    }
                    Key::Char('T') => {
                        player.play("e", seq + 2, ndur);
                        draw(seq + 2, (34 - 42, c, true), mdur, rb);
                    }
                    Key::Char('Y') => {
                        player.play("f", seq + 2, ndur);
                        draw(seq + 2, (37 - 42, c, true), mdur, rb);
                    }
                    Key::Char('&') => {
                        player.play("fs", seq + 2, ndur);
                        draw(seq + 2, (39 - 42, c, false), mdur, rb);
                    }
                    Key::Char('U') => {
                        player.play("g", seq + 2, ndur);
                        draw(seq + 2, (40 - 42, c, true), mdur, rb);
                    }
                    Key::Char('*') => {
                        player.play("gs", seq + 2, ndur);
                        draw(seq + 2, (42 - 42, c, false), mdur, rb);
                    }
                    Key::Char('I') => {
                        player.play("a", seq + 2, ndur);
                        draw(seq + 2, (43 - 42, c, true), mdur, rb);
                    }
                    Key::Char('(') => {
                        player.play("as", seq + 2, ndur);
                        draw(seq + 2, (45 - 42, c, false), mdur, rb);
                    }
                    Key::Char('O') => {
                        player.play("b", seq + 2, ndur);
                        draw(seq + 2, (46 - 42, c, true), mdur, rb);
                    }
                    Key::Char('P') => {
                        player.play("c", seq + 3, ndur);
                        draw(seq + 3, (49 - 63, c, true), mdur, rb);
                    }
                    Key::Char('{') => {
                        if seq < 5 {
                            player.play("d", seq + 3, ndur);
                            draw(seq + 3, (52 - 63, c, true), mdur, rb);
                        }
                    }
                    Key::Char('}') => {
                        if seq < 5 {
                            player.play("e", seq + 3, ndur);
                            draw(seq + 3, (55 - 63, c, true), mdur, rb);
                        }
                    }
                    Key::Ctrl('A') => {
                        if seq > 0 {
                            player.play("gs", seq, ndur);
                            draw(seq, (0, c, false), mdur, rb);
                        }
                    }
                    Key::Ctrl('z') => {
                        player.play("a", seq - 2, ndur);
                        draw(seq - 2, (1, c, true), mdur, rb);
                    }
                    Key::Ctrl('s') => {
                        player.play("as", seq - 2, ndur);
                        draw(seq - 2, (3, c, false), mdur, rb);
                    }
                    Key::Ctrl('x') => {
                        player.play("b", seq - 2, ndur);
                        draw(seq - 2, (4, c, true), mdur, rb);
                    }
                    Key::Ctrl('c') => {
                        player.play("c", seq - 1, ndur);
                        draw(seq - 1, (7 - 21, c, true), mdur, rb);
                    }
                    Key::Ctrl('f') => {
                        player.play("cs", seq - 1, ndur);
                        draw(seq - 1, (9 - 21, c, false), mdur, rb);
                    }
                    Key::Ctrl('v') => {
                        player.play("d", seq - 1, ndur);
                        draw(seq - 1, (10 - 21, c, true), mdur, rb);
                    }
                    Key::Ctrl('g') => {
                        player.play("ds", seq - 1, ndur);
                        draw(seq - 1, (12 - 21, c, false), mdur, rb);
                    }
                    Key::Ctrl('b') => {
                        player.play("e", seq - 1, ndur);
                        draw(seq - 1, (13 - 21, c, true), mdur, rb);
                    }
                    Key::Ctrl('n') => {
                        player.play("f", seq - 1, ndur);
                        draw(seq - 1, (16 - 21, c, true), mdur, rb);
                    }
                    Key::Ctrl('j') => {
                        player.play("fs", seq - 1, ndur);
                        draw(seq - 1, (18 - 21, c, false), mdur, rb);
                    }
                    Key::Ctrl('m') => {
                        player.play("g", seq - 1, ndur);
                        draw(seq - 1, (19 - 21, c, true), mdur, rb);
                    }
                    Key::Ctrl('k') | Key::Ctrl('1') => {
                        player.play("gs", seq - 1, ndur);
                        draw(seq - 1, (21 - 21, c, false), mdur, rb);
                    }
                    Key::Ctrl(',') | Key::Ctrl('q') => {
                        player.play("a", seq - 1, ndur);
                        draw(seq - 1, (22 - 21, c, true), mdur, rb);
                    }
                    Key::Ctrl('l') | Key::Ctrl('2') => {
                        player.play("as", seq - 1, ndur);
                        draw(seq - 1, (24 - 21, c, false), mdur, rb);
                    }
                    Key::Ctrl('.') | Key::Ctrl('w') => {
                        player.play("b", seq - 1, ndur);
                        draw(seq - 1, (25 - 21, c, true), mdur, rb);
                    }
                    Key::Ctrl('/') | Key::Ctrl('e') => {
                        player.play("c", seq, ndur);
                        draw(seq, (28 - 42, c, true), mdur, rb);
                    }
                    Key::Ctrl('\'') | Key::Ctrl('4') => {
                        player.play("cs", seq, ndur);
                        draw(seq, (30 - 42, c, false), mdur, rb);
                    }
                    Key::Ctrl('r') => {
                        player.play("d", seq, ndur);
                        draw(seq, (31 - 42, c, true), mdur, rb);
                    }
                    Key::Ctrl('5') => {
                        player.play("ds", seq, ndur);
                        draw(seq, (33 - 42, c, false), mdur, rb);
                    }
                    Key::Ctrl('t') => {
                        player.play("e", seq, ndur);
                        draw(seq, (34 - 42, c, true), mdur, rb);
                    }
                    Key::Ctrl('y') => {
                        player.play("f", seq, ndur);
                        draw(seq, (37 - 42, c, true), mdur, rb);
                    }
                    Key::Ctrl('7') => {
                        player.play("fs", seq, ndur);
                        draw(seq, (39 - 42, c, false), mdur, rb);
                    }
                    Key::Ctrl('u') => {
                        player.play("g", seq, ndur);
                        draw(seq, (40 - 42, c, true), mdur, rb);
                    }
                    Key::Ctrl('8') => {
                        player.play("gs", seq, ndur);
                        draw(seq, (42 - 42, c, false), mdur, rb);
                    }
                    Key::Ctrl('i') => {
                        player.play("a", seq, ndur);
                        draw(seq, (43 - 42, c, true), mdur, rb);
                    }
                    Key::Ctrl('9') => {
                        player.play("as", seq, ndur);
                        draw(seq, (45 - 42, c, false), mdur, rb);
                    }
                    Key::Ctrl('o') => {
                        player.play("b", seq, ndur);
                        draw(seq, (46 - 42, c, true), mdur, rb);
                    }
                    Key::Ctrl('p') => {
                        player.play("c", seq + 1, ndur);
                        draw(seq + 1, (49 - 63, c, true), mdur, rb);
                    }
                    Key::Ctrl('[') => {
                        if seq < 5 {
                            player.play("d", seq + 1, ndur);
                            draw(seq + 1, (52 - 63, c, true), mdur, rb);
                        }
                    }
                    Key::Ctrl(']') => {
                        if seq < 5 {
                            player.play("e", seq + 1, ndur);
                            draw(seq + 1, (55 - 63, c, true), mdur, rb);
                        }
                    }
                    Key::Ctrl('a') => {
                        if seq > 0 {
                            player.play("gs", seq - 2, ndur);
                            draw(seq - 2, (0, c, false), mdur, rb);
                        }
                    }
                    Key::Right => { if seq < 5 { seq += 1 } }
                    Key::Left => { if seq > 0 { seq -= 1 } }
                    Key::Up => { if ndur < 8000 { ndur += 50 } }
                    Key::Down => { if ndur > 0 { ndur -= 50 } }
                    Key::Esc => { break; }
                    _ => {}
                }
            }
            Err(e) => panic!("{}", e),
            _ => {}
        }
    }
}
