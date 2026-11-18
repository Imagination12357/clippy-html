# Contributing

Development requires Rust with Clippy and rustfmt, Git, and Node.js 24 or newer.
The minimum supported Rust version is 1.88. The lockfiles pin Rust and Node test
dependencies. Install the 1.88.0 toolchain if you want to run the MSRV check.

```sh
rustup component add clippy rustfmt
npm ci --ignore-scripts
cargo test --locked
cargo fmt --check
cargo clippy --all-targets --locked -- -D warnings
cargo +1.88.0 check --all-targets --locked
npm test
```

The process-pipe test compiles the standard-library-only `examples/cargo_probe.rs`
into a temporary directory; no helper prebuild is required. Rust integration tests
copy fixture projects into temporary directories. Suggestion tests use Git to
apply patches and compare the resulting bytes. DOM tests use jsdom; they do not
prove browser layout, clipboard permissions, accessibility or local-file policies.

## Source packages

```sh
cargo package --list --locked
cargo package --locked
```

Cargo's `include` list defines the source distribution: product sources, Askama
templates, report assets and licenses, README, the public metadata contract, and
the process helper used by retained in-source unit tests. Cargo adds its manifest,
lockfile and packaging metadata. Historical records, audit scripts, CI, Node tools
and repository integration tests are not part of the archive. Nothing is deleted
from the checkout by these packaging rules.

Run the full test suite from a Git checkout. Cargo always excludes nested packages,
including this repository's Cargo-project fixtures, so shipping the integration
test files without those fixtures would be incomplete. The packaged source retains
its in-source unit tests and their helper. Verify the actual archive, not just the
working-tree file inventory, when changing package inputs.

Keep the retired favicon CC license excluded even though it is locally preserved:
an explicit `include` list overrides Git ignore rules. Preserve all current
embedded third-party notices and font licenses. Standalone executable releases
also need the accompanying notices described in [THIRD-PARTY-NOTICES](THIRD-PARTY-NOTICES.md).

## CI and the self-report

[The workflow](.github/workflows/ci.yml) runs stable Rust tests on Windows, Linux
and macOS; a Linux Rust 1.88 all-target compile check; fmt, strict Clippy, Node/DOM
tests and dogfood failure-handling checks.

PRs and non-main manual runs install the checkout and generate an online-category
self-report, preserving it and its logs as a `dogfood-report` artifact for 14 days.
On main, successful checks are followed by report generation and GitHub Pages
deployment in one job. Successful main runs use the deployment's `github-pages`
artifact; `dogfood-report` is retained on failure for troubleshooting. Installation,
Clippy or report-generation failures remain failures; Pages is not deployed from a
failed generation. Pedantic warnings are included in the self-report.

For a fork that should deploy its self-report, configure repository Settings →
Pages → Build and deployment → Source as **GitHub Actions**. Main manual runs can
deploy; other branches only produce the report artifact. The workflow and local
tests do not themselves change repository Pages settings.

To run a local self-report check, pass the script a fresh absolute working directory:

```sh
bash .github/scripts/dogfood.sh /absolute/path/to/a-fresh-directory
```

The script installs into that directory and retains the report and logs there.
Do not use an existing directory. It does not deploy anything.

## Structure and reference behavior

The pipeline is CLI → Cargo collection → normalization/deduplication → source
loading → HTML rendering → staged output publication. Optional navigation JSON is
built from the same page map and validated against actual HTML destinations.

This project ports rustdoc output, rather than designing a replacement style.
Inspect the pinned HTML/CSS/JavaScript before changing interface structure and
record necessary report-specific adaptations. Provenance lives under
`assets/vendor/rustdoc/`; the main CSS reference is 1.97.1 and the tabbed Settings
reference is 1.102.0-nightly. Keep basic navigation usable without JavaScript.

[ROADMAP](ROADMAP.md) tracks current remaining work. Historical implementation and
validation records stay under `docs/superpowers/` and `docs/validation/`; use current
code and public contracts when an older phase description differs.
