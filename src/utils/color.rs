use yansi::Paint;

#[derive(Debug, Clone, Copy)]
pub enum Color {
    Blue,
    Green,
    Red,
    Yellow,
    Cyan,
    White,
}

pub fn colorize(input: String, color: Option<Color>) -> Paint<String> {
    match color {
        Some(Color::Blue) => Paint::blue(input),
        Some(Color::Green) => Paint::green(input),
        Some(Color::Red) => Paint::red(input),
        Some(Color::Yellow) => Paint::yellow(input),
        Some(Color::Cyan) => Paint::cyan(input),
        Some(Color::White) => Paint::new(input),
        None => Paint::new(input),
    }
}
