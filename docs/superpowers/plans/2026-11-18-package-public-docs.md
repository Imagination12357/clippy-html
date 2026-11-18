# Source package and public documentation plan

**Goal:** Ship an explicit source-package inventory and current English user docs.

**Architecture:** Cargo `include` admits build inputs, bundled report assets and
licenses, README, the public metadata contract and the unit-test process helper.
Repository-only tests/fixtures, Node tooling, CI, audits and historical records
stay outside the archive. README serves users; CONTRIBUTING serves contributors.

**Spec:** User-approved include approach and English README, 2026-11-18.

## Constraints

- Preserve all embedded assets/notices; explicitly exclude the retired favicon CC
  license even though include patterns override gitignore/exclude defaults.
- No new product behavior/dependencies, publication, push or remote mutations.
- Keep current source installation instructions; do not imply a registry release.
- Preserve repository audit records and tests; do not delete excluded files.

## Task: package boundary and docs

- [x] Replace exclude with a rooted include list and inspect actual Cargo inventory.
- [x] Rewrite README in English with installation, options, navigation, metadata,
  offline semantics, failure/output handling, static hosting and license links.
- [x] Move repository development/CI instructions to CONTRIBUTING and reconcile
  current roadmap distribution status. Preserve historical records.
- [x] Build the actual archive; inspect its files and included document links.
- [x] Extract into target, compile/test retained unit tests, install into an
  isolated local prefix, then generate HTML/metadata with installed binary.
- [x] Read-only final review, verified local commits, no push.
