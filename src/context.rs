use crate::addresses::{Addresses, AddrsSimple};
use crate::commands::{ProcessArgs, TypeArgs, ValType};
use crate::memory_reader::{MemoryReader, MemoryReaderSimple};
use crate::process::Process;

use std::io;

pub struct Context {
    pub quit: bool,
    pub process: Option<Process>,
    pub addrs: Option<Box<dyn Addresses>>,
}

impl Context {
    pub fn new() -> Context {
        Context {
            process: None,
            quit: false,
            addrs: None,
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

    pub fn change_type(&mut self, args: &TypeArgs) {
        let proc = &self.process.as_ref().unwrap();
        match args.val_type {
            ValType::I32 => {
                self.addrs = Some(Box::new(AddrsSimple::<i32, MemoryReaderSimple>::new(proc)));
            }
            ValType::U32 => {
                self.addrs = Some(Box::new(AddrsSimple::<u32, MemoryReaderSimple>::new(proc)));
            }
            ValType::I16 => {
                self.addrs = Some(Box::new(AddrsSimple::<i16, MemoryReaderSimple>::new(proc)));
            }
            ValType::U16 => {
                self.addrs = Some(Box::new(AddrsSimple::<u16, MemoryReaderSimple>::new(proc)));
            }
        }
    }

    pub fn get_type(&self) -> String {
        match &self.addrs {
            None => "none".to_string(),
            Some(addrs) => addrs.get_type(),
        }
    }
}
