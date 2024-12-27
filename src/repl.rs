use crate::commands::Cli;

use clap::Parser;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Editor, Helper};
use rustyline::history::DefaultHistory;

static DEFAULT_PROMPT: &str = "memori λ ";

pub struct Repl<'a> {
    editor: Editor<(), DefaultHistory>,
    prompt: &'a str,
}

impl<'a> Repl<'a> {
    pub fn new() -> Self {
        Repl {
            editor: DefaultEditor::new().expect("I just don't know what went wrong"),
            prompt: DEFAULT_PROMPT,
        }
    }

    // Read a string from stdin. If None is returned it means the user
    // pressed C-c or C-d or we encountered an error. Either way the
    // caller probably should end the execution of the program.
    pub fn read(&mut self) -> Option<String> {
        let line = self.editor.readline(self.prompt);
        match line {
            Ok(line) => {
                self.editor.add_history_entry(&line).unwrap();
                Some(line.to_string())
            },
            Err(ReadlineError::Interrupted) => {
                None
            },
            Err(ReadlineError::Eof) => {
                None
            },
            Err(err) => {
                eprintln!("Error: {:?}", err);
                None
            }
        }
    }

    pub fn eval(&mut self) {
        
    }

    pub fn repl(&mut self) {
        loop {
            match self.read() {
                Some(line) => {
                    let cli = Cli::try_parse_from(line.split_whitespace());
                    if let Ok(cli) = cli {
                        cli.exec();
                    } else if let Err(e) = cli {
                        eprintln!("{}", e.render());
                    };
                },
                None => {
                    println!("WHYYY???");
                    break;
                }
            }
        }
    }
}
