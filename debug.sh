#!/bin/sh
cargo bootimage && qemu-system-x86_64 -drive format=raw,file=./target/x86_64-uwu_kernel/debug/bootimage-uwu-kernel.bin
