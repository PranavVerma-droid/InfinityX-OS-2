#!/bin/bash
set -e
mkdir -p build

nasm -f bin src/bootloader.asm -o build/bootloader.bin

qemu-system-x86_64 build/bootloader.bin 