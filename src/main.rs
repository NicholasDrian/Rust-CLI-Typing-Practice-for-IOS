mod file_io;
mod filter;
mod window;

use window::Window;

use std::io;
use termion::color;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

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

    //run();
}

fn run() {
    let mut stdout = io::stdout().into_raw_mode();
    let mut stdin = termion::async_stdin(); // maybe shouldnt be async?
    let mut it = stdin.keys();
    loop {
        let b = it.next();
        match b {
            Some(x) => match x {
                Ok(k) => match k {
                    Key::Left => {
                        return;
                    }
                    _ => {}
                },
                _ => {}
            },
            None => {}
        }
    }
}
