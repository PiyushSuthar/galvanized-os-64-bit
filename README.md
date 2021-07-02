# Galvanized OS [64 Bit]

> An OS Kernel made using Rust language!

### Why?

This repo was created to make my own OS kernel, and Learn Rust along with it too.

### Usage

This project uses cargo, Be sure to ensure you got that installed.

Compile commands for different hosts

```sh
# Linux
cargo rustc -- -C link-arg=-nostartfiles

# Windows
cargo rustc -- -C link-args="/ENTRY:_start /SUBSYSTEM:console"

# macOS
cargo rustc -- -C link-args="-e __start -static -nostartfiles"

# Bare metal
cargo build --target thumbv7em-none-eabihf
```

Download LLVM tools for running bootimage, and building bootloader

```sh
rustup component add llvm-tools-preview
```

To build for the target, This is the command.

```sh
cargo build
```

Running the Raw image file

```sh
qemu-system-x86_64 -drive format=raw,file=target/x86_64-blog_os/debug/bootimage-galvanized_os.bin

# Easier option added!
cargo run
```
