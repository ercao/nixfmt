<h1 align="center">nixfmt 💅</h2>

<p align="center">The Uncompromising Nix Code Formatter</p>

## Features

This extension adds built-in editor support
for formatting Nix files automatically
with [nixfmt](https://github.com/kamadorueda/nixfmt).

## Getting started

1.  Make sure to install
    [nixfmt](https://github.com/kamadorueda/nixfmt)
    in your system first
    as explained [here](https://github.com/kamadorueda/nixfmt).

1.  Install the vscode extension and reload the window (just close and open again).

1.  Open a Nix file,
    do a right click
    and you should be able to see "Format Document" in the menu.

    Alternatively, it will be formatted automatically when you save the file.

Enjoy!

## Troubleshooting

If you encounter a problem
please let us know in the
[issues section](https://github.com/kamadorueda/nixfmt/issues).

The most probable causes of failure are:

- Not having nixfmt installed in your system.

  In this case please follow the instructions
  [here](https://github.com/kamadorueda/nixfmt).

- A misconfiguration.

  In this case please make sure that your config contains the following values:

  ```json
  {
    "[nix]": {
      "editor.defaultFormatter": "kamadorueda.nixfmt",
      "editor.formatOnPaste": true,
      "editor.formatOnSave": true,
      "editor.formatOnType": false
    },
    "nixfmt.program": "nixfmt"
  }
  ```
