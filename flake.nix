{
  description = "The Uncompromising Nix Code Formatter";

  inputs = {
    fenix.url = "github:nix-community/fenix";
    fenix.inputs.nixpkgs.follows = "nixpkgs";

    flakeCompat.url = "github:edolstra/flake-compat";
    flakeCompat.flake = false;

    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable-small";
  };

  outputs = inputs: let
    commit = inputs.self.shortRev or "dirty";
    date = inputs.self.lastModifiedDate or inputs.self.lastModified or "19700101";
    version = "0.1.0+${builtins.substring 0 8 date}.${commit}";

    nixpkgsForHost = host:
      import inputs.nixpkgs {
        overlays = [overlay];
        system = host;
      };

    nixpkgs."aarch64-darwin" = nixpkgsForHost "aarch64-darwin";
    nixpkgs."aarch64-linux" = nixpkgsForHost "aarch64-linux";
    nixpkgs."i686-linux" = nixpkgsForHost "i686-linux";
    nixpkgs."x86_64-darwin" = nixpkgsForHost "x86_64-darwin";
    nixpkgs."x86_64-linux" = nixpkgsForHost "x86_64-linux";

    fenix."x86_64-linux" = inputs.fenix.packages."x86_64-linux";

    overlay = final: prev: {
      nixfmt = final.rustPlatform.buildRustPackage {
        pname = "nixfmt";
        inherit version;
        src = ./.;
        cargoLock.lockFile = ./Cargo.lock;

        passthru.tests = {
          version = final.testVersion {package = final.nixfmt;};
        };

        meta = {
          description = "The Uncompromising Nix Code Formatter.";
          homepage = "https://github.com/kamadorueda/nixfmt";
          license = final.lib.licenses.mit;
          maintainers = [final.lib.maintainers.kamadorueda];
          platforms = final.lib.systems.doubles.all;
          mainProgram = "nixfmt";
        };
      };
    };

    buildBinariesForHost = host: pkgs: let
      binaries = builtins.listToAttrs (
        builtins.map (pkg: {
          name = "nixfmt-${pkg.stdenv.targetPlatform.config}";
          value = pkg;
        })
        pkgs
      );
    in
      binaries
      // {
        "nixfmt-binaries" = nixpkgs.${host}.linkFarm "nixfmt-binaries" (
          nixpkgs.${host}.lib.mapAttrsToList
          (name: binary: {
            inherit name;
            path = "${binary}/bin/nixfmt";
          })
          binaries
        );
        "default" = builtins.elemAt pkgs 0;
      };
  in rec {
    checks."aarch64-darwin" = packages."aarch64-darwin";
    checks."aarch64-linux" = packages."aarch64-linux";
    checks."i686-linux" = packages."i686-linux";
    checks."x86_64-darwin" = packages."x86_64-darwin";
    checks."x86_64-linux" = packages."x86_64-linux";

    defaultPackage."aarch64-darwin" = packages."aarch64-darwin"."nixfmt-aarch64-apple-darwin";
    defaultPackage."aarch64-linux" = packages."aarch64-linux"."nixfmt-aarch64-unknown-linux-gnu";
    defaultPackage."i686-linux" = packages."i686-linux"."nixfmt-i686-unknown-linux-gnu";
    defaultPackage."x86_64-darwin" = packages."x86_64-darwin"."nixfmt-x86_64-apple-darwin";
    defaultPackage."x86_64-linux" = packages."x86_64-linux"."nixfmt-x86_64-unknown-linux-gnu";

    devShell."x86_64-linux" = with nixpkgs."x86_64-linux";
      mkShell {
        name = "nixfmt";
        packages = [
          cargo-bloat
          cargo-license
          cargo-tarpaulin
          jq
          inputs.fenix.packages."x86_64-linux".latest.rustfmt
          inputs.fenix.packages."x86_64-linux".stable.toolchain
          nodejs
          nodePackages.prettier
          perf
          shfmt
          treefmt
          yarn
          yarn2nix
        ];
      };

    inherit overlay;
    overlays.default = overlay;

    packages."aarch64-darwin" = with nixpkgs."aarch64-darwin";
      buildBinariesForHost "aarch64-darwin" [
        nixfmt
      ];
    packages."aarch64-linux" = with nixpkgs."aarch64-linux";
      buildBinariesForHost "aarch64-linux" [
        nixfmt
        pkgsStatic.nixfmt
      ];
    packages."i686-linux" = with nixpkgs."i686-linux";
      buildBinariesForHost "i686-linux" [
        nixfmt
      ];
    packages."x86_64-darwin" = with nixpkgs."x86_64-darwin";
      buildBinariesForHost "x86_64-darwin" [
        nixfmt
      ];
    packages."x86_64-linux" = with nixpkgs."x86_64-linux";
      (buildBinariesForHost "x86_64-linux" [
        nixfmt
        pkgsStatic.nixfmt

        pkgsCross.aarch64-multiplatform.pkgsStatic.nixfmt
        # Temporarily disabled to speed up release
        # pkgsCross.armv7l-hf-multiplatform.pkgsStatic.nixfmt
        # pkgsCross.gnu32.pkgsStatic.nixfmt
        # pkgsCross.raspberryPi.pkgsStatic.nixfmt
      ])
      // {
        "nixfmt-vscode-vsix" = mkYarnPackage {
          name = "nixfmt";
          src = ./integrations/vscode;
          packageJSON = ./integrations/vscode/package.json;
          yarnLock = ./integrations/vscode/yarn.lock;
          yarnNix = ./integrations/vscode/yarn.lock.nix;
        };
        "build-wasm" = writeShellApplication {
          name = "build-wasm";
          runtimeInputs = [
            (fenix."x86_64-linux".combine [
              fenix."x86_64-linux".latest.rustc
              fenix."x86_64-linux".latest.toolchain
              fenix."x86_64-linux".targets."wasm32-unknown-unknown".latest.rust-std
            ])
            binaryen
            pkg-config
            openssl
            wasm-pack
          ];
          text = ''
            cd src/nixfmt_wasm
            wasm-pack build --target web
          '';
        };
      };
  };
}
