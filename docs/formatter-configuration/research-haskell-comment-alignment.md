# Haskell formatter trailing-comment alignment research

Research date: 2026-08-25. Sources are pinned to the checked repository commits:

- Ormolu: `d5727c0718b540a828cfff1cd629afbb47768956`
- Fourmolu: `2b4f97a655eab4e8aefc69cacc48469441fc5291`
- Brittany: `e03ab8425bbc5a3171808cee3285480f64d21536`

Every claim below is marked either **Confirmed** or **Not confirmed**. A
confirmed claim links to an official repository at a pinned commit and names
the source file, symbol or golden test, and line range used as evidence.

## Result summary

- **Ormolu — Confirmed:** it does not align an arbitrary consecutive run of
  trailing comments. Each same-line comment is associated with an AST element
  and emitted after one requested separator. Ormolu has no formatting-style
  configuration.
- **Fourmolu — Confirmed:** it retains Ormolu's comment-emission mechanism and
  has no generic trailing-comment run grouping or comment-alignment option.
  Its `column-limit` default is `none`.
- **Brittany — Confirmed:** it has no independent trailing-comment alignment
  pass. Its syntactic column blocks can make comments align as a consequence
  of aligning the surrounding syntax. Its grouping unit is an AST-created
  `BDCols` block, not a textually consecutive run of lines.
- **All three — Not confirmed:** none of the inspected first-party sources
  specifies terminal-cell-width handling for wide Unicode glyphs. The source
  does confirm `Text.length`-based accounting and ASCII-space padding.

## Ormolu

### Placement and grouping — confirmed

