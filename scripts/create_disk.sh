#!/bin/bash

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Check if running as root
if [ "$EUID" -ne 0 ]; then
  echo -e "${RED}Please run as root (use sudo)${NC}"
  exit 1
fi

# Find the binary file
if [ -f "./target/x86_64-infinityx/release/bootimage-os.bin" ]; then
  BIN_FILE="./target/x86_64-infinityx/release/bootimage-os.bin"
  BIN_DIR="./target/x86_64-infinityx/release"
  echo -e "${GREEN}Found release binary: $BIN_FILE${NC}"
elif [ -f "./target/x86_64-infinityx/debug/bootimage-os.bin" ]; then
  BIN_FILE="./target/x86_64-infinityx/debug/bootimage-os.bin"
  BIN_DIR="./target/x86_64-infinityx/debug"
  echo -e "${YELLOW}Found debug binary: $BIN_FILE${NC}"
else
  echo -e "${RED}No bootimage-os.bin found. Please build the OS first with:${NC}"
  echo -e "cargo bootimage --release"
  exit 1
fi

# Ask user to choose between direct binary writing or ISO creation
echo -e "\n${BLUE}Choose writing method:${NC}"
echo -e "1) Direct binary writing (simple, might not be bootable on all systems)"
echo -e "2) Create bootable ISO first (more compatible with UEFI systems)"
read -p "Enter choice (1 or 2): " WRITE_METHOD

# For ISO method, ensure xorriso is installed
if [ "$WRITE_METHOD" = "2" ]; then
  echo -e "\n${BLUE}Checking for xorriso...${NC}"
  if ! command -v xorriso &> /dev/null; then
    echo -e "${YELLOW}xorriso not found. Installing...${NC}"
    apt-get update && apt-get install -y xorriso
  fi
  
  echo -e "\n${BLUE}Creating bootable ISO...${NC}"
  ISO_FILE="infinityx-os.iso"
  
  # Copy the binary to current directory temporarily for ISO creation
  cp "$BIN_FILE" "./bootimage-os.bin"
  
  # Create ISO
  xorriso -as mkisofs -b bootimage-os.bin -no-emul-boot -boot-load-size 4 \
    -o "$ISO_FILE" .
  
  # Remove temporary binary file
  rm "./bootimage-os.bin"
  
  echo -e "${GREEN}ISO created: $ISO_FILE${NC}"
  
  # The image to write will now be the ISO
  WRITE_FILE="$ISO_FILE"
else
  # For direct binary writing
  WRITE_FILE="$BIN_FILE"
fi

# List available drives
echo -e "\n${YELLOW}Available drives:${NC}"
lsblk -d -o NAME,SIZE,MODEL,VENDOR | grep -v loop

# Ask user to select a drive
echo -e "\n${RED}WARNING: ALL DATA ON THE SELECTED DEVICE WILL BE ERASED!${NC}"
read -p "Enter the device name to write to (e.g., sdb, sdc): " DEVICE

# Validate device name format
if ! [[ $DEVICE =~ ^[a-z]+$ ]]; then
  echo -e "${RED}Invalid device name. Please use format like 'sdb' or 'sdc'.${NC}"
  exit 1
fi

# Full device path
USB_DEVICE="/dev/${DEVICE}"

# Check if the device exists
if [ ! -b "$USB_DEVICE" ]; then
  echo -e "${RED}Device $USB_DEVICE does not exist!${NC}"
  exit 1
fi

# Confirm the user's choice
echo -e "${RED}You are about to erase ALL DATA on $USB_DEVICE${NC}"
lsblk "$USB_DEVICE"
read -p "Are you ABSOLUTELY sure you want to continue? (Type 'YES' to confirm): " CONFIRM

if [ "$CONFIRM" != "YES" ]; then
  echo -e "${YELLOW}Operation cancelled.${NC}"
  exit 0
fi

# Unmount any mounted partitions
MOUNTED_PARTITIONS=$(mount | grep "$USB_DEVICE" | awk '{print $1}')
if [ ! -z "$MOUNTED_PARTITIONS" ]; then
  echo "Unmounting partitions..."
  for partition in $MOUNTED_PARTITIONS; do
    echo "Unmounting $partition"
    umount "$partition" || true
  done
fi

echo -e "\n${YELLOW}Writing image to USB drive...${NC}"
echo -e "This may take a few minutes. Please be patient."

# Write the image to the USB drive
dd if="$WRITE_FILE" of="$USB_DEVICE" bs=4M status=progress conv=fsync

# Sync to ensure all data is written
sync

# Clean up ISO if it was created
if [ "$WRITE_METHOD" = "2" ] && [ -f "$ISO_FILE" ]; then
  echo -e "\n${BLUE}Cleaning up temporary ISO file...${NC}"
  rm "$ISO_FILE"
fi

echo -e "\n${GREEN}USB drive creation complete!${NC}"
echo -e "\nTo boot from this USB drive:"
echo -e "1. Insert the USB drive into the target computer"
echo -e "2. Power on the computer and enter the boot menu (typically F12, F2, or DEL)"
echo -e "3. Select the USB drive from the boot menu"

if [ "$WRITE_METHOD" = "1" ]; then
  echo -e "\n${YELLOW}Note: If the system doesn't boot, try using method 2 next time.${NC}"
fi

echo -e "\n${YELLOW}Note: The OS expects a VGA-compatible system and may not work on all hardware.${NC}"