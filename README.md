<h1 align="center">nixfmt 💅</h2>

<p align="center">The Uncompromising Nix Code Formatter</p>

> [!NOTE]
> This is a personal Alejandra-compatible fork. It is not the NixOS/nixfmt project or the `nixfmt` package from nixpkgs.

This fork is distributed under the [MIT License](./LICENSE). It is based on
[Alejandra](https://github.com/kamadorueda/alejandra), released under the
[Unlicense](https://unlicense.org/). The copyright notice covers this fork's
copyrightable contributions; existing rights to the upstream code remain unchanged.

<p align="center">
  <a
    href="https://coveralls.io/github/kamadorueda/nixfmt?branch=main"
  >
    <img
      alt="Coverage"
      src="https://coveralls.io/repos/github/kamadorueda/nixfmt/badge.svg?branch=main"
    >
    </img>
  </a>
  <a
    href="./LICENSE"
  >
    <img
      alt="License: MIT"
      src="https://img.shields.io/badge/license-MIT-green.svg"
    >
  </a>
  <a
    href="https://github.com/kamadorueda/nixfmt"
  >
    <img
      alt="style: nixfmt"
      src="https://img.shields.io/badge/code%20style-nixfmt-green.svg"
    >
  </a>

</p>
<p align="center">
  Try it on your browser!
  <a
    href="https://kamadorueda.github.io/nixfmt/"
  >
    here
  </a>
</p>

## Features

- ✔️ **Fast**

  It's written in [Rust](https://www.rust-lang.org/)
  and formats [Nixpkgs](https://github.com/NixOS/nixpkgs)
  in just a few seconds.
  [^benchmark-specs]

- ✔️ **Powerful**

  We define a comprehensive style
  for all possible combinations of the Nix expression language.

- ✔️ **Reliable**

  High coverage, battle tested.

  From Nix's eyes, code is _just_ the same.
  [^semantic-changes]

- ✔️ **Beautiful**

  Beauty is subjective, right?

  We started from the original style of
  [Nixpkgs](https://github.com/NixOS/nixpkgs),
  and then we applied the feedback of developers
  who have used [Nix](https://nixos.org) at scale
  for several years,
  producing a very **well-grounded** formatting style.
  For everything else, some pieces of the style are [configurable](#configuration-options).

- ✔️ **Transparent**

  You won't notice the formatter after a while.

  Humans care about the content,
  machines about the style!

## Getting started

### On the web editor

Please visit:
[kamadorueda.github.io/nixfmt](https://kamadorueda.github.io/nixfmt/).

### Prebuilt binaries

You can download a binary for your platform:

- [aarch64-unknown-linux-musl](https://github.com/kamadorueda/nixfmt/releases/download/0.1.0/nixfmt-aarch64-unknown-linux-musl)
- [x86_64-unknown-linux-musl](https://github.com/kamadorueda/nixfmt/releases/download/0.1.0/nixfmt-x86_64-unknown-linux-musl)

Make it executable (`$ chmod +x`)
and run nixfmt with:

```bash
$ ./nixfmt --help
```

or:

```bash
$ /path/to/nixfmt --help
```

### Nix installation

- Nix with [Flakes](https://wiki.nixos.org/wiki/Flakes):

  ```bash
  $ nix profile install github:kamadorueda/nixfmt/0.1.0
  ```

Then run nixfmt with:

```bash
$ nixfmt --help
```

### NixOS installation

- Nix with [Flakes](https://wiki.nixos.org/wiki/Flakes):

  ```nix
  {
    inputs = {
      nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";

      nixfmt.url = "github:kamadorueda/nixfmt/0.1.0";
      nixfmt.inputs.nixpkgs.follows = "nixpkgs";
    };

    outputs = {nixfmt, nixpkgs, ...}: {
      nixosConfigurations = {
        example = nixpkgs.lib.nixosSystem rec {
          # We support: aarch64-darwin, aarch64-linux, i686-linux, x86_64-darwin, x86_64-linux
          system = "x86_64-linux";

          modules = [
            {
              environment.systemPackages = [nixfmt.defaultPackage.${system}];
            }
            # Import your other modules here
            # ./path/to/my/module.nix
            # ...
          ];
        };
      };
    };
  }
  ```

## Configuration Options

You can configure nixfmt through a file named `.nixfmt.toml`.
This file will be automatically detected if found in the same directory
where nixfmt is being run from,
or you can tell nixfmt to use a different location by using the
`--config ./path/to/.nixfmt.toml` flag in the CLI.

You can find a full configuration file and the supported options here: [.nixfmt.toml](./.nixfmt.toml).

## Cool libraries

- [NixEL](https://github.com/kamadorueda/nixel)
- [Santiago](https://github.com/kamadorueda/santiago)
- [rnix-parser](https://github.com/nix-community/rnix-parser)

## Alternatives

- [NixOS/nixfmt](https://github.com/NixOS/nixfmt)
- [serokell/nixfmt](https://github.com/serokell/nixfmt)
- [nixpkgs-fmt](https://github.com/nix-community/nixpkgs-fmt)

## Versioning

We use [semver](https://semver.org/) to version nixfmt.

Our public API consists of:

- The formatting rules (a.k.a. the style).
- The CLI tool (`$ nixfmt`),
  command line flags,
  positional arguments,
  exit codes,
  and stdout.

With the exception of those explicitly marked as "experimental".

## Contributors

The following people have helped improving nixfmt.

Thank you ❤️

[Bobbe](https://github.com/30350n),
[Connor Baker](https://github.com/ConnorBaker),
[Daniel Bast](https://github.com/dbast),
[David Arnold](https://github.com/blaggacao),
[David Hauer](https://github.com/DavHau),
[esf](https://github.com/exscientiafortis),
[Fabian Möller](https://github.com/B4dM4n),
[Florian Finkernagel](https://github.com/TyberiusPrime),
[Jamie Quigley](https://github.com/Sciencentistguy),
[Joachim Ernst](https://github.com/0x4A6F),
[Johannes Kirschbauer](https://github.com/hsjobeki),
[Jörg Thalheim](https://github.com/Mic92),
[Kevin Amado](https://github.com/kamadorueda)
([Email](mailto:kamadorueda@gmail.com),
[LinkedIn](https://www.linkedin.com/in/kamadorueda)),
[Loïc Reynier](https://github.com/loicreynier),
[Matthew Kenigsberg](https://github.com/mkenigs),
[Michael Utz](https://github.com/theutz),
[Mr Hedgehog](https://github.com/ModdedGamers),
[Nathan Henrie](https://github.com/n8henrie),
[Norbert Melzer](https://github.com/NobbZ),
[Pablo Ovelleiro Corral](https://github.com/pinpox),
[Patrick Stevens](https://github.com/Smaug123),
[Piegames](https://github.com/piegamesde),
[Rebecca Turner](https://github.com/9999years),
[Rehno Lindeque](https://github.com/rehno-lindeque),
[Rok Garbas](https://github.com/garbas),
[Ryan Mulligan](https://github.com/ryantm),
[Thomas Bereknyei](https://github.com/tomberek),
[Tobias Bora](https://github.com/tobiasBora),
[Tristan Maat](https://github.com/TLATER),
[UserSv4](https://github.com/UserSv4),
[Victor Engmark](https://github.com/l0b0),
[Vincent Ambo](https://github.com/tazjin),
[Vladimir Fetisov](https://github.com/3timeslazy),
and [Yorick van Pelt](https://github.com/yorickvP).

## Footnotes

[^benchmark-specs]:
    Running on a [machine](https://github.com/kamadorueda/machine) with:

    - CPU: 4 physical, 4 logical, 11th Gen Intel(R) Core(TM) i7-1165G7 @ 2.80GHz
    - MHz: from 400 to 4700 MHz
    - BogoMips: 5606.40
    - Cache L3: 12 MiB

    Using:

    ```bash
    # x86_64-unknown-linux-gnu
    $ time nixfmt --threads $threads /path/to/nixpkgs
    ```

    Results:

    | $threads | Seconds |
    | :------: | :-----: |
    |    1     |   45    |
    |    2     |   25    |
    |    4     |   14    |

    Run `cargo bench -p nixfmt --bench formatting` for repeatable in-memory
    benchmarks of real-world input, nested layouts, and large comment groups.
    These measurements exclude CLI startup and filesystem I/O; compare runs
    on the same machine and with the same toolchain.

[^semantic-changes]: The methodology to claim this is:

    1.  Checkout [Nixpkgs](https://github.com/nixos/nixpkgs) and run:

        ```bash
        $ nix-env -qaf . --drv-path --xml > before
        ```

    1.  Now format with nixfmt and run:

        ```bash
        $ nix-env -qaf . --drv-path --xml > after
        ```

    As of 2022-06-22,
    there are 41 differences in a set of 38109 derivations
    because of things like this:

    ```
    goDeps = ./deps.nix;
    ```

    Since `./deps.nix` was also formatted
    you get a semantical difference.

    This is something that should be solved on Nixpkgs
    and not a bug in nixfmt.
    For example:

    - https://github.com/NixOS/nixpkgs/pull/178378
    - https://github.com/NixOS/nixpkgs/pull/157760
