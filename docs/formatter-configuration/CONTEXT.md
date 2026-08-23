# Formatter Configuration

This context defines user-facing options that influence nixfmt's formatting output.

## Language

### Maximum line width

The preferred maximum number of columns in a formatted line.

### Soft limit

A maximum line width that the formatter should honor when a valid syntax-aware break opportunity exists. Unbreakable content may exceed the configured width.

### Break opportunity

A syntax-aware location where Alejandra can insert a line break without changing the meaning of the Nix expression.

### Input shape

Whether a syntax construct was written on one line or across multiple lines in the source input.

## Rules

- Maximum line width is a soft limit, not a guarantee that every output line fits within the configured width.
- When maximum line width is not configured, width-based line breaking is disabled.
- The first version exposes maximum line width only as the optional `max_width` field in the existing TOML configuration.
- The first version does not add a command-line override for maximum line width.
- TOML configuration is a stable feature rather than an experimental interface.
- Without an explicit configuration path, the CLI discovers `.nixfmt.toml` in the current directory and otherwise uses default formatting settings.
- Automatic discovery does not fall back to the former `nixfmt.toml` filename.
- Automatic discovery checks only the current working directory and does not search parent directories.
- Configurations outside the current working directory require `--config PATH`.
- The stable explicit-path option is `--config PATH`.
- The experimental `--experimental-config` option is removed without a compatibility alias.
- A missing automatically discovered configuration file uses default settings; an unreadable or invalid explicitly selected configuration exits with status code 1.
- `max_width` accepts only positive integers; omission disables width-based line breaking, while zero and negative values are configuration errors.
- `max_width` has no additional minimum or maximum bound.
- Width includes indentation and uses the formatter's existing column count, where each Unicode scalar value counts as one column.
- The first version does not calculate terminal display width for wide Unicode characters such as CJK text or emoji.
- The formatter must not alter program content solely to satisfy the maximum line width.
- Line breaking must use syntax-aware break opportunities.
- When `max_width` is enabled, it applies to every syntax construct that already has a safe multiline representation.
- Width-based formatting may add line breaks to the baseline formatted output but does not collapse baseline multiline structures into single lines.
- Content without a safe break opportunity, including strings and long identifiers, may exceed `max_width`.
- Comment text is not reflowed or split to satisfy `max_width`; long comments may exceed the limit.
- Lines that remain over `max_width` after all safe breaks are applied are kept without warnings or errors.
- An attribute set containing any input line break is normalized as a multiline attribute set.
- An attribute set containing no input line break remains single-line unless an enabled formatting constraint requires expansion.
- Input-shape preservation applies only to attribute sets; other syntax constructs retain their existing formatting rules.
