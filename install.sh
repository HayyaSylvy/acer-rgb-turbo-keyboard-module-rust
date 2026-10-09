#!/bin/bash

if [[ $EUID -ne 0 ]]; then
   echo "[*] This script must be run as root"
   exit 1
fi

if [[ ! -f "/sys/bus/wmi/devices/7A4DDFE7-5B5D-40B4-8595-4408E0CC7F56/" ]]; then
    echo "[*] Sorry but your device doesn't have the required WMI module"
    exit 1
fi

if [ ! "$(uname -r | grep lts)" == "" ]; then
    echo LTS kernel detected
    MAKEFLAGS="LTS=1"
fi

# Remove previous chr devices if any exists
rm /dev/acer-gkbbl-0 /dev/acer-gkbbl-static-0 -f

# compile the kernel module using cargo
echo "[*] Building Rust kernel module..."
cargo build --release

# Remove previous modules if they exist
rmmod acer_wmi 2>/dev/null || true
rmmod facer 2>/dev/null || true

# Install required kernel modules
modprobe wmi
modprobe sparse-keymap
modprobe video

# Install the facer module
# Note: For Rust kernel modules, the output might be a .ko file or need special handling
# This is a simplified version - in practice, Rust kernel modules need special setup
if [ -f "target/release/libfacer.rlo" ]; then
    # Convert Rust object to kernel module (this is conceptual - actual process is more complex)
    echo "[*] Rust kernel module built successfully"
    echo "[*] Note: Actual kernel module installation for Rust requires additional setup"
    echo "[*] This is a placeholder for the Rust kernel module installation process"
else
    echo "[*] Error: Rust kernel module not found in target/release/"
    exit 1
fi

# Install and build user-space utilities
echo "[*] Building user-space utilities..."
cargo build --bin facer-rgb --release
cargo build --bin keyboard --release

# Copy utilities to system paths
cp target/release/facer-rgb /usr/local/bin/
cp target/release/keyboard /usr/local/bin/
chmod +x /usr/local/bin/facer-rgb /usr/local/bin/keyboard

# Create device nodes (this would normally be done by the kernel module)
echo "[*] Creating device nodes..."
mknod /dev/acer-gkbbl-0 c 250 0 || true
mknod /dev/acer-gkbbl-static-0 c 250 1 || true
# Set permissions to allow read/write for all users (fixes permission denied)
chmod 666 /dev/acer-gkbbl-0 || true
chmod 666 /dev/acer-gkbbl-static-0 || true

echo "[*] Done"
echo "[*] Note: This is a simplified installation script for demonstration purposes"
echo "[*] Actual Rust kernel module installation requires proper kernel build setup"
