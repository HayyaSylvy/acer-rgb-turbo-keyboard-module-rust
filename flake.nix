{
  description = "Acer Predator Turbo and RGB Keyboard Linux Kernel Module (Rust Rewrite)";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };
  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = import nixpkgs { inherit system; };
  in {
    packages.${system}.default = pkgs.rustPlatform.buildRustPackage rec {
      pname = "acer-predator-turbo-rgb";
      version = "0.1.0";
      src = ./.;
      meta = with pkgs.lib; {
        description = "Acer Predator Turbo and RGB keyboard Linux kernel module utilities (Rust rewrite)";
        homepage = https://github.com/HayyaSylvy/acer-rgb-turbo-keyboard-module-rust;
        license = pkgs.lib.licenses.gpl3;
        maintainers = with pkgs.lib.maintainers; [ ];
        platforms = pkgs.lib.platforms.linux;
      };
    };
    packages.${system}.acer-predator-turbo-rgb = packages.${system}.default;
    apps.${system}.default = {
      facer-rgb = {
        type = "app";
        program = "${packages.${system}.default}/bin/facer-rgb";
      };
      keyboard = {
        type = "app";
        program = "${packages.${system}.default}/bin/keyboard";
      };
    };
  };
}
