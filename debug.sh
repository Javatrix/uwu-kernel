#!/bin/sh
cargo build && qemu-system-x86_64 -drive format=raw,file=./target/x86_64-my_os/debug/bootimage-os.bin
