{ pkgs ? import <nixpkgs> { } }:

with pkgs;

mkShell rec {
  nativeBuildInputs = [
    rustup
    pkg-config
    gnumake

    # find missing test cases
    cargo-tarpaulin
  ];
  buildInputs = [
    # servicepoint2 compression
    lzma xe

    # for bevy
    udev alsa-lib vulkan-loader
    # bevy x11
    xorg.libX11 xorg.libXcursor xorg.libXi xorg.libXrandr
    # bevy wayland
    libxkbcommon wayland
  ];
  LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
}
