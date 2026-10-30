# Roadmap Reconciliation and Rustdoc Audit

**Goal:** Record the approved revised roadmap and compare current rustdoc/report output before the next feature milestone.

**Approved scope:** Preserve completed Phase 1 work and early interface work; prioritize category then search. Compare this project's own generated rustdoc, adopt worthwhile simplifications, retain justified diagnostic-specific differences. No requirement for identical pages or rustdoc JavaScript.

**Inputs:** DETAILS.md; original design/Phase 1 plan and follow-up records; current templates/CSS; freshly generated rustdoc 1.97.1 and the report for this repository.

**Files:** ROADMAP.md is the current roadmap; README.md and the original design's status/roadmap point to it. docs/validation/2026-10-30-rustdoc-audit.md records paired pages and decisions. assets/clippy-html.css loses redundant version sizing and inherited source white-space overrides. src/render/highlight.rs receives only the reproduced underscore classification correction.

## Execution

- [x] Reconcile original milestones with the current implementation and remaining verification.
- [x] Regenerate native docs; inspect overview/item/source HTML, native source-tree JS, CSS rules and font files/declarations against report output.
- [x] Write an underscore keyword regression and observe failure; add only the missing lexical class.
- [x] Remove redundant CSS overrides, preserving common table columns and the user-approved source-tree inset.
- [x] Update current roadmap/status and document comparison findings, including retained lexical differences and browser limitations.
- [x] Run tests, fmt, strict Clippy, regenerate both outputs, repeat asset/source/link comparison and perform independent review; prepare the verified change for commit.

Results: 47 unit + 9 integration tests passed; fmt and strict Clippy passed. Nine paired source texts and all 12 selected font declarations/binaries matched; only recorded qualified-macro prefix coloring differs in this source corpus. Independent review found no Important/Critical code or scope issues; requested final status/evidence placeholders have been filled. Browser/platform acceptance remains explicitly pending. See docs/validation/2026-10-30-rustdoc-audit.md.

## Limits

This is generated DOM/CSS/asset/source comparison, not a browser pixel comparison. No file-URL browser policy workaround. Do not implement category, search, theme, CI or metadata in this audit. This task adds no dependency and changes no license policy.
