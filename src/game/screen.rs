use std::{thread, time};

use crossterm::{
    style,
    queue,
    Colorize,
    Goto,
    PrintStyledFont,
};

use crossterm_style::Color;

use std::io::{stdout, Write};

/*
█▒
*/

pub mod pianokeys {
    use crossterm::{
        queue,
        Colorize,
        Goto,
        PrintStyledFont,
        Result,
    };

    use std::io::{stdout, Stdout, Write};

    struct Point {
        x: u16,
        y: u16,
    }

    pub fn draw() -> Result<()> {
        let mut stdout = stdout();
        print_whites(&mut stdout)?;
        print_blacks(&mut stdout)?;
        stdout.flush()?;
        Ok(())
    }

    fn print_whitekey(initial_point: Point, stdout: &mut Stdout) -> Result<()> {
        let key_height: u16 = 16;

        for column_height in 0..key_height {
            queue!(
                stdout,
                Goto(initial_point.x, initial_point.y + column_height),
                PrintStyledFont("|".black().on_white())
            )?;
            queue!(
                stdout,
                Goto(initial_point.x + 1, initial_point.y + column_height),
                PrintStyledFont("██".white())
            )?;
            queue!(
                stdout,
                Goto(initial_point.x + 3, initial_point.y + column_height),
                PrintStyledFont("|".black())
            )?;
        }
        Ok(())
    }

    fn print_whites(stdout: &mut Stdout) -> Result<()> {
        for key in 0..58 {
            let initial_point = Point { x: key * 3, y: 0 };
            print_whitekey(initial_point, stdout)?;
        }
        Ok(())
    }

    fn print_blackkey(initial_point: Point, stdout: &mut Stdout) -> Result<()> {
        let key_height = 9;
        for column_height in 0..key_height {
            queue!(
                stdout,
                Goto(initial_point.x, initial_point.y + column_height),
                PrintStyledFont("█".black())
            )?;
        }
        Ok(())
    }

    fn print_blacks(stdout: &mut Stdout) -> Result<()> {
        // First black key is lonely
        let mut initial_point = Point { x: 3, y: 0 };
        print_blackkey(initial_point, stdout)?;

        for x in 0..8 {
            let g1k1 = x * 21 + 9;
            let g1k2 = g1k1 + 3;
            initial_point = Point { x: g1k1, y: 0 };
            print_blackkey(initial_point, stdout)?;
            initial_point = Point { x: g1k2, y: 0 };
            print_blackkey(initial_point, stdout)?;

            let g2k1 = g1k2 + 6;
            let g2k2 = g2k1 + 3;
            let g2k3 = g2k2 + 3;
            initial_point = Point { x: g2k1, y: 0 };
            print_blackkey(initial_point, stdout)?;
            initial_point = Point { x: g2k2, y: 0 };
            print_blackkey(initial_point, stdout)?;
            initial_point = Point { x: g2k3, y: 0 };
            print_blackkey(initial_point, stdout)?;
        }

        Ok(())
    }
}

pub fn mark_note(pos: i16, white: bool, color: Color, duration: time::Duration) {
    if white {
        // This causes a compiler panic!
        /* queue!( */
        /*     stdout(), */
        /*     Goto(pos as u16, 15), */
        /*     PrintStyledFont(StyledObject("██").with(color)) */
        /* ).unwrap(); */

        queue!(
            stdout(),
            Goto(pos as u16, 15),
            PrintStyledFont(style("██").with(color))
        ).unwrap();

    /* println!("{} Red foreground text", Colored::Fg(Color::Red)); */
    } else {
        queue!(
            stdout(),
            Goto(pos as u16, 8),
            PrintStyledFont(style("█").with(color))
        ).unwrap();
    }

    thread::spawn(move || {
        thread::sleep(duration);
        if white {
        queue!(
            stdout(),
            Goto(pos as u16, 15),
            PrintStyledFont("██".white())
        ).unwrap();
        } else {
        queue!(
            stdout(),
            Goto(pos as u16, 8),
            PrintStyledFont("█".black())
        ).unwrap();
        }
    });
}

