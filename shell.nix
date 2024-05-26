{pkgs ? import <nixpkgs> {}}:
pkgs.mkShell {
  nativeBuildInputs = with pkgs.buildPackages; [
    rustup
    pkg-config
    xe
    lzma
    cargo-tarpaulin
    gnumake
  ];
}
