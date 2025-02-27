#!/bin/bash
set -e

check_package() {
    dpkg-query -W -f='${Status}' "$1" 2>/dev/null | grep -q "ok installed"
}

declare -A PACKAGE_MAP=(
    ["qemu-system-x86"]="qemu-system-x86_64"
    ["build-essential"]="gcc"
)

echo "Updating package lists..."
sudo apt-get update

echo "Installing basic dependencies..."
for pkg in "${!PACKAGE_MAP[@]}"; do
    if ! check_package "$pkg"; then
        echo "Installing $pkg..."
        sudo apt-get install -y "$pkg"
    else
        echo "$pkg already installed"
    fi
done

if ! command -v rustc >/dev/null 2>&1; then
    echo "Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
else
    echo "Rust is already installed"
fi

echo "Installing required Rust components..."
rustup component add rust-src
rustup component add llvm-tools-preview
rustup override set nightly
rustup target add x86_64-unknown-none

echo "Installing cargo-bootimage..."
cargo install bootimage

echo "Cleaning up..."
sudo apt-get autoremove -y
sudo apt-get clean

echo "Verifying installations..."
MISSING=0

for pkg in "${!PACKAGE_MAP[@]}"; do
    CMD="${PACKAGE_MAP[$pkg]}"
    if ! command -v "$CMD" >/dev/null 2>&1; then
        echo "⚠️ Warning: $pkg ($CMD) is not properly installed"
        MISSING=1
    fi
done

if ! command -v rustc >/dev/null 2>&1; then
    echo "⚠️ Warning: Rust is not properly installed"
    MISSING=1
fi

if ! command -v cargo >/dev/null 2>&1; then
    echo "⚠️ Warning: Cargo is not properly installed"
    MISSING=1
fi

if [ $MISSING -eq 0 ]; then
    echo "All dependencies installed successfully!"
    echo "Rust version: $(rustc --version)"
    echo "Cargo version: $(cargo --version)"
    echo "QEMU version: $(qemu-system-x86_64 --version)"
else
    echo "Some dependencies are missing. Please check the error messages above."
    exit 1
fi