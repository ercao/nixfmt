{
  description = "The Uncompromising Nix Code Formatter";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable-small";
  };

  outputs = inputs: let
    cargoPackage = (fromTOML (builtins.readFile ./crates/nixfmt_cli/Cargo.toml)).package;

    forAllSystems = inputs.nixpkgs.lib.genAttrs [
      "aarch64-darwin"
      "aarch64-linux"
      "i686-linux"
      "x86_64-darwin"
      "x86_64-linux"
    ];

    overlay = final: prev: {
      nixfmt = final.rustPlatform.buildRustPackage {
        pname = "nixfmt";
        inherit (cargoPackage) version;
        src = final.lib.fileset.toSource {
          root = ./.;
          fileset = final.lib.fileset.unions [
            ./.nixfmt.toml
            ./Cargo.toml
            ./Cargo.lock
            ./crates/nixfmt
            ./crates/nixfmt_cli
          ];
        };
        cargoLock.lockFile = ./Cargo.lock;
        cargoBuildFlags = [ "-p" "nixfmt_cli" ];

        passthru.tests = {
          version = final.testers.testVersion { package = final.nixfmt; };
        };

        meta = {
          description = "The Uncompromising Nix Code Formatter.";
          homepage = "https://github.com/ercao/nixfmt";
          license = final.lib.licenses.mit;
          maintainers = [ final.lib.maintainers.kamadorueda ];
          platforms = final.lib.systems.doubles.all;
          mainProgram = "nixfmt";
        };
      };
    };
  in rec {
    checks = builtins.mapAttrs (_: pkgs:
      pkgs
      // {
        version = pkgs.default.tests.version;
      })
    packages;
    defaultPackage = builtins.mapAttrs (_: pkgs: pkgs.default) packages;

    inherit overlay;
    overlays.default = overlay;

    packages = forAllSystems (system: let
      pkgs = import inputs.nixpkgs {
        inherit system;
        overlays = [ overlay ];
      };
    in {
      default = pkgs.nixfmt;
      "nixfmt-${pkgs.stdenv.targetPlatform.config}" = pkgs.nixfmt;
    });
  };
}
