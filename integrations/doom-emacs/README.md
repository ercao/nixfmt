# Doom Emacs integration

In order to configure nixfmt in
[Doom Emacs](https://github.com/hlissner/doom-emacs)
just use the following:

```lisp
(after! nix-mode
  (set-formatter! 'nixfmt '("nixfmt" "--quiet") :modes '(nix-mode)))
```

If you've enabled formatting via LSP in Nix,
you might also need to add the following:

```lisp
(setq-hook! 'nix-mode-hook +format-with-lsp nil)
```
