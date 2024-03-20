mod file_io;
mod filter;
mod window;

use window::Window;

// for output
use std::io;
use termion::color;

// for input
extern crate termios;
use std::io::Read;
use std::io::Write;
use termios::{tcsetattr, Termios, ECHO, ICANON, TCSANOW};

enum Mode {
    Menu,
    Ready,
    Going,
    InputPath,
    InputExtension,
}

fn main() {
    print!("{}", termion::clear::All);

    let main_window = Window::new("typing by kot", 6, 6, 68, 20, color::Fg(color::Blue));
    let prompt_window = Window::new("prompt", 10, 10, 60, 10, color::Fg(color::Green));
    let path_window = Window::new("path to text", 10, 22, 40, 1, color::Fg(color::Yellow));
    let extension_window = Window::new("extension", 52, 22, 18, 1, color::Fg(color::LightRed));

    main_window.print(
        "0:Quit 1:Begin 2:UpdatePath 3:UpdateExtension",
        4,
        2,
        color::Fg(color::White),
    );

    run();
}

fn run() {
    let stdin = 0;
    let termios = Termios::from_fd(stdin).unwrap();
    let mut new_termios = termios.clone();
    new_termios.c_lflag &= !(ICANON | ECHO);
    tcsetattr(stdin, TCSANOW, &mut new_termios).unwrap();
    let stdout = io::stdout();
    let mut reader = io::stdin();
    let mut buffer = [0; 1];
    loop {
        print!("Hit a key! ");
        stdout.lock().flush().unwrap();
        reader.read_exact(&mut buffer).unwrap();
        match buffer {
            [b'0'] => {
                break;
            }
            _ => {
                print!("{:?}", buffer);
            }
        }
    }
    tcsetattr(stdin, TCSANOW, &termios).unwrap();
}
