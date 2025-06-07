# memori
Low-level memory scanner for Linux, written in Rust. ~~and a debugger in the future~~

Warning! Still in early development.

## Compile
```shell
# The program uses nightly features so to compile first you need to switch to nightly
$ rustup default nightly

$ cargo build
# For the program to work properly you probably need to launch it with root privileges
# so it can attach to other processes
$ sudo ./target/debug/memori
```
