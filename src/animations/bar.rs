use std::cmp;
use std::io::{self, Write};
use std::sync::mpsc::{Receiver, TryRecvError};
use std::mem;
use std::thread::sleep;
use std::time;

use crossterm;
use crossterm::cursor;
use crossterm::style::Color;
use crossterm::style::SetBackgroundColor;
use crossterm::terminal::{Clear, ClearType};
use owo_colors::colors::{Green, Yellow};
use owo_colors::OwoColorize;

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

#[derive(Clone, PartialEq, Eq)]
enum GOLCell {
    Dead,
    Newborn,
    Alive,
}

pub fn game_of_life(progress_recv: Receiver<(usize, usize)>) {
    let board_size_x = 20;
    let board_size_y = 10;
    let mut board = vec![vec![GOLCell::Dead; board_size_x]; board_size_y];
    let mut next_board = vec![vec![GOLCell::Dead; board_size_x]; board_size_y];

    // Random board that looks okayish.
    // In the future it would be cool to have them randomly generated
    // based on some patterns, like gliders etc.
    board[5][10] = GOLCell::Alive;
    board[5][11] = GOLCell::Alive;
    board[5][12] = GOLCell::Alive;

    board[2][3] = GOLCell::Alive;
    board[2][4] = GOLCell::Alive;
    board[2][5] = GOLCell::Alive;
    board[3][2] = GOLCell::Alive;
    board[3][3] = GOLCell::Alive;
    board[3][4] = GOLCell::Alive;

    board[6][2] = GOLCell::Alive;
    board[6][3] = GOLCell::Alive;
    board[7][2] = GOLCell::Alive;
    board[7][3] = GOLCell::Alive;

    let (_, cursor_y) = cursor::position().unwrap();
    let mut stdout = io::stdout();
    let mut percentage: i32 = 0;
    crossterm::execute!(stdout, cursor::Hide).unwrap();
    // Game of life main loop
    loop {
        // Logic loops
        for y in 1..board.len()-1 {
            for x in 1..board[y].len()-1 {
                let mut neighbours = 0;
                for x_diff in [-1isize, 0, 1] {
                    for y_diff in [-1isize, 0, 1] {
                        if x_diff == 0 && y_diff == 0 {
                            continue;
                        }
                        let new_x = (x as isize + x_diff) as usize;
                        let new_y = (y as isize + y_diff) as usize;
                        if board[new_y][new_x] != GOLCell::Dead {
                            neighbours += 1;
                        }
                    }
                }
                
                match board[y][x] {
                    GOLCell::Newborn | GOLCell::Alive => {
                        next_board[y][x] = if (2..=3).contains(&neighbours) {
                            GOLCell::Alive
                        } else {
                            GOLCell::Dead
                        };
                    },
                    GOLCell::Dead => {
                        next_board[y][x] = if neighbours == 3 {
                            GOLCell::Newborn
                        } else {
                            GOLCell::Dead
                        };
                    }
                }
            }
        }

        mem::swap(&mut board, &mut next_board);

        // "Rendering" loops
        for y in 0..board.len() {
            crossterm::execute!(stdout, cursor::MoveTo(0, cursor_y+y as u16)).unwrap();
            crossterm::execute!(stdout, Clear(ClearType::CurrentLine)).unwrap();
            for x in 0..board[y].len() {
                if board[y][x] == GOLCell::Newborn {
                    write!(stdout, "{}", " ".fg::<Green>().bg::<Green>()).unwrap();
                } else if board[y][x] == GOLCell::Alive {
                    write!(stdout, "{}", " ".fg::<Yellow>().bg::<Yellow>()).unwrap();
                } else {
                    write!(stdout, ".").unwrap();
                }
            }
        }

        crossterm::execute!(stdout, cursor::MoveTo(0, cursor_y+board.len() as u16)).unwrap();
        crossterm::execute!(stdout, Clear(ClearType::CurrentLine)).unwrap();
        write!(stdout, "Scanning... {}%", percentage).unwrap();
        stdout.flush().unwrap();

        // Handle "events"
        match progress_recv.try_recv() {
            Ok((from, to)) => {
                percentage = ((from as f64)/(to as f64) * 100.0) as i32;
                if from == to {
                    break;
                }
            },
            Err(TryRecvError::Disconnected) => {
                break;
            },
            Err(TryRecvError::Empty) => {}
        }

        sleep(time::Duration::from_millis(333));
    }

    // Cleanup everything
    for y in 0..board.len()+1 {
        crossterm::execute!(stdout, cursor::MoveTo(0, cursor_y+y as u16)).unwrap();
        crossterm::execute!(stdout, Clear(ClearType::CurrentLine)).unwrap();
    }
    crossterm::execute!(stdout, cursor::MoveTo(0, cursor_y)).unwrap();
    crossterm::execute!(stdout, Clear(ClearType::CurrentLine)).unwrap();
    crossterm::execute!(stdout, cursor::Show).unwrap();
}
