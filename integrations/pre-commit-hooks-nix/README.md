# Pre-commit-hooks.nix integration

In order to use nixfmt with
[pre-commit-hooks.nix](https://github.com/cachix/pre-commit-hooks.nix)
use a configuration file like the following:

```nix
{
  pre-commit-check = pre-commit-hooks.lib.${system}.run {
    hooks = {
      nixfmt.enable = true;
    };
  };
}
```
