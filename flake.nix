{
  description = "Acer Predator Turbo and RGB Keyboard Linux Kernel Module (Rust Rewrite)";

  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
      lib = pkgs.lib;

      acerPredatorTurboRgb = pkgs.rustPlatform.buildRustPackage {
        pname = "acer-predator-turbo-rgb";
        version = "0.1.0";
        src = ./.;

        # Generate this file with: cargo generate-lockfile
        cargoLock.lockFile = ./Cargo.lock;

        meta = {
          description = "Acer Predator Turbo and RGB keyboard utilities";
          homepage = "https://github.com/HayyaSylvy/acer-rgb-turbo-keyboard-module-rust";
          license = lib.licenses.gpl3;
          platforms = lib.platforms.linux;
        };
      };
    in
    {
      packages.${system} = {
        default = acerPredatorTurboRgb;
        acer-predator-turbo-rgb = acerPredatorTurboRgb;
      };

      apps.${system} = {
        default = {
          type = "app";
          program = "${acerPredatorTurboRgb}/bin/facer-rgb";
        };

        facer-rgb = {
          type = "app";
          program = "${acerPredatorTurboRgb}/bin/facer-rgb";
        };

        keyboard = {
          type = "app";
          program = "${acerPredatorTurboRgb}/bin/keyboard";
        };
      };

      nixosModules.default = { config, lib, ... }:
        let
          cfg = config.services.acer-predator-turbo-rgb;
        in
        {
          options.services.acer-predator-turbo-rgb.enable =
            lib.mkEnableOption "Acer Predator Turbo and RGB keyboard utilities";

          config = lib.mkIf cfg.enable {
            environment.systemPackages = [ acerPredatorTurboRgb ];
            boot.kernelModules = [ "facer" ];
          };
        };
    };
}