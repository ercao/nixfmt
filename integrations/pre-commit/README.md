# Pre-Commit integration

In order to use nixfmt with
[Pre-Commit](https://pre-commit.com/)
just create a file named `.pre-commit-config.yaml`
with contents:

```yaml
repos:
  - repo: https://github.com/kamadorueda/nixfmt
    rev: 0.1.0
    # Choose either the 'nixfmt' or 'nixfmt-system' hook
    # depending on what pre-requisites you have:
    hooks:
      # No prerequisites
      - id: nixfmt

      # Requires Nix to be previously installed in the system
      - id: nixfmt-nix

      # Requires nixfmt to be previously installed in the system
      - id: nixfmt-system
```

To use the latest hook, run `pre-commit autoupdate --freeze --repo=https://github.com/kamadorueda/nixfmt`.
