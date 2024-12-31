use crate::commands::ProcessArgs;
use crate::process::Process;

use std::io;

pub struct Context {
    pub quit: bool,
    pub process: Option<Process>,
}

impl Context {
    pub fn new() -> Context {
        Context {
            process: None,
            quit: false,
        }
    }

    pub fn process(&mut self, args: &ProcessArgs) -> io::Result<()> {
        match Process::try_new(args.pid) {
            Ok(proc) => {
                self.process = Some(proc);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}
