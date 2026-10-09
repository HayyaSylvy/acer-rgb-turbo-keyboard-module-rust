{
  description = "Acer Predator Turbo and RGB Keyboard Linux Kernel Module (Rust Rewrite)";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };
  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = import nixpkgs { inherit system; };
  in
  {
    packages.${system}.default = pkgs.rustPlatform.buildRustPackage rec {
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
    nixosModules.default = { config, pkgs, lib, ... }: let
      mypkg = pkgs.rustPlatform.buildRustPackage rec {
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
      options = {
        services.acer-predator-tubo-rgb.enable = pkgs.lib.mkEnableOption "Whether to enable the Acer Predator Turbo RGB keyboard utilities and kernel module.";
      };
      config = pkgs.lib.mkIf config.services.acer-predator-tubo-rgb.enable {
        environment.systemPackages = [ mypkg ];
        boot.kernelModules = [ "facer" ];
        # Note: The utilities (facer-rgb and keyboard) are designed for interactive or one-off use.
        # For persistent turbo-button functionality, you may need to create a systemd service that
        # loads the kernel module and then runs the utilities in a loop or via a daemon.
        # Refer to the provided install_service.sh and install_openrc.sh scripts in the repository
        # for examples of how to create such a service.
      };
    };
  };
}
