use termion::{color, style};
use termion::color::Color;

// Ugly logging implementation, will do for now

fn inner_log<C: Color>(color: C, text: &str) {
    println!(
        "{}{}{}",
        color::Fg(color),
        text,
        style::Reset
    );
}

pub fn info(text: &str) {
    inner_log(color::Blue, text)
}

pub fn success(text: &str) {
    inner_log(color::Green, text)
}

pub fn warn(text: &str) {
    inner_log(color::Yellow, text)
}

pub fn error(text: &str) {
    inner_log(color::Red, text)
}
