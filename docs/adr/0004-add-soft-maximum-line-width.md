# Add a soft maximum line width

## Context

The formatter currently has no numeric line-width preference. Single-line and multiline layouts are selected by syntax-specific rules, so long breakable expressions may remain on one line.

## Decision

Add an optional positive-integer `max_width` field to the stable TOML configuration. Omission disables width-based line breaking. When enabled, the formatter may add syntax-aware line breaks across every construct that already has a safe multiline representation, but it does not collapse baseline multiline output.

Width is a soft limit. Indentation counts toward the limit, and the existing formatter column model counts each Unicode scalar value as one column. Strings, identifiers, comments, and other content without a safe break opportunity may exceed the limit. Remaining overflow does not produce a warning or error.

## Consequences

- Existing output remains unchanged when `max_width` is omitted, apart from separately specified attribute-set input-shape behavior.
- Configured formatting makes a best effort to keep breakable syntax within the preferred width.
- The first version requires no Unicode display-width dependency and does not reflow comments.
- A line may still exceed the configured width when satisfying it would require altering content or inventing an unsafe break.

## Acceptance criteria

- `max_width` accepts positive integers and rejects zero or negative values.
- Omitting `max_width` disables width-based line breaking.
- Width-driven formatting only adds line breaks at existing syntax-aware break opportunities.
- Indentation is included in the width calculation.
- Unbreakable overflow is preserved silently.
- Formatting is idempotent.
