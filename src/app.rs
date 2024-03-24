mod file_io;
mod filter;
mod typing;
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

pub struct App {
    mode: Mode,
    path: String,
    extensions: Vec<String>,
    text: Vec<String>,
}

impl App {
    pub fn new(mut args: Vec<String>) -> Self {
        if args.len() == 1 {
            args.push("/Users/nickdrian/dev/rust/learn_webgpu/".to_string());
            args.push(".rs".to_string());
            args.push(".cpp".to_string());
        }
        if args.len() == 2 {
            args.push(".rs".to_string());
            args.push(".cpp".to_string());
        }
        print!("{}", termion::clear::All);

        let mut mode: Mode = Mode::Menu;

        let main_window = Window::new("TYPING", 6, 6, 108, 25, Box::new(color::Fg(color::Blue)));

        let path_window = Window::new("PATH", 10, 28, 58, 1, Box::new(color::Fg(color::Yellow)));
        let path: String = args[1].clone();
        path_window.update_contents(&args[1..2].to_vec(), None, true);

        let extension_window = Window::new(
            "EXTENSIONS",
            72,
            28,
            38,
            1,
            Box::new(color::Fg(color::LightRed)),
        );
        let extensions: Vec<String> = args[2..].to_vec();
        extension_window.update_contents(&extensions, None, true);

        let prompt_window =
            Window::new("PROMPT", 10, 10, 100, 15, Box::new(color::Fg(color::Green)));
        let mut text: Vec<String> =
            file_io::concat_file_contents(file_io::get_sub_paths_with_exts(&path, &extensions))
                .lines()
                .map(|s| s.to_string())
                .collect();
        if text.len() == 0 {
            text = vec!["could not load files".to_string()];
        } else {
            mode = Mode::Ready;
        }
        text = filter::remove_long_lines(90, &text);
        text = text.into_iter().take(15).collect();
        prompt_window.update_contents(&text, Some(Box::new(color::Fg(color::LightWhite))), false);

        main_window.print(
            Self::get_instructions(&mode),
            4,
            2,
            Box::new(color::Fg(color::LightWhite)),
        );
        App {
            mode,
            path,
            extensions,
            text,
        }
    }

    pub fn run(self) {
        let stdin = 0;
        let termios = Termios::from_fd(stdin).unwrap();
        let mut new_termios = termios.clone();
        new_termios.c_lflag &= !(ICANON | ECHO);
        tcsetattr(stdin, TCSANOW, &mut new_termios).unwrap();
        let stdout = io::stdout();
        let mut reader = io::stdin();
        let mut buffer = [0; 1];
        loop {
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

    fn get_instructions(mode: &Mode) -> &str {
        match mode {
            Mode::Menu => "0:Quit 1:Begin 2:UpdatePath 3:UpdateExtension",
            Mode::Ready | Mode::Going => "[ESC]:BackToMenu Begin Typing",
            Mode::InputPath => "[ESC]:BackToMenu Enter Path to Typable Files",
            Mode::InputExtension => "[ESC]:BackToMenu Enter Allowed Extensions",
        }
    }
}
