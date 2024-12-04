#!/bin/bash

mkdir -p ../build

nasm -f bin ../src/bootloader.asm -o ../build/bootloader.bin
dd if=/dev/zero of=../build/os.img bs=1024 count=20480

# 20MB file
dd if=../build/bootloader.bin of=../build/os.img conv=notrunc

echo "Build complete! Enhanced bootloader with kernel loading capability."