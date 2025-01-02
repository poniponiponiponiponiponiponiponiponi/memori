use crate::commands::{Cli, Command};
use crate::context::Context;
use crate::util;

use clap::Parser;
use rustyline::error::ReadlineError;
use rustyline::history::DefaultHistory;
use rustyline::{DefaultEditor, Editor};

static DEFAULT_PROMPT: &str = "memori λ ";

pub struct Repl<'a> {
    editor: Editor<(), DefaultHistory>,
    prompt: &'a str,
}

pub struct Message {
    pub message: String,
    pub is_error: bool,
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
            }
            Err(ReadlineError::Interrupted) => None,
            Err(ReadlineError::Eof) => None,
            Err(err) => {
                eprintln!("Error: {:?}", err);
                None
            }
        }
    }

    pub fn eval(&mut self, cmd: &Command, ctx: &mut Context) -> Message {
        match cmd {
            Command::Process(process_args) => match ctx.process(process_args) {
                Ok(()) => {
                    let message = format!(
                        "connected to process: {}",
                        ctx.process.as_ref().unwrap().command
                    )
                    .to_string();
                    return Message {
                        message,
                        is_error: false,
                    };
                }
                Err(err) => {
                    return Message {
                        message: err.to_string(),
                        is_error: true,
                    };
                }
            },
            Command::Type(type_args) => {
                ctx.change_type(type_args);
                return Message {
                    message: format!("changed type successfuly to {}", ctx.get_type()),
                    is_error: false,
                };
            }
            Command::Filter(filter_args) => {
                let scan_expr = util::filter_args_to_scan_expr(&filter_args);
                // Little weird to satisfy the borrow checker
                if let Some(mut addrs) = ctx.addrs.take() {
                    addrs.scan(ctx, &scan_expr);
                    ctx.addrs = Some(addrs);
                }
                return Message {
                    message: format!(
                        "scanner found {} addresses",
                        ctx.addrs.as_ref().unwrap().len()
                    ),
                    is_error: false,
                };
            }
            Command::Print => {
                util::print_addrs(ctx.addrs.as_mut().unwrap());
                return Message {
                    message: "".to_string(),
                    is_error: false,
                }
            }
            Command::Exit => {
                ctx.quit = true;
                return Message {
                    message: "".to_string(),
                    is_error: false,
                };
            }
            _ => panic!("Impossible command"),
        }
    }

    pub fn print(&mut self, msg: Message) {
        match msg.is_error {
            false => {
                if !msg.message.is_empty() {
                    println!("{}", msg.message);
                }
            }
            true => {
                println!("got error: {}", msg.message);
            }
        }
    }

    pub fn repl(&mut self) {
        let mut ctx = Context::new();
        while !ctx.quit {
            match self.read() {
                Some(line) => {
                    let cli = Cli::try_parse_from(line.split_whitespace());
                    if let Ok(cli) = cli {
                        let msg = self.eval(&cli.command, &mut ctx);
                        self.print(msg);
                    } else if let Err(e) = cli {
                        eprintln!("{}", e.render());
                    };
                }
                None => {
                    println!("WHYYY???");
                    break;
                }
            }
        }
    }
}
