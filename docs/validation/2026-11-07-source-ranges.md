# Precise full-source diagnostic ranges

## Contract and implementation

Approved on local main from 4222481. Plan/design:
../superpowers/plans/2026-11-07-source-ranges.md. No new dependency, JavaScript,
source modification, license change or diagnostic identity change.

Coordinate reference: https://doc.rust-lang.org/rustc/json.html#diagnostics
specifies 0-based half-open bytes, 1-based Unicode scalar columns with exclusive
end, and primary flags. Existing source.rs validates bytes, character boundaries,
line/column agreement and original snippet text before any range is projected.

render/ranges.rs separates annotation from lexical classification. It splits
fragments at token and span boundaries, preserves original text/classes, retains
distinct overlap descriptions and prioritizes primary styling. pages.rs projects
validated columns onto normalized displayed lines; a next-line column-1 endpoint
does not paint that line. Identical locations promote primary while retaining
labels. Source template escapes labels/text; zero-text carets have accessible
labels. CSS uses existing theme foreground and solid/dashed patterns with a
visible legend; carets paint with an absolute pseudo-element to avoid shifting
subsequent source columns. Existing row context and finding links remain.

Snippets continue to show the original plain diagnostic text and labels; this
slice adds exact styling to full-source pages. Changed/unavailable/invalid source
uses the existing snippet fallback. Empty interior lines/newline-only portions
have row context rather than fabricated character ranges. Settings remains next.

## Verification

- Initial range tests failed because the new annotation API was absent.
- 64 Rust unit + 11 integration tests pass. New cases cover lexical splitting,
  primary/secondary overlap and label retention, Unicode, tabs, CRLF/multiline,
  next-line exclusive end, empty final-line caret, primary promotion, stale
  source fallback, escaped HTML-looking source/labels and accessible caret markup.
- Existing long-source test still verifies each span is validated once per file.
- Real Clippy fixture verifies an exact primary range around clone rather than
  the surrounding declaration; source round-trip and existing links still pass.
- All 14 existing Node engine/DOM tests pass; search code was not changed.
- cargo fmt --check and strict Clippy pass.
- Actual report: cargo run --locked -- --offline --output
  target/source-ranges-preview -- --offline --all-targets -- -W clippy::pedantic;
  exit 0 with staged link/asset/anchor validation. Parsed all 15 full-source
  pages and reconstructed their .line-text contents exactly against repository
  text with existing CRLF normalization. This report has 848 primary annotation
  fragments and 2 insertion carets; it has no secondary spans. Secondary and
  overlap behavior are covered by synthetic renderer tests, not claimed as
  observed in this real project's Clippy run.
- Fresh independent final review found no Critical, Important or actionable
  Minor defect. It inspected code/tests without rerunning the coordinator's
  suites. It explicitly left pixel/clipboard/caret hit-testing/screen-reader
  acceptance to browser checks; Settings, suggestion diff/copy and precise-range
  deep links remain separate work. Existing source validation was inspected for
  regressions, not expanded into a broader hardening project.

## Browser boundary

User follow-up: the actual preview's 15 source pages have no secondary ranges,
so no dashed-range location exists to link. Static comparison confirmed the
primary/secondary underline scheme is report-specific, not part of the pinned
rustdoc stylesheet. The user found 2px underlines too heavy; reduced underline
thickness to 1px, retaining solid/dashed distinction and the insertion caret.

No browser was launched and no local-file policy workaround was used. Parsed
HTML proves text/classes/links, not the underline's pixel appearance, copy/paste
behavior, font metrics or screen-reader announcements. Manual checks should cover
primary/secondary styles, overlapping tooltips, insertion/EOL carets, tabs,
long-line scrolling and narrow screens. No cross-browser pass is claimed.
