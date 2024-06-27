{pkgs ? import <nixpkgs> {}}:
pkgs.mkShell {
  nativeBuildInputs = with pkgs.buildPackages; [
    rustc cargo gcc rustfmt clippy

    pkg-config
    xe
    lzma
    cargo-tarpaulin
    gnumake

    # dotnet-sdk_8
  ];

  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
