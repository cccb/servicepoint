{
  description = "Flake for servicepoint-simulator";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-24.05";
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
          nativeBuildInputs = with pkgs; [
            pkg-config
            makeWrapper
          ];
          buildInputs = with pkgs; [
            xe
            lzma
          ];
          makeExample =
            package: example:
            naersk'.buildPackage {
              pname = example;
              cargoBuildOptions =
                x:
                x
                ++ [
                  "--package"
                  package
                ];
              src = ./.;
              nativeBuildInputs = nativeBuildInputs;
              strictDeps = true;
              buildInputs = buildInputs;
              overrideMain = old: {
                  preConfigure = ''
                    cargo_build_options="$cargo_build_options --example ${example}"
                  '';
                };
            };
          makePackage =
            package:
            let
              package-param = [
                "--package"
                package
              ];
            in
            naersk'.buildPackage {
              pname = package;
              cargoBuildOptions = x: x ++ package-param;
              cargoTestOptions = x: x ++ package-param;
              src = ./.;
              doCheck = true;
              nativeBuildInputs = nativeBuildInputs;
              strictDeps = true;
              buildInputs = buildInputs;
            };
        in
        rec {
          servicepoint = makePackage "servicepoint";
          announce = makeExample "servicepoint" "announce";
          game-of-life = makeExample "servicepoint" "game_of_life";
          moving-line = makeExample "servicepoint" "moving_line";
          random-brightness = makeExample "servicepoint" "random_brightness";
          wiping-clear = makeExample "servicepoint" "wiping_clear";
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