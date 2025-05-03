{
  description = "Flake for the servicepoint library.";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-24.11";
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
      forAllSystems =
        f:
        lib.genAttrs supported-systems (
          system:
          f rec {
            pkgs = nixpkgs.legacyPackages.${system};
            inherit system;
          }
        );
    in
    rec {
      packages = forAllSystems (
        { pkgs, ... }:
        let
          naersk' = pkgs.callPackage naersk { };
          nativeBuildInputs = with pkgs; [
            pkg-config
            makeWrapper
          ];
          buildInputs = with pkgs; [
            xe
            xz
          ];
          makeExample =
            {
              package,
              example,
              features ? "",
            }:
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
              inherit nativeBuildInputs buildInputs;
              strictDeps = true;
              gitSubmodules = true;
              overrideMain = old: {
                preConfigure = ''
                  cargo_build_options="$cargo_build_options --example ${example} ${
                    if features == "" then "" else "--features " + features
                  }"
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
              strictDeps = true;
              inherit nativeBuildInputs buildInputs;
            };
        in
        rec {
          servicepoint = makePackage "servicepoint";
          announce = makeExample {
            package = "servicepoint";
            example = "announce";
          };
          game-of-life = makeExample {
            package = "servicepoint";
            example = "game_of_life";
            features = "rand";
          };
          moving-line = makeExample {
            package = "servicepoint";
            example = "moving_line";
          };
          random-brightness = makeExample {
            package = "servicepoint";
            example = "random_brightness";
            features = "rand";
          };
          wiping-clear = makeExample {
            package = "servicepoint";
            example = "wiping_clear";
          };
        }
      );

      legacyPackages = packages;

      devShells = forAllSystems (
        { pkgs, system }:
        {
          default = pkgs.mkShell rec {
            inputsFrom = [ self.packages.${system}.servicepoint ];
            packages = with pkgs; [
              (pkgs.symlinkJoin
              {
                name = "rust-toolchain";
                paths = with pkgs; [
                  rustc
                  cargo
                  rustPlatform.rustcSrc
                  rustfmt
                  clippy
                  cargo-expand
                  cargo-tarpaulin
                  cargo-semver-checks
                  cargo-show-asm
                  cargo-flamegraph
                ];
              })
            ];
            LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath (builtins.concatMap (d: d.buildInputs) inputsFrom)}";
            RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
          };
        }
      );

      formatter = forAllSystems ({ pkgs, ... }: pkgs.nixfmt-rfc-style);
    };
}
