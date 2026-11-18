# Roadmap

## Open work

- Choose a release version and write the changelog.
- Check the report in Chrome or Edge and Firefox. Cover local file URLs, keyboard
  navigation, narrow windows, long source files, and hosting under a repository
  subpath. Include search with Unicode input and no matches, Settings across themes,
  and Summary controls on lint and source pages.
- Check diagnostic navigation and controls with assistive technology. The
  suggestion diff and copy flow passed a manual smoke check. Screen-reader
  behavior still needs testing.

## Implemented

| Area | What works |
|---|---|
| Report generation | Collect and deduplicate Clippy diagnostics, preserve failure status, and validate staged reports before replacement. |
| Navigation | Browse by category, lint, and source file. Search locally by lint name, path, message, or category. |
| Diagnostics | Highlight validated ranges and show insertion markers. Group multipart replacements into a single patch with a snippet fallback. |
| Interface | Bundle rustdoc styles and fonts, offer light, dark, and Ayu themes, and keep basic navigation available without JavaScript. Help lists keyboard shortcuts and Summary folds diagnostic details. |
| Distribution | Export optional navigation JSON. The packaged source passed a Rust 1.88 compile check and a Windows release install that generated HTML and JSON. |

CI runs tests on Windows, Linux, and macOS, an MSRV compile check on Linux, and
the Rust and Node checks described in the contributor guide. The repository's
self-report has passed a Pages smoke check. DOM tests cover interaction logic.
Use browsers for the remaining layout and local-file checks. [Contributor guide](CONTRIBUTING.md) · [CI validation](docs/validation/2026-11-10-ci.md)

## Design and verification records

- [Original design](docs/superpowers/specs/2026-10-27-clippy-html-design.md),
  [Phase 1 ledger](docs/superpowers/plans/phase-1-progress.md), and
  [rustdoc comparison](docs/validation/2026-10-30-rustdoc-audit.md)
- [Categories](docs/superpowers/specs/2026-11-03-categories-design.md) and
  [category tests](docs/validation/2026-11-07-categories.md)
- [Search](docs/validation/2026-11-07-search.md) and
  [source ranges](docs/validation/2026-11-07-source-ranges.md)
- [Suggestion diffs](docs/superpowers/specs/2026-11-11-suggestion-diff-design.md)
  and [patch tests](docs/validation/2026-11-11-suggestion-diff.md). A separate
  before/after view remains an option for later.
- [Settings design](docs/superpowers/specs/2026-11-09-settings-design.md) and
  [Settings tests](docs/validation/2026-11-09-settings.md)
- [Help design](docs/superpowers/specs/2026-11-12-help-shortcuts-design.md) and
  [keyboard tests](docs/validation/2026-11-12-help-shortcuts.md)
- [Summary design](docs/superpowers/specs/2026-11-14-diagnostic-summary-design.md)
  and [Summary tests](docs/validation/2026-11-14-diagnostic-summary.md)
- [Metadata schema](docs/metadata.md),
  [metadata design](docs/superpowers/specs/2026-11-16-navigation-metadata-design.md),
  and [package verification](docs/validation/2026-11-18-package-public-docs.md)
