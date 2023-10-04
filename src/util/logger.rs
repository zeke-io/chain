use crossterm::execute;
use crossterm::style::{Color, Print, ResetColor, SetForegroundColor};
use std::io::stdout;

// Ugly and rushed logging implementation, will do for now
// TODO: Refactor?

fn inner_log(color: Color, text: &str) {
    let _ = execute!(
        stdout(),
        SetForegroundColor(color),
        Print(text),
        Print("\n"),
        ResetColor
    );
}

pub fn info(text: &str) {
    inner_log(Color::Blue, text)
}

pub fn success(text: &str) {
    inner_log(Color::Green, text)
}

pub fn warn(text: &str) {
    inner_log(Color::Yellow, text)
}

pub fn error(text: &str) {
    inner_log(Color::Red, text)
}
