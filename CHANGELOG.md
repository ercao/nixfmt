# Changelog

All notable changes to nixfmt will be documented in this file.

## [0.1.0] - Unreleased

### Changed

- Renamed the personal Alejandra-compatible fork to nixfmt.
- Reset the project version line to 0.1.0.
- Renamed the CLI, Rust crates, Nix outputs, configuration file, and bundled integrations without compatibility aliases.
- Stabilized TOML configuration as `.nixfmt.toml` and renamed the explicit path option to `--config`.
- Added the optional soft `max_width` formatting preference.
- Preserved single-line and multiline input shape for attribute sets.

Other formatting behavior and exit codes remain unchanged.

The pre-fork Alejandra release history is preserved in [CHANGELOG-ALEJANDRA.md](./CHANGELOG-ALEJANDRA.md).

[0.1.0]: https://github.com/kamadorueda/nixfmt/releases/tag/0.1.0
