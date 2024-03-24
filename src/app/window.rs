pub mod utils;

pub struct Window<C: std::fmt::Display> {
    title: &'static str,
    x: u16,
    y: u16,
    width: u16,
    height: u16,
    color: C,
}

impl<C: std::fmt::Display> Window<C> {
    pub fn new(title: &'static str, x: u16, y: u16, width: u16, height: u16, color: C) -> Self {
        utils::create(title, x, y, width, height, &color);
        Window {
            title,
            x,
            y,
            width,
            height,
            color,
        }
    }

    pub fn print<Color: std::fmt::Display>(self, text: &str, x: u16, y: u16, color: Color) {
        print!(
            "{}{}{}{}",
            termion::cursor::Goto(self.x + x, self.y + y),
            color,
            text,
            termion::style::Reset
        );
    }

    pub fn update_contents(
        self,
        contents: &Vec<String>,
        color: Option<Box<dyn std::fmt::Display>>,
        one_line: bool,
    ) {
        print!(
            "{}{}",
            self.color,
            termion::cursor::Goto(self.x + 1, self.y + 1),
        );
        if color.is_some() {
            print!("{}", color.unwrap());
        }
        for i in 0..contents.len() {
            print!("{} ", contents[i]);
            if !one_line {
                print!(
                    "{}",
                    termion::cursor::Goto(self.x + 1, self.y + 2 + i as u16)
                );
            }
        }
        print!("{}", termion::style::Reset);
    }
}