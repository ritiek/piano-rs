use rustbox::{Color, RustBox};
use std::sync::{Arc, Mutex};
use std::{thread, time};

/*
█▒
*/

pub mod pianokeys {
    use rustbox::{Color, RustBox};
    use std::sync::{Arc, Mutex};

    struct Point {
        x: u16,
        y: u16,
    }

    pub fn draw(keys: u16, rustbox: &Arc<Mutex<RustBox>>) {
        print_whites(rustbox);
        print_blacks(rustbox);
        rustbox.lock().unwrap().present();
    }

    fn print_whitekey(initial_point: Point, rustbox: &Arc<Mutex<RustBox>>) {
        let key_height: u16 = 16;

        for column_height in 0..key_height {
            rustbox.lock().unwrap().print(
                initial_point.x as usize,
                (initial_point.y + column_height) as usize,
                rustbox::RB_BOLD,
                Color::Black,
                Color::White,
                "|",
            );
            rustbox.lock().unwrap().print(
                (initial_point.x + 1) as usize,
                (initial_point.y + column_height) as usize,
                rustbox::RB_BOLD,
                Color::White,
                Color::Black,
                "██",
            );
            rustbox.lock().unwrap().print(
                (initial_point.x + 3) as usize,
                (initial_point.y + column_height) as usize,
                rustbox::RB_BOLD,
                Color::Black,
                Color::White,
                "|",
            );
        }
    }

    fn print_whites(rustbox: &Arc<Mutex<RustBox>>) {
        for key in 0..58 {
            let initial_point = Point { x: key * 3, y: 0 };
            print_whitekey(initial_point, rustbox);
        }
    }

    fn print_blackkey(initial_point: Point, rustbox: &Arc<Mutex<RustBox>>) {
        let key_height = 9;
        for column_height in 0..key_height {
            rustbox.lock().unwrap().print(
                initial_point.x as usize,
                (initial_point.y + column_height) as usize,
                rustbox::RB_BOLD,
                Color::Black,
                Color::White,
                "█",
            );
        }
    }

    fn print_blacks(rustbox: &Arc<Mutex<RustBox>>) {
        // First black key is lonely
        let mut initial_point = Point { x: 3, y: 0 };
        print_blackkey(initial_point, rustbox);

        for x in 0..8 {
            let g1k1 = x * 21 + 9;
            let g1k2 = g1k1 + 3;
            initial_point = Point { x: g1k1, y: 0 };
            print_blackkey(initial_point, rustbox);
            initial_point = Point { x: g1k2, y: 0 };
            print_blackkey(initial_point, rustbox);

            let g2k1 = g1k2 + 6;
            let g2k2 = g2k1 + 3;
            let g2k3 = g2k2 + 3;
            initial_point = Point { x: g2k1, y: 0 };
            print_blackkey(initial_point, rustbox);
            initial_point = Point { x: g2k2, y: 0 };
            print_blackkey(initial_point, rustbox);
            initial_point = Point { x: g2k3, y: 0 };
            print_blackkey(initial_point, rustbox);
        }
    }
}

pub fn mark_note(pos: i16, white: bool, color: Color, duration: time::Duration, rustbox: &Arc<Mutex<RustBox>>) {
    if white {
        rustbox.lock().unwrap().print(pos as usize, 15, rustbox::RB_BOLD, color, Color::White, "▒▒");
    } else {
        rustbox.lock().unwrap().print(pos as usize, 8, rustbox::RB_BOLD, color, Color::White, "▒");
    }

    rustbox.lock().unwrap().present();
    let clonebox = rustbox.clone();
    thread::spawn(move || {
        thread::sleep(duration);
        if white {
            clonebox.lock().unwrap().print(pos as usize, 15, rustbox::RB_BOLD, Color::White, Color::White, "▒▒");
        } else {
            clonebox.lock().unwrap().print(pos as usize, 8, rustbox::RB_BOLD, Color::Black, Color::White, "▒");
        }
    });
}

