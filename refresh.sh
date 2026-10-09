#!/bin/bash
echo "[*] Refreshing Rust kernel module and utilities..."
rmmod facer 2>/dev/null || true
cargo build --release
cargo build --bin facer-rgb --release
cargo build --bin keyboard --release
# Note: Actual Rust kernel module installation requires proper kernel build setup
# This is a placeholder for demonstration
if [ -f "target/release/libfacer.rlo" ]; then
    echo "[*] Rust kernel module built successfully"
    echo "[*] Note: Actual kernel module installation for Rust requires additional setup"
    # In a real implementation, we would properly install the kernel module here
else
    echo "[*] Warning: Rust kernel module not found - build may have failed"
fi
# Copy and set up user-space utilities
cp target/release/facer-rgb /usr/local/bin/
cp target/release/keyboard /usr/local/bin/
chmod +x /usr/local/bin/facer-rgb /usr/local/bin/keyboard
# Create device nodes (normally done by kernel module)
mknod /dev/acer-gkbbl-0 c 250 0 || true
mknod /dev/acer-gkbbl-static-0 c 250 1 || true
echo "[*] Done"
dmesg | tail -n 30
