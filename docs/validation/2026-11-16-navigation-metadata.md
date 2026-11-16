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

## Final review and fix

One fresh-context read-only reviewer found one important issue and no minor
findings: the HTML publication lock was released before external JSON publication,
allowing successful concurrent writers to leave mixed snapshots. The new
`publication_lock_covers_external_metadata_and_is_released_with_the_receipt` test
failed on the old behavior (a second publisher entered between the two outputs).
PublishedReport now retains the existing RAII lock until its receipt is dropped.
The test passes and confirms the lock is subsequently released.

After the fix, Rust 1.100 full suite: 79 unit + 15 existing integration + 5 metadata
integration = 99/99. Strict Clippy and fmt pass. Rust 1.88 all-target check and all
79 unit tests pass. The real preview was regenerated after the fix with the same
43 pages/73 findings and all URL destinations validated. Node's 49 passing tests
were not repeated for a Rust-only lock-lifetime change.

Power-loss atomicity across HTML and external JSON remains deliberately excluded:
interruption can leave different snapshots. The writer lock prevents successful
cooperating writers from interleaving; it does not make two filesystem publications
one transaction. This boundary is documented in the public contract.

Local results are not claims of Linux/macOS or browser execution. Remote CI runs
after the separately authorized delivery push; its result is not inferred here.
