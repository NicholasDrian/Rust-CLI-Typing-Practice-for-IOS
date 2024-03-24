use termion::color;

extern crate termios;
use super::window::Window;
use std::io;
use std::io::Read;
use std::io::Write;
use termios::{tcsetattr, Termios, ECHO, ICANON, TCSANOW};

const ESC: [u8; 1] = [27];

pub fn run(window: &Window, text: &Vec<String>) {
    let stdin = 0;
    let termios = Termios::from_fd(stdin).unwrap();
    let mut new_termios = termios.clone();
    new_termios.c_lflag &= !(ICANON | ECHO);
    tcsetattr(stdin, TCSANOW, &mut new_termios).unwrap();
    let stdout = io::stdout();
    let mut reader = io::stdin();
    let mut buffer = [0; 1];

    let mut line: i32 = 0;
    let mut char: i32 = 0;

    draw_text(window, text, 0);

    loop {
        stdout.lock().flush().unwrap();
        reader.read_exact(&mut buffer).unwrap();
        match buffer {
            ESC => {
                break;
            }
            [b'\n'] => {
                line += 1;
                if line > text.len() as i32 {
                    break;
                }
                draw_text(window, text, line);
            }
            _ => {
                print!("{:?}", buffer);
            }
        }
    }
    tcsetattr(stdin, TCSANOW, &termios).unwrap();
}

fn draw_text(window: &Window, text: &Vec<String>, line: i32) {
    let half: i32 = (window.height as i32 + 1) / 2;
    let first_line: i32 = std::cmp::max(0, line as i32 - half);

    window.clear();
    for i in first_line..first_line + window.height as i32 {
        if i >= text.len() as i32 {
            break;
        }
        if i == line {
            print!("{}", termion::style::Invert);
        }
        window.print(
            &text[i as usize],
            1,
            i as u16 - first_line as u16 + 1,
            Box::new(color::Fg(color::LightWhite)),
        );
        print!("{}", termion::style::NoInvert);
    }
}
