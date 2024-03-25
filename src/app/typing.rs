use termion::color;

extern crate termios;
use super::window::Window;
use std::io;
use std::io::Read;
use std::io::Write;
use termios::{tcsetattr, Termios, ECHO, ICANON, TCSANOW};

const ESC: [u8; 1] = [27];
const DELETE: [u8; 1] = [127];
const NEW_LINE: [u8; 1] = [b'\n'];

pub fn run(window: &Window, text: &Vec<String>) {
    let stdin = 0;
    let termios = Termios::from_fd(stdin).unwrap();
    let mut new_termios = termios.clone();
    new_termios.c_lflag &= !(ICANON | ECHO);
    tcsetattr(stdin, TCSANOW, &new_termios).unwrap();
    let stdout = io::stdout();
    let mut reader = io::stdin();
    let mut buffer = [0; 1];

    let mut row: i32 = 0;
    let mut col: i32 = 0;

    let mut golbal_correct_chars: i32 = 0;
    let mut global_total_chars: i32 = 0;

    let mut correct_chars: i32 = 0;
    let mut total_chars: i32 = 0;

    update_scroll(window, text, 0);

    loop {
        stdout.lock().flush().unwrap();
        reader.read_exact(&mut buffer).unwrap();
        match buffer {
            ESC => {
                break;
            }
            DELETE => {
                on_delete(&text[row as usize], &mut col, &mut row, window);
            }
            NEW_LINE => {
                row += 1;
                col = 0;
                if row > text.len() as i32 {
                    break;
                }
                update_scroll(window, text, row);
            }
            _ => {
                on_char_typed(
                    &text[row as usize],
                    buffer[0] as char,
                    &mut col,
                    &mut row,
                    window,
                );
                col += 1;
            }
        }
    }
    tcsetattr(stdin, TCSANOW, &termios).unwrap();
}

fn on_char_typed(text: &String, c: char, col: &mut i32, row: &mut i32, window: &Window) {
    window.print(
        &c.to_string()[..],
        col.clone() as u16 + 1,
        row.clone() as u16 + 1,
        Box::new(color::Fg(color::Green)),
    );
}

fn on_delete(text: &String, col: &mut i32, row: &mut i32, window: &Window) {
    //TODO:
}

fn update_scroll(window: &Window, text: &Vec<String>, line: i32) {
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
