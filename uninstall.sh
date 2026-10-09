#!/bin/bash
if [[ $EUID -ne 0 ]]; then
   echo "[*] This script must be run as root"
   exit 1
fi

echo "[*] Removing Rust kernel module..."
rmmod facer 2>/dev/null || true

echo "[*] Restoring original acer_wmi module..."
modprobe acer_wmi 2>/dev/null || true

echo "[*] Cleaning up device nodes..."
rm -f /dev/acer-gkbbl-0 /dev/acer-gkbbl-static-0

echo "[*] Removing user-space utilities from /usr/local/bin..."
rm -f /usr/local/bin/facer-rgb /usr/local/bin/keyboard

echo "[*] Done"
