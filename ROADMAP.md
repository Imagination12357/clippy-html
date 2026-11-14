# Current roadmap

Updated 2026-11-14 after diagnostic Summary implementation. This document is the current milestone/status index. DETAILS.md remains the full product specification with later approved design decisions applied; older plans and ledgers preserve execution history rather than current status.

## Product boundary

Clippy's structured diagnostics become portable, offline static HTML with rustdoc styling. Keep a single Cargo package, read-only source handling, relative links and working basic navigation without JavaScript. No linter, automatic fixes, backend or deployment service is added.

## Current baseline

| Area | Implemented | Remaining |
|---|---|---|
| Functional core | CLI, Cargo/Clippy collection, normalization, target deduplication, deterministic IDs, failure status, safe staged output | Continue regression coverage as features change |
| Report navigation | Overview/category/lint/source pages, online and bundled category data, readable URLs, file tree, finding links and report-local search | Browser acceptance of search |
| Source and diagnostics | Whole-source lexical syntax colors, precise primary/secondary ranges and insertion triangles, line anchors/highlights, notes and grouped replacements; unified suggestion diff/patch copy accepted by the user | Broader cross-browser and assistive-technology coverage |
| Familiar presentation | Pinned rustdoc CSS/DOM, bundled fonts/notices, favicon, native Settings structure and light/dark/ayu/system preference | Browser theme/keyboard/small-screen acceptance |
| Help and keyboard | Pinned rustdoc Help, layered Escape and input guards; diagnostic-only Summary/Show all and +/-/= commands | Browser acceptance of the new Summary view |
| Distribution | Local install/package verification, assets/notices, Rust 1.88 checks, cross-platform CI and self-report/Pages workflow; Pages smoke reported successful by user | Broader browser/static-host coverage, metadata export, release preparation |

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

### 2. Search — implemented; browser acceptance pending

Search lint names, file paths, diagnostic messages and categories using report-local data. Use classic JavaScript without runtime fetch/module/CDN dependencies; preserve basic navigation when JS is unavailable. Escape generated data and create results with safe text/local links.

2026-11-07: statically analyzed this project's pinned rustdoc output and adapted
its toolbar, in-place search screen, tabs, result rows and keyboard flow. The
report owns matching and its four-kind index. Rust/Node/DOM validation is in
[the search record](docs/validation/2026-11-07-search.md); actual file:// and
narrow-screen browser acceptance remain manual.

**Done when:** the user can find each supported result kind and reach its lint/source/category location from a file:// report, including Unicode inputs and empty/no-match cases.

### 3. Diagnostic understanding

Precise full-source ranges implemented 2026-11-07 and later refined to primary/
secondary backgrounds and insertion triangles, with unchanged-source validation. See
[range validation](docs/validation/2026-11-07-source-ranges.md). The user selected
Settings/theme work next, ahead of the remaining suggestion diff/copy work;
that port was implemented on 2026-11-09.

Precise primary/secondary span presentation, then grouped suggestion before/after diff and copy. Preserve multipart suggestions as a unit; do not synthesize misleading diffs for stale source, invalid coordinates or overlapping edits. Keep original snippets/replacements as fallback; never modify the user's code.

2026-11-11: A1 unified line diffs and B1 whole-suggestion patch copying are
implemented with source validation and all-or-none fallback. A2 separate
Before/After display is recorded only as a possible future option. See
[design](docs/superpowers/specs/2026-11-11-suggestion-diff-design.md) and
[validation](docs/validation/2026-11-11-suggestion-diff.md). Actual browser
clipboard/theme acceptance remains separate from local tests.

The user subsequently confirmed suggestion diff/copy layout and behavior as
working; screen-reader runtime behavior remains unverified. Do not repeat that
completed smoke acceptance as a new implementation task.

**Done when:** Unicode/CRLF/multiline ranges and multipart suggestions have verified examples, and unsupported cases show an explicit useful fallback.

### 4. Settings/themes — implemented; browser acceptance pending

Light/Dark/Ayu/system preference selection, preferred light/dark choices and
applicable native settings are implemented with the pinned radio/checkbox,
popover and standalone-page structure. Persistence follows browser capabilities;
storage denial still allows current-page use. No API-only inactive controls.
See [Settings design](docs/superpowers/specs/2026-11-09-settings-design.md) and
[validation](docs/validation/2026-11-09-settings.md). Actual browser theme,
file-URL, keyboard and narrow-screen acceptance remain manual.

**Done when:** representative home/lint/category/source/search flows are usable by keyboard and on small screens in the supported themes; actual browser checks are recorded separately from DOM tests.

### Help and keyboard — implemented; browser acceptance pending

2026-11-12: Help and keyboard coordination implemented separately from section
expand/collapse. See [design](docs/superpowers/specs/2026-11-12-help-shortcuts-design.md)
and [validation](docs/validation/2026-11-12-help-shortcuts.md).

2026-11-14: diagnostic-only Summary/Show all and - / + / = were implemented,
preserving summaries, sidebar/source/log independence and deep-link reveal.
No _ command is needed without impl blocks. See [design](docs/superpowers/specs/2026-11-14-diagnostic-summary-design.md)
and [validation](docs/validation/2026-11-14-diagnostic-summary.md). Browser
acceptance of this new view remains separate from earlier user confirmations.

### 5. Metadata and distribution

Add opt-in --metadata <FILE>, schema/version and safe output-path handling. Verify installation, offline/static hosting under a repository subpath, Actions artifact and Pages guidance; prepare the release package and bundled notices. Publishing/pushing remains separately authorized.

**Done when:** explicit metadata export is tested without changing the default output contract, the support matrix is exercised, and install/hosting/package examples work.

## Verification throughout

- The Windows/Linux/macOS stable test matrix, Linux 1.88 compile check, quality/DOM checks and online self-report/Pages workflow were configured on 2026-11-10. The user subsequently reported a successful Pages smoke; do not treat the initial CI setup as still unexecuted. See [CI validation](docs/validation/2026-11-10-ci.md) for local implementation evidence and initial history.
- Run relevant tests at each feature checkpoint; preserve forwarding, exit status, escaping, stable IDs, source boundaries and safe output replacement.
- Compare meaningful UI changes to the same pinned rustdoc version and record why deviations remain.
- Browser review still needs actual Chrome/Edge and Firefox coverage of file://, long files, keyboard, narrow screens and static subpath hosting. Existing local-file automation restrictions do not count as a pass.

## Records

- [Original design](docs/superpowers/specs/2026-10-27-clippy-html-design.md)
- [Historical Phase 1 ledger](docs/superpowers/plans/phase-1-progress.md)
- [Current rustdoc comparison](docs/validation/2026-10-30-rustdoc-audit.md)
