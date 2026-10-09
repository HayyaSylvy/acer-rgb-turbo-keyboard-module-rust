{
  description = "Acer Predator Turbo and RGB Keyboard Linux Kernel Module (Rust Rewrite)";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };
  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = import nixpkgs { inherit system; };
    pkg = pkgs.rustPlatform.buildRustPackage rec {
      pname = "acer-predator-tubo-rgb";
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
  in
  {
    packages.${system}.default = pkg;
    packages.${system}.acer-predator-turbo-rgb = pkg;
    apps.${system}.default = {
      facer-rgb = {
        type = "app";
        program = "${pkg}/bin/facer-rgb";
      };
      keyboard = {
        type = "app";
        program = "${pkg}/bin/keyboard";
      };
    };
  };
}
