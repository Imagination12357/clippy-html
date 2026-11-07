# Current roadmap

Updated 2026-11-07 after official category implementation. This document is the current milestone/status index. DETAILS.md remains the full product specification with later approved design decisions applied; older plans and ledgers preserve execution history rather than current status.

## Product boundary

Clippy's structured diagnostics become portable, offline static HTML with rustdoc styling. Keep a single Cargo package, read-only source handling, relative links and working basic navigation without JavaScript. No linter, automatic fixes, backend or deployment service is added.

## Current baseline

| Area | Implemented | Remaining |
|---|---|---|
| Functional core | CLI, Cargo/Clippy collection, normalization, target deduplication, deterministic IDs, failure status, safe staged output | Continue regression coverage as features change |
| Report navigation | Overview/category/lint/source pages, online and bundled category data, readable URLs, file tree and finding links | Search |
| Source and diagnostics | Whole-source lexical syntax colors, line anchors/highlights, notes and grouped replacements | Precise primary/secondary ranges, suggestion diff/copy |
| Familiar presentation | Pinned rustdoc CSS/DOM, bundled fonts/notices, favicon | Light/Dark/Auto selection, keyboard and small-screen acceptance |
| Distribution | Local install smoke, packaged assets and notices, Rust 1.94 locked compile check | Cross-platform CI, browser/static-host checks, metadata export, release preparation |

Phase 1 functional implementation is complete. This is not a claim that every browser/platform acceptance check has passed. Interface work from Phase 2 and font/license work originally in Phase 3 were intentionally brought forward in user-directed follow-ups.

## Next milestones

### 0. Reconcile and compare — recorded checkpoint

- Synchronize current status in README/design and preserve older execution records as history.
- Compare this project's own rustdoc overview/item/source output against corresponding report output.
- Remove unnecessary custom behavior only where the reference and report requirements support the change.
- Keep diagnostic-specific UI, unified source sidebar/inset, readable report URLs and JS-free basic navigation.
- Record remaining visual/browser checks honestly; no continuing cosmetic work without a concrete problem.

### 1. Official categories — implemented

2026-11-03 policy, implemented 2026-11-07: fetch version-appropriate official Clippy category data online by default. Explicit tool --offline, or online retrieval failure, uses a bundled static table by lint name even if versions differ; record its source/version and fallback reason. Unknown remains for absent/unsupported entries, and category stays independent from severity. Generated reports still work offline. Category lists/pages and navigation to lint/source findings are implemented. See [category design and research](docs/superpowers/specs/2026-11-03-categories-design.md) and [validation](docs/validation/2026-11-07-categories.md). Actual browser acceptance remains manual.

**Done when:** known representative lints map to verified categories, unknown entries stay reachable, and category-to-lint-to-source navigation works offline with validated relative links.

### 2. Search — next product milestone

Search lint names, file paths, diagnostic messages and categories using report-local data. Use classic JavaScript without runtime fetch/module/CDN dependencies; preserve basic navigation when JS is unavailable. Escape generated data and create results with safe text/local links.

**Done when:** the user can find each supported result kind and reach its lint/source/category location from a file:// report, including Unicode inputs and empty/no-match cases.

### 3. Diagnostic understanding

Precise primary/secondary span presentation, then grouped suggestion before/after diff and copy. Preserve multipart suggestions as a unit; do not synthesize misleading diffs for stale source, invalid coordinates or overlapping edits. Keep original snippets/replacements as fallback; never modify the user's code.

**Done when:** Unicode/CRLF/multiline ranges and multipart suggestions have verified examples, and unsupported cases show an explicit useful fallback.

### 4. Themes and access

Light/Dark/Auto selection, page-to-page persistence where the browser allows it, keyboard navigation and small-screen review. Reuse appropriate pinned rustdoc styles without copying inactive toolbar controls or unrelated runtime infrastructure.

**Done when:** representative home/lint/category/source/search flows are usable by keyboard and on small screens in the supported themes; actual browser checks are recorded separately from DOM tests.

### 5. Metadata and distribution

Add opt-in --metadata <FILE>, schema/version and safe output-path handling. Verify installation, offline/static hosting under a repository subpath, Actions artifact and Pages guidance; prepare the release package and bundled notices. Publishing/pushing remains separately authorized.

**Done when:** explicit metadata export is tested without changing the default output contract, the support matrix is exercised, and install/hosting/package examples work.

## Verification throughout

- Bring the baseline Windows/Linux/macOS test matrix forward from the original final phase as an early follow-up, rather than discovering platform defects at release time. No cross-platform pass is claimed until CI actually runs.
- Run relevant tests at each feature checkpoint; preserve forwarding, exit status, escaping, stable IDs, source boundaries and safe output replacement.
- Compare meaningful UI changes to the same pinned rustdoc version and record why deviations remain.
- Browser review still needs actual Chrome/Edge and Firefox coverage of file://, long files, keyboard, narrow screens and static subpath hosting. Existing local-file automation restrictions do not count as a pass.

## Records

- [Original design](docs/superpowers/specs/2026-10-27-clippy-html-design.md)
- [Historical Phase 1 ledger](docs/superpowers/plans/phase-1-progress.md)
- [Current rustdoc comparison](docs/validation/2026-10-30-rustdoc-audit.md)
