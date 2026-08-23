# Preserve attribute-set input shape

## Context

nixfmt currently makes some attribute sets multiline based on structural formatting rules, even when the input attribute set was entirely on one line. The formatter is gaining an optional maximum line width, but width-based wrapping is disabled when that option is not configured.

## Decision

Attribute-set brace layout is input-sensitive. If top-level trivia between an attribute set's braces contains a line break, nixfmt normalizes the braces as multiline. Otherwise nixfmt does not expand the braces unless an enabled constraint such as maximum line width requires expansion. Line breaks inside nested syntax do not make the surrounding braces multiline, and nested syntax still follows its own formatting rules.

## Consequences

- Equivalent attribute sets may have different valid formatted layouts depending on their input shape.
- Formatting remains idempotent after the first pass.
- Existing attribute sets with top-level multiline layout remain multiline.
- Compact attribute-set braces are not expanded solely because of their nested structure.
- A configured maximum line width may still expand a compact attribute set.
- Lists, function arguments, `let/in` expressions, and other syntax constructs retain their existing formatting rules.

## Acceptance criteria

- A fully single-line attribute set keeps compact braces when no configured constraint requires expansion; nested syntax may still introduce line breaks according to its own rules.
- An attribute set containing a top-level trivia line break is formatted with multiline braces.
- Input-shape preservation does not change formatting rules for non-attribute-set constructs.
- Running the formatter twice produces the same output.
