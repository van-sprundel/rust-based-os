## Rust-based OS

#### Linux
```cargo rustc -- -C link-arg=-nostartfiles```
#### Windows
```cargo rustc -- -C link-args="/ENTRY:_start /SUBSYSTEM:console"```
#### macOS
```cargo rustc -- -C link-args="-e __start -static -nostartfiles"```
