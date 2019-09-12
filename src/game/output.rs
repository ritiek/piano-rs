extern crate rustbox;

use rustbox::{Color, RustBox};
use std::sync::{Arc, Mutex};
use std::{thread, time};

/*
█▒
*/

fn print_whitekeys(rustbox: &Arc<Mutex<RustBox>>) {
    for y in 0..16 {
        // Last border is lonely
        rustbox.lock().unwrap().print(156, y, rustbox::RB_BOLD, Color::Black, Color::White, "|");
        for x in 0..52 {
            let k = x * 3;
            rustbox.lock().unwrap().print(k, y, rustbox::RB_BOLD, Color::Black, Color::White, "|");
            rustbox.lock().unwrap().print(k + 1, y, rustbox::RB_BOLD, Color::White, Color::Black, "██");
        }
    }
    rustbox.lock().unwrap().present();
}

fn print_blackkeys(rustbox: &Arc<Mutex<RustBox>>) {
    for y in 0..9 {
        // First black key is lonely
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
    rustbox.lock().unwrap().present();
}


pub fn display_keyboard(rustbox: &Arc<Mutex<RustBox>>) {
    print_whitekeys(rustbox);
    print_blackkeys(rustbox);
}

pub fn draw(pos: i16, white: bool, color: &str, duration: u32, rustbox: Arc<Mutex<RustBox>>) {
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

    let color_pos = colors.iter().position(|&c| c == color).unwrap();

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