- A following comment is recognized by source-span and AST-enclosure relationships. Same-line comments are registered as `OnTheSameLine`; multiline block comments are printed immediately after `space` instead ([`Printer/Comments.hs` lines 79-98](https://github.com/tweag/ormolu/blob/d5727c0718b540a828cfff1cd629afbb47768956/src/Ormolu/Printer/Comments.hs#L79-L98), [`Printer/Comments.hs` lines 182-257](https://github.com/tweag/ormolu/blob/d5727c0718b540a828cfff1cd629afbb47768956/src/Ormolu/Printer/Comments.hs#L182-L257)).
- Pending same-line comments are flushed by requesting `space`; the `space` primitive guarantees at most one separator, and `spit` realizes it with either zero or one ASCII space ([`Printer/Internal.hs` lines 270-332](https://github.com/tweag/ormolu/blob/d5727c0718b540a828cfff1cd629afbb47768956/src/Ormolu/Printer/Internal.hs#L270-L332), [`Printer/Internal.hs` lines 334-366](https://github.com/tweag/ormolu/blob/d5727c0718b540a828cfff1cd629afbb47768956/src/Ormolu/Printer/Internal.hs#L334-L366)). There is no collection of neighboring source lines and no maximum-prefix calculation in this path.
- The golden output contains adjacent trailing comments at different columns, for example `bar $ -- bar` and the following `baz -- baz` ([`infix/comments-out.hs` lines 1-3](https://github.com/tweag/ormolu/blob/d5727c0718b540a828cfff1cd629afbb47768956/data/examples/declaration/value/function/infix/comments-out.hs#L1-L3)). This rules out a generic consecutive-line alignment policy.

### Configuration and width — confirmed

- Ormolu describes its style as one style with no configuration ([`README.md` lines 35-45](https://github.com/tweag/ormolu/blob/d5727c0718b540a828cfff1cd629afbb47768956/README.md#L35-L45)); the design document explicitly says it does not try to fit reasonable line lengths and does not allow formatter configuration ([`DESIGN.md` lines 302-319](https://github.com/tweag/ormolu/blob/d5727c0718b540a828cfff1cd629afbb47768956/DESIGN.md#L302-L319)). Therefore there is no comment-alignment default and no width fallback for comment padding.

### Block comments, Unicode, and tabs

- **Confirmed:** a same-line multiline `{- ... -}` comment bypasses the
  pending-comment path but still receives one `space` before immediate output
  ([official source: `src/Ormolu/Printer/Comments.hs`, symbol
  `spitFollowingComment`, lines 92-98](https://github.com/tweag/ormolu/blob/d5727c0718b540a828cfff1cd629afbb47768956/src/Ormolu/Printer/Comments.hs#L92-L98)).
- **Confirmed:** output column tracking adds `Data.Text.length`, while generated
  indentation and separators use ASCII spaces
  ([official source: `src/Ormolu/Printer/Internal.hs`, symbol `spit`, lines
  284-297](https://github.com/tweag/ormolu/blob/d5727c0718b540a828cfff1cd629afbb47768956/src/Ormolu/Printer/Internal.hs#L284-L297)).
- **Not confirmed:** the inspected official documentation does not promise
  terminal-cell-width alignment for East Asian wide glyphs, emoji, or tabs.
  There is no trailing-comment alignment padding in Ormolu to which such a
  promise could apply.

## Fourmolu

### Placement and grouping — confirmed

- Fourmolu's printer follows the same mechanism: pending comments are documented as being inserted before the next newline, with a space inserted when they follow code on the same line ([`Printer/Internal.hs` lines 579-590](https://github.com/fourmolu/fourmolu/blob/2b4f97a655eab4e8aefc69cacc48469441fc5291/src/Ormolu/Printer/Internal.hs#L579-L590)). Its low-level printer generates zero or one separator and advances the column by `Text.length` ([`Printer/Internal.hs` lines 296-324](https://github.com/fourmolu/fourmolu/blob/2b4f97a655eab4e8aefc69cacc48469441fc5291/src/Ormolu/Printer/Internal.hs#L296-L324)).
- Its Fourmolu golden output also leaves adjacent trailing comments at different columns, `bar $ -- bar` followed by `baz -- baz` ([`infix/comments-four-out.hs` lines 1-3](https://github.com/fourmolu/fourmolu/blob/2b4f97a655eab4e8aefc69cacc48469441fc5291/data/examples/declaration/value/function/infix/comments-four-out.hs#L1-L3)). There is no generic consecutive-line alignment group.

### Configuration and width — confirmed

- The generated configuration source defines `column-limit` as the maximum line length for automatic line breaking and gives it the default `NoLimit` ([`ConfigData.hs` lines 50-70](https://github.com/fourmolu/fourmolu/blob/2b4f97a655eab4e8aefc69cacc48469441fc5291/config/FourmoluConfig/ConfigData.hs#L50-L70)); the checked default configuration serializes this as `column-limit: none` ([`fourmolu.yaml` lines 1-7](https://github.com/fourmolu/fourmolu/blob/2b4f97a655eab4e8aefc69cacc48469441fc5291/fourmolu.yaml#L1-L7)). The complete option registry contains no comment-alignment field ([`ConfigData.hs` `allOptions`, lines 50-290](https://github.com/fourmolu/fourmolu/blob/2b4f97a655eab4e8aefc69cacc48469441fc5291/config/FourmoluConfig/ConfigData.hs#L50-L290)).
- `column-limit` affects the single-line versus multi-line layout choice by comparing the source span length with the configured limit; one helper deliberately ignores the limit where enforcing it would require changing the AST ([`Printer/Combinators.hs` lines 180-218](https://github.com/fourmolu/fourmolu/blob/2b4f97a655eab4e8aefc69cacc48469441fc5291/src/Ormolu/Printer/Combinators.hs#L180-L218)). No source adds or removes comment padding in response to this limit because the printer does not perform trailing-comment alignment.

### Block comments, Unicode, and tabs

- **Confirmed:** Fourmolu's generated whitespace is ASCII space and output
  columns use `Text.length`
  ([official source: `src/Ormolu/Printer/Internal.hs`, symbol `spit`, lines
  311-324](https://github.com/fourmolu/fourmolu/blob/2b4f97a655eab4e8aefc69cacc48469441fc5291/src/Ormolu/Printer/Internal.hs#L311-L324)).
- **Not confirmed:** the inspected official documentation does not define
  terminal display-cell semantics for wide Unicode glyphs or tabs, and there
  is no generated trailing-comment padding whose tab policy is configurable.

## Brittany

### Placement and grouping — confirmed

- Brittany has a general column-layout node, `BDCols`; individual syntactic layouters create these nodes explicitly, for example type signatures and bind statements ([`Layouters/Decl.hs` lines 90-139](https://github.com/lspitzner/brittany/blob/e03ab8425bbc5a3171808cee3285480f64d21536/source/library/Language/Haskell/Brittany/Internal/Layouters/Decl.hs#L90-L139), [`LayouterBasics.hs` lines 475-476](https://github.com/lspitzner/brittany/blob/e03ab8425bbc5a3171808cee3285480f64d21536/source/library/Language/Haskell/Brittany/Internal/LayouterBasics.hs#L475-L476)). Therefore its groups are syntactic column blocks, not arbitrary consecutive text lines.
- Comments are annotations emitted after the associated document using their exact-print delta positions ([`Backend.hs` lines 255-289](https://github.com/lspitzner/brittany/blob/e03ab8425bbc5a3171808cee3285480f64d21536/source/library/Language/Haskell/Brittany/Internal/Backend.hs#L255-L289)). Column-width calculation recurses through annotation wrappers without adding the comment text to the measured document width ([`Backend.hs` lines 323-349](https://github.com/lspitzner/brittany/blob/e03ab8425bbc5a3171808cee3285480f64d21536/source/library/Language/Haskell/Brittany/Internal/Backend.hs#L323-L349)). Consequently, Brittany's column machinery does not define a separate "longest code prefix then align comments" operation.
- A fixed-point test fixture shows three pattern-synonym equations whose code and comments end at common columns ([`Test250.hs` lines 3-6](https://github.com/lspitzner/brittany/blob/e03ab8425bbc5a3171808cee3285480f64d21536/data/Test250.hs#L3-L6)). The test harness formats every file and requires byte-for-byte equality with the fixture ([`source/test-suite/Main.hs` lines 8-30](https://github.com/lspitzner/brittany/blob/e03ab8425bbc5a3171808cee3285480f64d21536/source/test-suite/Main.hs#L8-L30)). This confirms that such aligned output is stable, but the source above shows it arises from the surrounding syntactic column layout rather than a comment-only grouping pass.

### Configuration and width — confirmed

- Default layout settings are 80 columns, `ColumnAlignModeMajority 0.7`, an alignment limit of 30 spaces, and breaking alignment when an item becomes multiline ([`Config.hs` lines 45-57](https://github.com/lspitzner/brittany/blob/e03ab8425bbc5a3171808cee3285480f64d21536/source/library/Language/Haskell/Brittany/Internal/Config.hs#L45-L57)).
- The configuration type documents `alignmentLimit` as an upper bound on spaces inserted for horizontal alignment and `alignmentBreakOnMultiline` as disabling an alignment when items do not remain single-line ([`Config/Types.hs` lines 41-76](https://github.com/lspitzner/brittany/blob/e03ab8425bbc5a3171808cee3285480f64d21536/source/library/Language/Haskell/Brittany/Internal/Config/Types.hs#L41-L76)).
- During rendering, `ColumnAlignModeDisabled` selects the unaligned path; unanimous and majority modes accept or reject the aligned path according to the configured column maximum and ratio ([`Backend.hs` lines 655-725](https://github.com/lspitzner/brittany/blob/e03ab8425bbc5a3171808cee3285480f64d21536/source/library/Language/Haskell/Brittany/Internal/Backend.hs#L655-L725)). These are general syntax-column rules, not comment-specific controls.

### Block comments, Unicode, and tabs

- **Confirmed:** the annotation renderer handles comment text generically after
  splitting it into lines; this path has no distinct line-comment versus block-
  comment alignment branch
  ([official source: `source/library/Language/Haskell/Brittany/Internal/Backend.hs`,
  `layoutBriDocM` annotation branch, lines 270-289](https://github.com/lspitzner/brittany/blob/e03ab8425bbc5a3171808cee3285480f64d21536/source/library/Language/Haskell/Brittany/Internal/Backend.hs#L270-L289)).
- **Confirmed:** Brittany measures literal and external text using
  `Data.Text.length`
  ([official source: `source/library/Language/Haskell/Brittany/Internal/Backend.hs`,
  symbol `briDocLineLength`, lines 323-349](https://github.com/lspitzner/brittany/blob/e03ab8425bbc5a3171808cee3285480f64d21536/source/library/Language/Haskell/Brittany/Internal/Backend.hs#L323-L349))
  and emits positioning whitespace with repeated ASCII spaces
  ([official source: `source/library/Language/Haskell/Brittany/Internal/BackendUtils.hs`,
  symbol `layoutWriteEnsureAbsoluteN`, lines 176-190](https://github.com/lspitzner/brittany/blob/e03ab8425bbc5a3171808cee3285480f64d21536/source/library/Language/Haskell/Brittany/Internal/BackendUtils.hs#L176-L190)).
- **Not confirmed:** the official sources inspected do not specify terminal
  display-cell width for East Asian wide glyphs, emoji, or tabs.

## Direct mapping to nixfmt's proposed `align_trailing_comments`

Facts established by these implementations:

1. Ormolu and Fourmolu support the proposed feature only as negative references: their comment paths normalize a trailing comment to a single separator and do not form textual alignment groups.
2. Brittany supplies reusable constraints for alignment generally: cap added padding, stop aligning multiline items, and reject alignment that exceeds the configured column budget. Its grouping mechanism cannot be copied directly because it depends on syntax-specific column blocks rather than a uniform post-format trailing-comment pass.
3. All three inspected implementations use code-point counts (`Text.length`) and ASCII spaces for generated positioning. This matches the proposed nixfmt choice to reuse its existing column model and space-only padding, including the same limitation for wide Unicode glyphs.
4. None of the three provides a first-party precedent for all of the proposed nixfmt semantics together: consecutive same-indent textual grouping, minimum group size two, default-off `align_trailing_comments`, and all-or-nothing rollback when padding alone crosses `max_width`.

## Differences requiring a product decision

- Whether nixfmt should keep the proposed generic post-format run grouping, or restrict alignment to syntax-aware groups as Brittany does.
- Whether the width response should remain all-or-nothing per group, or adopt Brittany-like bounded padding and per-item fallback.
- Whether block comments stay excluded. Ormolu/Fourmolu place same-line block comments, and Brittany renders comments generically, but none provides a dedicated block-comment alignment policy.
