#!/bin/bash
set -e

check_package() {
    dpkg-query -W -f='${Status}' "$1" 2>/dev/null | grep -q "ok installed"
}

declare -A PACKAGE_MAP=(
    ["build-essential"]="gcc"
    ["qemu-system-x86"]="qemu-system-x86_64"
    ["binutils"]="as"
    ["nasm"]="nasm"
    ["gcc"]="gcc"
    ["gdb"]="gdb"
    ["make"]="make"
    ["git"]="git"
    ["xorriso"]="xorriso"
    ["mtools"]="mcopy"
    ["curl"]="curl"
)

echo "Updating package lists..."
sudo apt-get update

echo "Installing packages..."
for pkg in "${!PACKAGE_MAP[@]}"; do
    if ! check_package "$pkg"; then
        echo "Installing $pkg..."
        sudo apt-get install -y "$pkg"
    else
        echo "$pkg already installed"
    fi
done

echo "Cleaning up..."
sudo apt-get autoremove -y
sudo apt-get clean

echo "Verifying installations..."
MISSING=0
for pkg in "${!PACKAGE_MAP[@]}"; do
    CMD="${PACKAGE_MAP[$pkg]}"
    if ! command -v "$CMD" >/dev/null 2>&1; then
        echo "⚠️ Warning: $pkg ($CMD) is not properly installed"
        echo "Attempting to fix..."
        sudo apt-get install --reinstall -y "$pkg"
        if ! command -v "$CMD" >/dev/null 2>&1; then
            MISSING=1
        fi
    fi
done

if [ $MISSING -eq 0 ]; then
    echo "All dependencies installed successfully!"
else
    echo "Some dependencies are still missing. Try running:"
    echo "sudo apt-get update && sudo apt-get install -y ${!PACKAGE_MAP[*]}"
fi