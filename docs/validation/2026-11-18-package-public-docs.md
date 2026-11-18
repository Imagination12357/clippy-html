# Source package and English public docs — 2026-11-18

## Changes

Replaced Cargo's short exclude list with a rooted include inventory for sources,
templates, assets, licenses/notices, README, the metadata contract and cargo_probe.
Explicit negative entries keep the locally preserved retired favicon CC license
and font Git attributes out of the archive; include overrides ignore rules.

No repository files are deleted. The archive omits audit records, agent guidance,
roadmap/history/validation files, CI, Node tooling, integration tests and the
category-maintenance example. Cargo warns about intentionally omitted auto-detected
repository targets and removes them from its normalized package manifest.
Nested Cargo fixture packages are excluded by Cargo itself, so the incomplete
integration suite is not distributed. In-source unit tests and their process
helper remain usable.

README is now English and describes current features, source installation, options,
both offline switches, navigation/keyboard behavior, metadata URL resolution,
failure/output contracts, static hosting and licenses. It no longer calls metadata
or suggestion diff/copy unimplemented. CONTRIBUTING holds checkout tests, package
policy, CI/Pages setup and reference constraints. No registry-release availability
or new browser validation is claimed. The repository field points to the existing
Git remote; no registry or remote mutation was performed.

## Verification

- `cargo +1.100.0 package --list --locked --offline --allow-dirty`: inspected the
  include inventory. Built/verified the actual `.crate` with Cargo, not just a list.
- Archive: 90 files, about 1.5 MiB compressed; all 12 WOFF2 files retained. No audit,
  Node, integration-test, history, target, retired CC or font Git-attributes files.
- All five local Markdown links in packaged README/metadata docs resolve inside
  the extracted archive. Contributor/CI references use the repository URL rather
  than a relative path to a file excluded from the package.
- Actual extracted package: 82 unit tests passed with Rust 1.100.0, including the
  helper compilation/pipe test. Cargo 1.88 all-target locked/offline check passed.
- Installed the extracted package in release mode into a unique `target/package-install-*`
  prefix. The user's existing install and persistent PATH were not changed.
- Using only a subprocess PATH prefix, `cargo clippy-html --version`, `--help`,
  and an offline no-dependency fixture report with metadata succeeded: four page
  entries, one clone_on_copy finding, source unchanged, fonts/notices/favicon
  present and retired favicon CC notice absent.
- One fresh-context read-only review found no actionable issues.
- These are Windows local results. The source code/UI is unchanged; repository
  integration and Node suites were not repeated for this package/docs-only change.
  No push, publish, hosted CI or deployment was performed.

The initial verification used `--allow-dirty` while editing. A final archive is
also built from the committed included files; delivery reports that result separately.
