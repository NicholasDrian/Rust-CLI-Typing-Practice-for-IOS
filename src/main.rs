mod file_io;
mod window;

fn main() {
    println!("hello world from main.rs");
    window::create::create();
    window::update::update_window();
    println!("{}", file_io::load_from_path("this is a path"));
}
/*use termion;
use termion::{color, style};

use std::fs;

use walkdir::DirEntry;
use walkdir::WalkDir;

struct Screen {}

impl Screen {
    fn init() {
        println!(
            "{}{}{}{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            color::Fg(color::Blue),
            style::Bold,
            style::Invert,
        );
        print!("╔");
        for _ in 0..50 {
            print!("═");
        }
        println!("╗");
        for _ in 0..20 {
            print!("║");
            for _ in 0..50 {
                print!(" ");
            }
            println!("║");
        }
        print!("╚");
        for _ in 0..50 {
            print!("═");
        }
        println!("╝");

        println!(
            "{}Typing Practice{}",
            termion::cursor::Goto(3, 2),
            style::Reset
        );
    }

    fn draw_window(x: u16, y: u16, width: u16, height: u16, title: &str, contents: &str) {}
}

struct Window {
    title: String,
    pos: (u16, u16),
    size: (u16, u16),
    focused: bool,
    contents: Vec<String>,
}

impl Window {
    pub fn new(title: &str, pos: (u16, u16), size: (u16, u16), contents: Vec<String>) -> Self {
        let res = Window {
            title: title.to_string(),
            pos,
            size,
            focused: false,
            contents,
        };
        res.draw();
        res
    }

    pub fn set_focus(mut self, on: bool) {
        self.focused = on;
        sel
    }

    fn draw(self) {
        println!(
            "{}{}",
            termion::cursor::Goto(self.pos.0 + 1, self.pos.1 + 1),
            self.contents
        );
    }
}

fn get_sub_paths_with_ext(path: &str, extension: &str) -> Vec<DirEntry> {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .to_str()
                .unwrap()
                .chars()
                .rev()
                .zip(extension.chars().rev())
                .all(|(a, b)| a == b)
        })
        .collect()
}

fn main() {
    let paths = get_sub_paths_with_ext("/Users/nickdrian/dev/rust/typing/src", ".rs");
    for p in paths {
        println!("{}", p.path().to_str().unwrap());
    }

    let window = Window::new("test window", (4, 4), (20, 10), vec![]);
    window.update_contents("yo dawg");

    //Screen::init();
}

use std::fs;

fn main() {
    let data = fs::read_to_string("/etc/hosts").expect("Unable to read file");
    println!("{}", data);
}

// ╝╗╔╚	╣╩╦╠═║╬
*/
