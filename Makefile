# Makefile for Rust kernel module and utilities

.PHONY: all default clean install uninstall

# Default target
all: default

# Build the kernel module and user-space utilities
default: kernel-module utilities

# Build the kernel module using cargo
kernel-module:
	@echo "Building Rust kernel module..."
	@cargo build --release
	@echo "Kernel module built successfully!"

# Build the user-space utilities
utilities: facer-rgb keyboard

# Build the facer-rgb binary
facer-rgb:
	@echo "Building facer-rgb utility..."
	@cargo build --bin facer-rgb --release
	@echo "facer-rgb built successfully!"

# Build the keyboard binary
keyboard:
	@echo "Building keyboard utility..."
	@cargo build --bin keyboard --release
	@echo "keyboard built successfully!"

# Install the kernel module and utilities
install: kernel-module-install utilities-install

# Install the kernel module
kernel-module-install: kernel-module
	@echo "Installing kernel module..."
	@sudo cp target/release/libfacer.rlo /lib/modules/$(shell uname -r)/kernel/drivers/acpi/facer.ko
	@sudo depmod -a
	@echo "Kernel module installed successfully!"

# Install the user-space utilities
utilities-install: facer-rgb keyboard
	@echo "Installing utilities..."
	@sudo cp target/release/facer-rgb /usr/local/bin/
	@sudo cp target/release/keyboard /usr/local/bin/
	@sudo chmod +x /usr/local/bin/facer-rgb /usr/local/bin/keyboard
	@echo "Utilities installed successfully!"

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	@cargo clean
	@rm -f src/*.o src/*~ src/.*.cmd src/*.ko src/*.mod.c \
		.tmp_versions modules.order Module.symvers
	@echo "Cleaned successfully!"

# DKMS targets (adapted for Rust)
dkmsclean:
	@dkms remove facer/0.1 --all || true
	@dkms remove facer/0.2 --all || true

dkms: dkmsclean
	@echo "Setting up DKMS for Rust module..."
	@dkms add .
	@dkms install -m facer -v 0.1.0 --kernelver $(shell uname -r)

# Service installation targets
onboot:
	@echo "facer" > /etc/modules-load.d/facer.conf

noboot:
	@rm -f /etc/modules-load.d/facer.conf
