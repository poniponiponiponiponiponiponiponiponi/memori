use std::cmp;
use std::io::{self, Write};
use std::sync::mpsc::Receiver;

use crossterm;
use crossterm::cursor;
use crossterm::style::Color;
use crossterm::style::SetBackgroundColor;
use crossterm::terminal::{Clear, ClearType};

pub fn bar(progress_recv: Receiver<(usize, usize)>) {
    let bar_width: usize = cmp::min(64, crossterm::terminal::size().unwrap().0 as usize - 3);
    let (_, y) = cursor::position().unwrap();
    let mut stdout = io::stdout();

    crossterm::execute!(stdout, cursor::Hide).unwrap();
    for (scanned, to_scan) in progress_recv {
        crossterm::execute!(stdout, cursor::MoveTo(0, y)).unwrap();
        crossterm::execute!(stdout, Clear(ClearType::CurrentLine)).unwrap();

        let proportion = scanned as f64 / to_scan as f64;
        let bar_chars_amount = cmp::min((proportion * bar_width as f64) as usize, bar_width);
        let bar_chars = "-".repeat(bar_chars_amount);
        let padding = bar_width - bar_chars_amount;

        write!(stdout, "[").unwrap();
        crossterm::execute!(stdout, SetBackgroundColor(Color::Magenta)).unwrap();
        write!(stdout, "{}", bar_chars).unwrap();
        crossterm::execute!(stdout, SetBackgroundColor(Color::Reset)).unwrap();
        write!(stdout, "{}]", " ".repeat(padding)).unwrap();

        stdout.flush().unwrap();
    }

    crossterm::execute!(stdout, cursor::MoveTo(0, y)).unwrap();
    crossterm::execute!(stdout, Clear(ClearType::CurrentLine)).unwrap();
    crossterm::execute!(stdout, cursor::Show).unwrap();
}
