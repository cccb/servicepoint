{
  description = "Flake for servicepoint-simulator";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-24.05";
    nix-filter.url = "github:numtide/nix-filter";
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs@{
      self,
      nixpkgs,
      naersk,
      nix-filter,
    }:
    let
      lib = nixpkgs.lib;
      supported-systems = [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
        "x86_64-darwin"
      ];
      forAllSystems = lib.genAttrs supported-systems;
      make-rust-toolchain-core =
        pkgs:
        pkgs.symlinkJoin {
          name = "rust-toolchain-core";
          paths = with pkgs; [
            rustc
            cargo
            rustPlatform.rustcSrc
          ];
        };
    in
    rec {
      packages = forAllSystems (
        system:
        let
          pkgs = nixpkgs.legacyPackages."${system}";
          rust-toolchain-core = make-rust-toolchain-core pkgs;
          naersk' = pkgs.callPackage naersk {
            cargo = rust-toolchain-core;
            rustc = rust-toolchain-core;
          };
          source = nix-filter.lib.filter {
             root = ./.;
             include = [
               ./Cargo.toml
               ./Cargo.lock
               ./crates
               ./Web437_IBM_BIOS.woff
               ./README.md
               ./LICENSE
             ];
           };
        in
        rec {
          servicepoint = naersk'.buildPackage rec {
            cargoBuildOptions =
              x:
              x
              ++ [
                "-p"
                "servicepoint"
              ];
            cargoTestOptions =
              x:
              x
              ++ [
                "-p"
                "servicepoint"
              ];
            src = source;
            doCheck = true;
            nativeBuildInputs = with pkgs; [
              pkg-config
              makeWrapper
            ];
            strictDeps = true;
            buildInputs = with pkgs; [
              xe
              lzma
            ];
          };
        }
      );

      legacyPackages = packages;

      devShells = forAllSystems (
        system:
        let
          pkgs = nixpkgs.legacyPackages."${system}";
          rust-toolchain = pkgs.symlinkJoin {
            name = "rust-toolchain";
            paths = with pkgs; [
              (make-rust-toolchain-core pkgs)
              rustfmt
              clippy
              cargo-expand
              cargo-tarpaulin
              gcc
              gnumake
              dotnet-sdk_8
            ];
          };
        in
        {
          default = pkgs.mkShell rec {
            inputsFrom = [ self.packages.${system}.servicepoint ];
            packages = [ rust-toolchain ];
            LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath (builtins.concatMap (d: d.buildInputs) inputsFrom)}";
            RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
          };
        }
      );

      formatter = forAllSystems (system: nixpkgs.legacyPackages."${system}".nixfmt-rfc-style);
    };
}
