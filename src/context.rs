use crate::process::Process;
use crate::commands::ProcessArgs;

pub struct Context {
    process: Option<Process>,
}

impl Context {
    pub fn new() -> Context {
        Context {
            process: None
        }
    }

    pub fn process(&mut self, args: &ProcessArgs) {
        match Process::try_new(args.pid) {
            Ok(proc) => {
                self.process = Some(proc);
            },
            Err(e) => {
            }
        }
    }
}
