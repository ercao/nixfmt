# Align trailing comments after baseline formatting

Trailing `#` comments are aligned in one rnix token pass after baseline formatting, only when `align_trailing_comments` is enabled. This adds one parse for opted-in formatting, but keeps alignment independent of individual syntax rules and reliably excludes `#` characters inside strings.

Ormolu and Fourmolu do not align arbitrary trailing-comment runs, while Brittany's incidental comment alignment depends on syntax-specific column blocks. We retain generic post-format grouping because it directly serves this option without adding column-layout logic to every syntax rule; the comparison is documented in [the Haskell formatter research](../formatter-configuration/research-haskell-comment-alignment.md).
