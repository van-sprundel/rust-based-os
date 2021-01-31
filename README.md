## Rust-based OS
A rust based Operating System. \
This project is configured to run on [Qemu](https://www.qemu.org/) using ``cargo run``

### Runnable commands
#### Linux
```cargo rustc -- -C link-arg=-nostartfiles```
#### Windows
```cargo rustc -- -C link-args="/ENTRY:_start /SUBSYSTEM:console"```
#### macOS
```cargo rustc -- -C link-args="-e __start -static -nostartfiles"```
