#!/bin/bash

mkdir -p ../build

nasm -f bin ../src/bootloader.asm -o ../build/bootloader.bin

dd if=/dev/zero of=../build/os.img bs=1024 count=1440
dd if=../build/bootloader.bin of=../build/os.img conv=notrunc

echo "Build complete! Run using the Run Script."