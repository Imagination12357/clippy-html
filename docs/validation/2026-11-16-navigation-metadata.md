# Navigation metadata validation — 2026-11-16

## Scope

Opt-in public JSON navigation index, basic finding fields and report-relative
URLs. No report base URL, full diagnostic dump, UI changes or new dependencies.
See [design](../superpowers/specs/2026-11-16-navigation-metadata-design.md) and
[public contract](../metadata.md).

## Evidence before final review

- RED: CLI test failed because RunOptions had no metadata field; after adding
  the option, real-output integration test failed because JSON did not exist.
- GREEN: Rust 1.100.0 full suite: 78 unit + 15 existing integration + 5 metadata
  integration = 98 tests, zero failures. Tests include actual HTML URLs/fragments,
  workspace/cached builds, Unicode paths, default absence, failed/clean inspection,
  stale/missing/no-span locations, deterministic ordering, invalid destinations,
  generated/reserved collisions, staging and final replacement failures.
- Windows-specific junction-parent rejection and case-alias collision tests pass.
  Unix symlink-file/parent tests are included for the CI platforms; not locally run.
- Rust 1.100.0 fmt and strict all-target Clippy pass.
- Rust 1.88.0 all-target locked compile check and process-pipe regression pass.
- Node suite: 49 tests pass; metadata adds no JavaScript or browser behavior.
- Real self-report generated in default online category mode using Rust 1.100.0,
  `--locked --all-targets -- -W clippy::pedantic`: 43 navigation pages, 73 findings,
  schema 1, collection complete, Clippy exit 0. JSON: 55,936 bytes. Every exported
  URL/fragment was validated against staged HTML before publication.
- Preview: `target/navigation-metadata-preview/index.html` and `metadata.json`.

Fresh final review, delivery commit and remote CI results are recorded below
after execution. Local tests are not claims of Linux/macOS or browser execution.
