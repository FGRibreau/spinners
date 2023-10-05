use yansi::Paint;

#[derive(Debug, Clone, Copy)]
pub enum Color {
    Blue,
    Green,
    Red,
    Yellow,
    Cyan,
    White,
    Magenta,
    Black,
}

pub fn colorize(input: String, color: Option<Color>) -> Paint<String> {
    match color {
        Some(Color::Blue) => Paint::blue(input),
        Some(Color::Green) => Paint::green(input),
        Some(Color::Red) => Paint::red(input),
        Some(Color::Yellow) => Paint::yellow(input),
        Some(Color::Cyan) => Paint::cyan(input),
        Some(Color::White) => Paint::white(input),
        Some(Color::Magenta) => Paint::magenta(input),
        Some(Color::Black) => Paint::black(input),
        None => Paint::new(input),
    }
}
