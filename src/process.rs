use crate::memory_map::MemoryMap;

use std::fs::File;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

pub struct Process {
    pid: usize,
    command: String,
    memory_maps: Vec<MemoryMap>,
}

impl Process {
    pub fn try_new(pid: usize) -> io::Result<Process> {
        let cmd_path = PathBuf::from("/proc").join(pid.to_string()).join("cmdline");
        let cmd_file = File::open(cmd_path)?;
        let content = io::read_to_string(cmd_file)?;
        let parts: Vec<&str> = content.trim_end_matches('\0').split('\0').collect();
        let command = parts.join(" ");

        let maps_path = PathBuf::from("/proc").join(pid.to_string()).join("maps");
        let maps_file = File::open(maps_path)?;
        let maps = io::read_to_string(maps_file)?;

        let memory_maps: Vec<_> = maps.lines().map(|l| MemoryMap::from(l)).collect();

        dbg!(&memory_maps);
        Ok(Process {
            pid,
            command,
            memory_maps
        })
    }
}
