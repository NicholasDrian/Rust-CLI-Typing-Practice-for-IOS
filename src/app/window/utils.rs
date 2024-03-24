use termion::style;

pub fn create(
    title: &str,
    x: u16,
    y: u16,
    width: u16,
    height: u16,
    color: &Box<dyn std::fmt::Display>,
) {
    print!("{}{}{}", termion::cursor::Goto(x, y), color, style::Invert);
    print!("╔");
    for _ in 0..width {
        print!("═");
    }
    print!("╗");

    for i in 1..=height {
        print!("{}║", termion::cursor::Goto(x, y + i));
        print!("{}║", termion::cursor::Goto(x + width + 1, y + i));
    }

    print!(
        "{}{}{}",
        termion::cursor::Goto(x, y + height + 1),
        color,
        style::Invert
    );
    print!("╚");
    for _ in 0..width {
        print!("═");
    }
    print!("╝");

    print!("{}{}", termion::cursor::Goto(x + 2, y), title);
    print!("{}", style::Reset);
}
