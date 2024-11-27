#!/bin/bash

cd ../src/

mkdir -p ../dist

nasm -f bin main.asm -o bootloader.bin

mv bootloader.bin ../dist/

cd ../dist/

# Create a blank hard disk image
dd if=/dev/zero of=os.img bs=512 count=2880

# Copy the boot sector into the hard disk image
dd if=bootloader.bin of=os.img conv=notrunc

echo "OS compiled successfully to ../dist/os.img"