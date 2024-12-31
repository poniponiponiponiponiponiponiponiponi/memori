use crate::commands::{ProcessArgs, TypeArgs, ValType};
use crate::process::Process;
use crate::addresses::{Addresses, AddrsSimple};

use std::io;

pub enum CtxAddrs {
    None,
    AddrsI32(AddrsSimple<i32>),
    AddrsU32(AddrsSimple<u32>),
    AddrsI16(AddrsSimple<i16>),
    AddrsU16(AddrsSimple<u16>),
}

pub struct Context {
    pub quit: bool,
    pub process: Option<Process>,
    pub addrs: CtxAddrs,
}

impl Context {
    pub fn new() -> Context {
        Context {
            process: None,
            quit: false,
            addrs: CtxAddrs::None,
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
        match args.val_type {
            ValType::I32 => {
                self.addrs = CtxAddrs::AddrsI32(AddrsSimple::<i32>::new());
            },
            ValType::U32 => {
                self.addrs = CtxAddrs::AddrsU32(AddrsSimple::<u32>::new());
            },
            ValType::I16 => {
                self.addrs = CtxAddrs::AddrsI16(AddrsSimple::<i16>::new());
            },
            ValType::U16 => {
                self.addrs = CtxAddrs::AddrsU16(AddrsSimple::<u16>::new());
            },
        }
    }

    pub fn get_type(&self) -> String {
        match &self.addrs {
            CtxAddrs::None => "none".to_string(),
            CtxAddrs::AddrsI32(addrs) => addrs.get_type(),
            CtxAddrs::AddrsU32(addrs) => addrs.get_type(),
            CtxAddrs::AddrsI16(addrs) => addrs.get_type(),
            CtxAddrs::AddrsU16(addrs) => addrs.get_type(),
        }
    }
}
