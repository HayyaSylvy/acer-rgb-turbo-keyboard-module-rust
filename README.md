# Acer Predator Turbo and RGB Keyboard Linux Kernel Module (Rust Rewrite)

[![GitHub license](https://img.shields.io/github/license/HayyaSylvy/acer-rgb-turbo-keyboard-module-rust)](https://github.com/HayyaSylvy/acer-rgb-turbo-keyboard-module-rust/blob/main/LICENSE)
[![GitHub release](https://img.shields.io/github/v/release/HayyaSylvy/acer-rgb-turbo-keyboard-module-rust)](https://github.com/HayyaSylvy/acer-rgb-turbo-keyboard-module-rust/releases)
[![GitHub issues](https://img.shields.io/github/issues/HayyaSylvy/acer-rgb-turbo-keyboard-module-rust)](https://github.com/HayyaSylvy/acer-rgb-turbo-keyboard-module-rust/issues)
[![GitHub stars](https://img.shields.io/github/stars/HayyaSylvy/acer-rgb-turbo-keyboard-module-rust)](https://github.com/HayyaSylvy/acer-rgb-turbo-keyboard-module-rust/stargazers)

An EXPERIMENTAL Rust rewrite of the [Acer Predator Turbo and RGB keyboard Linux kernel module](https://github.com/JafarAkhondali/acer-predator-turbo-and-rgb-keyboard-linux-module) that provides control for Acer Predator Turbo mode and RGB keyboard backlight via WMI interfaces.

## Overview

This project is a complete rewrite in Rust of the original C/Python implementation, providing the same functionality with improved safety, maintainability, and modern tooling. The rewrite includes:

- A Rust kernel module (using Rust for Linux bindings)
- Rust reimplementations of the `facer_rgb` and `keyboard` user-space utilities
- Proper build system with Cargo
- NixOS package support

## Features

- Turbo mode control
- RGB keyboard backlight control
- Multiple lighting effects: Static, Breathing, Neon, Wave, Shifting, Zoom and per-zone lighting control (4 zones)
- Adjustable speed, brightness, and direction
- Custom RGB colors
- Profile saving and loading
- Interactive keyboard utility
- NixOS package

## Installation

### Using NixOS (Recommended)

Add the following to your `configuration.nix`:

```nix
{
  environment.systemPackages = with pkgs; [
    acer-predator-turbo-rgb
  ];
  
  # Enable the kernel module
  boot.kernelModules = [ "facer" ];
  
  # Optional: Enable the service for persistent turbo button
  services.acer-predator-turbo-rgb.enable = true;
}
```

Then run:
```bash
sudo nixos-rebuild switch
```

#### Prerequisites

- Rust toolchain (via [rustup](https://rustup.rs/))
- Kernel headers for your running kernel
- `make`, `gcc`, `pkg-config`
- For systemd/openrc services: appropriate init system

#### Build and Install

```bash
# Clone the repository
git clone https://github.com/HayyaSylvy/acer-rgb-turbo-keyboard-module-rust.git
cd acer-predator-turbo-and-rgb-keyboard-linux-module-rust

# Build
cargo build --release

# Install kernel module (requires Rust for Linux kernel setup)
# Note: This requires a kernel with Rust support enabled
sudo make install

# Or install just the user-space utilities
sudo cp target/release/facer-rgb /usr/local/bin/
sudo cp target/release/keyboard /usr/local/bin/
sudo chmod +x /usr/local/bin/facer-rgb /usr/local/bin/keyboard
```

## Usage

### Controlling RGB Lighting

```bash
# List available profiles
facer-rgb --list

# Set breathing effect with purple color
facer-rgb -m 1 -s 4 -b 100 -cR 255 -cG 0 -cB 255

# Set static color for zone 1 (leftmost) to blue
facer-rgb -m 0 -z 1 -cR 0 -cB 255 -cG 0

# Save current settings as a profile
facer-rgb -m 1 -s 4 -b 100 -cR 255 -cG 0 -cB 255 --save mypurple

# Load a saved profile
facer-rgb --load mypurple
```

### Interactive Keyboard Utility

```bash
keyboard
```

This launches an interactive menu to control lighting effects without needing to remember command-line arguments.

## Compatibility

See the original [compatibility table](https://github.com/JafarAkhondali/acer-predator-turbo-and-rgb-keyboard-linux-module#will-this-work-on-my-laptop) for tested models.

## Building from Source

```bash
git clone https://github.com/HayyaSylvy/acer-rgb-turbo-keyboard-module-rust.git
cd acer-predator-turbo-and-rgb-keyboard-linux-module-rust
cargo build --release
```

The kernel module requires a kernel with Rust support. See the [Rust for Linux](https://rust-for-linux.com/) documentation for setup instructions.

## License

This project is licensed under the GPL-3.0 License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Original project: [JafarAkhondali/acer-predator-turbo-and-rgb-keyboard-linux-module](https://github.com/JafarAkhondali/acer-predator-turbo-and-rgb-keyboard-linux-module)
- Rust for Linux project
- Contributors to the original project 
