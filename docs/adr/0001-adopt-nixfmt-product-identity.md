# Adopt the nixfmt product identity for a personal fork

Status: Accepted

## Context

The project is being renamed from Alejandra to `nixfmt`, following the language-name-plus-`fmt` convention used by `rustfmt`. An existing NixOS/nixfmt project already uses that name, so the initial scope must distinguish this personal fork from an ecosystem-wide replacement.

## Decision

Rename all current public product identifiers to `nixfmt` without compatibility aliases. Rename the repository to `github.com/kamadorueda/nixfmt`, rename the CLI and Rust crates, and update Flake outputs, integrations, configuration keys, command IDs, documentation, metadata, and current URLs.

Keep Alejandra's formatter behavior, CLI arguments, exit codes, and formatted output unchanged. Start the renamed project at version `0.1.0`.

Treat the first release as a personal fork. Do not change nixpkgs, replace the existing NixOS/nixfmt implementation, or claim official formatter status.

## Consequences

- Existing `alejandra` commands, imports, package references, and integration settings stop working after migration.
- Users must update every exposed identifier in one breaking change.
- Existing formatting fixtures should continue to pass without output changes.
- Documentation must disambiguate this fork from NixOS/nixfmt.
- Historical records may keep the Alejandra name where rewriting them would misrepresent past releases.

## Acceptance criteria

- `nixfmt` is the only installed formatter executable from this repository.
- Rust packages and imports use `nixfmt`, `nixfmt_cli`, and `nixfmt_wasm`.
- Repository-owned integrations contain no active `alejandra` configuration keys, command IDs, executable paths, or current URLs.
- All package versions for the renamed release are `0.1.0`.
- Formatter snapshots and behavioral tests remain unchanged and pass.
- README wording states that this is a personal Alejandra-compatible fork and does not claim to replace nixpkgs' `nixfmt`.
