## Rust-based OS

A rust based Operating System.

## Installation

Requirements:

- cargo
- rust (nightly)
- rustup
- Qemu

To set up your environment, run the following commands:

```commandline
rustup override set nightly
rustup component add rust-src
cargo install bootimage
rustup component add llvm-tools-preview
```

Your program should now be ready.

## Debug

This project is configured to run on [Qemu](https://www.qemu.org/) using ``cargo run``

## Runnable commands

#### Linux

```cargo rustc -- -C link-arg=-nostartfiles```

#### Windows

```cargo rustc -- -C link-args="/ENTRY:_start /SUBSYSTEM:console"```

#### macOS

```cargo rustc -- -C link-args="-e __start -static -nostartfiles"```
