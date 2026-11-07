# Precise source diagnostic ranges

**Goal:** On full-source pages, distinguish the exact primary/secondary text
ranges while preserving syntax colors, source text, finding links and fallback.
User approved implementation on local main; Settings follows separately.

**Architecture:** Keep source validation in source.rs, lexical classification in
highlight.rs, and introduce render/ranges.rs to split classified fragments at
validated diagnostic boundaries. pages.rs projects line/Unicode columns onto
each displayed line and supplies finding labels; Askama escapes every fragment.
No dependency, JavaScript or finding-ID change.

## Design / acceptance

- Primary solid underline, secondary dashed underline using existing rustdoc
  theme variables; retain subtle whole-line context and existing markers.
- Overlap: primary wins the visible style; tooltip retains distinct kind/code/
  labels. Repeated same-location spans keep primary if any occurrence is primary.
- Zero-byte span: a zero-text caret at its exact position, including EOL/EOF;
  provide a textual accessible label. A range ending at the next line's first
  column does not paint that next line. Empty interior lines retain row context.
- Use rustc's Unicode scalar columns only after byte/line/column and source
  validation. Display CRLF as existing normalized lines; tabs remain literal.
- Do not highlight invalid/stale source; preserve existing diagnostic snippets
  and fallback navigation. Do not treat replacement text as source content.
- Source text extracted from .line-text must round-trip unchanged (existing LF
  normalization), including HTML-looking strings. Lexical classes remain intact.
- Visible legend explains solid/dashed/caret, tooltips explain range ownership.
- This slice concerns full-source ranges; snippet presentation and suggestion
  diffs, theme/settings UI and precise-range deep links are separate work.

## Tasks

1. Projection and annotation → verify unit tests for overlapping/disjoint ranges,
   lexical splitting, tabs/Unicode, zero-width/EOL, multiline/CRLF and end-exclusive
   boundaries. Test before implementation; validate every span once per file.
2. Source template/CSS → verify parsed rendered primary/secondary/caret markup,
   escaped labels/text and retained line/finding links. Keep changes surgical.
3. Full tests + actual report → run Cargo tests, fmt, strict Clippy and existing
   Node tests; generate this project's offline preview. Fresh final review;
   fix material defects with regression tests, document limits, commit on main.

## Review focus

Zero-width spans at empty/final lines; Unicode/CRLF and tabs; endpoint at next
line column 1; overlapping diagnostics/primary promotion; stale/invalid source;
no accidental changes to token colors, copying source text or stable IDs.

## Execution record

Implementation starts at main 4222481. No branch/worktree or remote action.

Tasks 1–3 complete: annotation API initially missing (RED), then overlap/caret
tests GREEN; final suite 64 unit + 11 integration, existing Node suite 14/14,
fmt and strict Clippy pass. Actual offline preview and source reconstruction
check passed for 15 full-source files. Fresh independent review found no
material or deferred minor defect. Browser pixel/copy/accessibility acceptance
remains manual; later Settings/diff/deep-link work is outside this slice.
