{ lib, rustPlatform, stdenv }:

rustPlatform.buildRustPackage rec {
  pname = "acer-predator-turbo-rgb";
  version = "0.1.0";

  src = ./.;

  meta = with lib; {
    description = "Acer Predator Turbo and RGB keyboard Linux kernel module utilities (Rust rewrite)";
    homepage = https://github.com/HayyaSylvy/acer-rgb-turbo-keyboard-module-rust;
    license = licenses.gpl3;
    maintainers = with maintainers; [ ];
    platforms = platforms.linux;
  };
}
