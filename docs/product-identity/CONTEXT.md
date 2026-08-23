# Product Identity

This context defines the names exposed by the personal formatter fork. It does not define formatting behavior, which remains Alejandra-compatible.

## Language

### nixfmt

The personal fork's product name and CLI command. The name follows the `<language>fmt` convention used by `rustfmt`.

### Alejandra-compatible formatting

The existing formatter behavior inherited from Alejandra. Renaming must not change formatting rules, CLI arguments, exit codes, or output.

### Complete rename

A breaking rename with no compatibility aliases. It covers the CLI, Rust crates and imports, Flake outputs, editor integrations, pre-commit integrations, documentation, configuration keys, command IDs, and current repository URLs.

### Personal fork

The initial distribution scope. It does not replace the NixOS/nixfmt implementation in nixpkgs and must not claim to be the official or canonical Nix formatter.

## Rules

- The repository is renamed from `kamadorueda/alejandra` to `kamadorueda/nixfmt`.
- Public identifiers use `nixfmt`, `nixfmt_cli`, or `nixfmt_wasm` as appropriate.
- The old `alejandra` command and package names are removed rather than retained as aliases.
- The renamed project starts a new version line at `0.1.0`.
- Historical changelog entries may retain the Alejandra name when describing releases that predate the fork.

## Scenarios

### Format a file

A user installs the personal fork and runs `nixfmt file.nix`. The result and exit status match the corresponding Alejandra invocation, apart from the executable name.

### Use an integration

An editor or pre-commit integration invokes `nixfmt` and exposes only `nixfmt`-named settings and commands.

### Encounter the existing nixfmt project

Documentation identifies this project as a personal Alejandra-compatible fork and does not imply that the existing NixOS/nixfmt project or nixpkgs package has been replaced.
