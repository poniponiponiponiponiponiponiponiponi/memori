use crate::process::Process;

use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::PathBuf;
use std::mem;

pub trait MemoryReader {
    fn new(process: &Process) -> Self;
    fn read<T: Copy + FromLeBytes>(&mut self, addr: usize) -> T
    where
        [(); mem::size_of::<T>()]:;
}

pub struct MemoryReaderSimple {
    mem_file: File,
}

pub trait FromLeBytes: Sized {
    fn from_le_bytes(bytes: &[u8]) -> Self;
}

macro_rules! impl_from_le_bytes {
    ($type:ty) => {
        impl FromLeBytes for $type {
            fn from_le_bytes(bytes: &[u8]) -> Self {
                // Don't do this at home! The safe way of doing this would be:
                // `u32::from_le_bytes(bytes.try_into().unwrap())` but I want the
                // conversion to be as fast as possible since it can be a bottleneck,
                // so it's a major no-no for me. If you have a better safe way of
                // doing this lemme know.
                unsafe { *(bytes.as_ptr() as *const $type) }
            }
        }
    };
}

impl_from_le_bytes!(i128);
impl_from_le_bytes!(u128);
impl_from_le_bytes!(i64);
impl_from_le_bytes!(u64);
impl_from_le_bytes!(i32);
impl_from_le_bytes!(u32);
impl_from_le_bytes!(i16);
impl_from_le_bytes!(u16);
impl_from_le_bytes!(i8);
impl_from_le_bytes!(u8);

impl MemoryReader for MemoryReaderSimple {
    fn new(process: &Process) -> Self {
        let mem_path = PathBuf::from("/proc")
            .join(process.pid.to_string())
            .join("mem");
        let mem_file = File::open(mem_path)
            .expect("Should work because /proc/pid/maps was already read in Process::try_new()");
        Self { mem_file }
    }

    fn read<T: Copy + FromLeBytes>(&mut self, addr: usize) -> T
    where
        [(); mem::size_of::<T>()]:,
    {
        self.mem_file
            .seek(SeekFrom::Start(addr as u64))
            .expect("Unexpected error when seeking");

        let mut buffer = [0u8; mem::size_of::<T>()];
        // Fail silently when read is unsuccessful. This is for the
        // rare case when address stops existing between scans, for
        // example because a memory region got munmapped. We can check
        // instead on every read if it was done correctly, for example
        // by returning an Option<T>, but I want to keep the inner
        // scanning inner loop tight. Obviously this might end up with
        // some incorrect results.
        let _ = self.mem_file.read_exact(&mut buffer);

        T::from_le_bytes(&buffer)
    }
}

mod tests {
    use super::*;
    use std::process;

    #[test]
    fn memory_reader_simple_test() {
        let self_proc = Process::try_new(process::id()).unwrap();
        let mut mem_reader = MemoryReaderSimple::new(&self_proc);
        let a = 32;
        let a_addr = &a as *const i32;
        let ret = mem_reader.read::<i32>(a_addr as usize);
        assert_eq!(32, ret);
    }
}
