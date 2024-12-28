#[derive(Debug)]
pub struct MemoryMap {
    pub addr_start: usize,
    pub addr_end: usize,
    pub perms: Permissions,
    pub offset: usize,
    pub dev: Device,
    pub inode: usize,
    pub pathname: String
}

#[derive(Debug)]
pub struct Permissions {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
    pub private: bool,
    pub shared: bool
}

#[derive(Debug)]
pub struct Device {
    pub major: i32,
    pub minor: i32,
}

impl MemoryMap {
    pub fn from(line: &str) -> MemoryMap {
        // example input from `man proc_pid_maps`:
        // address           perms offset  dev   inode       pathname
        // 00400000-00452000 r-xp 00000000 08:02 173521      /usr/bin/dbus-daemon
        let cols: Vec<_> = line.trim().split_whitespace().collect();

        if cols.len() < 5 {
            panic!("incorrect line supplied for MemoryMap");
        }

        let (addr_start, addr_end) = cols[0].split_once("-")
            .expect("incorrect input in memory maps");
        let addr_start = usize::from_str_radix(addr_start, 16)
            .expect("address is not a hex number");
        let addr_end = usize::from_str_radix(addr_end, 16)
            .expect("address is not a hex number");
        let perms = Permissions::from(&cols[1]);
        let offset = usize::from_str_radix(cols[2], 16)
            .expect("offset is not a hex number");
        let dev = Device::from(&cols[3]);
        let inode = cols[4].parse().expect("inode is not a number");
        let pathname = cols.get(5).unwrap_or(&"").to_string();

        MemoryMap {
            addr_start,
            addr_end,
            perms,
            offset,
            dev,
            inode,
            pathname
        }
    }
}

impl Permissions {
    fn from(perms: &str) -> Permissions {
        if perms.len() != 4 {
            panic!("incorrect permission string supplied");
        }
        Permissions {
            read: perms.as_bytes()[0] == b'r',
            write: perms.as_bytes()[1] == b'w',
            execute: perms.as_bytes()[2] == b'x',
            private: perms.as_bytes()[3] == b'p',
            shared: perms.as_bytes()[3] == b's',
        }
    }
}

impl Device {
    fn from(dev: &str) -> Device {
        let (major, minor) = dev.split_once(":").expect("bad device string supplied");
        let major = i32::from_str_radix(major, 16)
            .expect("major is not a hex number");
        let minor = i32::from_str_radix(minor, 16)
            .expect("minor is not a hex number");
        Device {
            major,
            minor
        }
    }
}
