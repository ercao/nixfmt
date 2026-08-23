#! /usr/bin/env sh

set -eux

if ! command -v nix-build; then
  echo 'ERROR: this pre-commit hook requires "nix-build" to be installed first'
  exit 1
fi

echo INFO: building nixfmt

nix-build \
  --out-link result-nixfmt \
  https://github.com/kamadorueda/nixfmt/tarball/0.1.0

echo INFO: running nixfmt:
result-nixfmt/bin/nixfmt -- -q "${@}"
