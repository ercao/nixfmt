# Stabilize the configuration interface

## Context

The CLI already discovers a TOML file in the current directory, while an explicitly selected configuration file is exposed through the experimental `--experimental-config` option. Maximum line width will become an additional supported TOML setting.

## Decision

TOML configuration becomes a stable interface. The default file is `.nixfmt.toml` in the current directory. The explicit-path option is renamed to `--config PATH`. The old `--experimental-config` option is removed without a hidden alias or deprecation period.

## Consequences

- Users can rely on `.nixfmt.toml` and `--config` as stable interfaces.
- Scripts using `--experimental-config` must migrate immediately to `--config`.
- Existing `nixfmt.toml` files must be renamed to `.nixfmt.toml`.
- Help text and documentation no longer describe configuration as experimental.

## Acceptance criteria

- `.nixfmt.toml` in the current directory is loaded without an opt-in flag.
- Automatic discovery does not search parent directories.
- `nixfmt.toml` is not used as a compatibility fallback.
- `--config PATH` loads the selected TOML file.
- `--experimental-config` is rejected as an unknown option.
- A missing `.nixfmt.toml` uses default settings, while an unreadable or invalid explicit configuration exits with status code 1.
