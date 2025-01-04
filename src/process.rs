use crate::memory_map::MemoryMap;

use std::fs::File;
use std::io::{self};
use std::path::PathBuf;

pub struct Process {
    pub pid: u32,
    pub command: String,
    pub memory_maps: Vec<MemoryMap>,
}

impl Process {
    pub fn try_new(pid: u32) -> io::Result<Process> {
        let cmd_path = PathBuf::from("/proc").join(pid.to_string()).join("cmdline");
        let cmd_file = File::open(cmd_path)?;
        let content = io::read_to_string(cmd_file)?;
        let parts: Vec<&str> = content.trim_end_matches('\0').split('\0').collect();
        let command = parts.join(" ");

        let maps_path = PathBuf::from("/proc").join(pid.to_string()).join("maps");
        let maps_file = File::open(maps_path)?;
        let maps = io::read_to_string(maps_file)?;

        let memory_maps: Vec<_> = maps.lines().map(MemoryMap::from).collect();

        Ok(Process {
            pid,
            command,
            memory_maps,
        })
    }
}
